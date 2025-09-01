  // Diagnostics provider for real syntax errors
  const diagnostics = vscode.languages.createDiagnosticCollection('ms');
  context.subscriptions.push(diagnostics);
  vscode.workspace.onDidOpenTextDocument(checkMsDiagnostics);
  vscode.workspace.onDidSaveTextDocument(checkMsDiagnostics);
  vscode.workspace.onDidCloseTextDocument(doc => diagnostics.delete(doc.uri));

  async function checkMsDiagnostics(doc: vscode.TextDocument) {
    if (doc.languageId !== 'ms') return;
    const exePath = await resolveRuntime();
    const diags: vscode.Diagnostic[] = [];

    // 1. Interpreter errors (as before)
    if (exePath) {
      const tmp = os.tmpdir();
      const tmpFile = path.join(tmp, `ms_diag_${Date.now()}_${Math.random().toString(36).slice(2)}.ms`);
      fs.writeFileSync(tmpFile, doc.getText());
      const result = spawnSync(exePath, [tmpFile], { encoding: 'utf8' });
      fs.unlinkSync(tmpFile);
      let output = (result.stderr || '').trim();
      if (!output) output = (result.stdout || '').trim();
      // Look for lines like 'ERROR: invalid syntax at line X' or similar
      const regex = /ERROR:.*at line (\d+)/gi;
      let match;
      while ((match = regex.exec(output))) {
        const line = parseInt(match[1], 10) - 1;
        const range = doc.lineAt(line).range;
        diags.push(new vscode.Diagnostic(range, output, vscode.DiagnosticSeverity.Error));
      }
    }

    // 2. Static analysis for unused/undefined/duplicate/type errors
    const text = doc.getText();
    const lines = text.split(/\r?\n/);
    const varDefs = new Map(); // name -> {line, type}
    const varUses = new Map(); // name -> [line,...]
    const funcDefs = new Map(); // name -> {line, params, paramTypes}
    const funcCalls = [];
    const varRegex = /^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=([^=].*)$/;
    const funcRegex = /^\s*([A-Za-z_][A-Za-z0-9_]*)\s*\(([^)]*)\)\s*=/;
    const callRegex = /([A-Za-z_][A-Za-z0-9_]*)\s*\(([^)]*)\)/g;
    // Pass 1: find definitions
    for (let i = 0; i < lines.length; ++i) {
      const line = lines[i];
      let m;
      if ((m = funcRegex.exec(line))) {
        const name = m[1];
        const params = m[2].split(',').map(s => s.trim()).filter(Boolean);
        if (funcDefs.has(name)) {
          diags.push(new vscode.Diagnostic(new vscode.Range(i, 0, i, line.length), `Duplicate function definition: ${name}`, vscode.DiagnosticSeverity.Warning));
        }
        funcDefs.set(name, { line: i, params, paramTypes: params.map(_ => 'unknown') });
        continue;
      }
      if ((m = varRegex.exec(line))) {
        const name = m[1];
        if (varDefs.has(name)) {
          diags.push(new vscode.Diagnostic(new vscode.Range(i, 0, i, line.length), `Duplicate variable definition: ${name}`, vscode.DiagnosticSeverity.Warning));
        }
        varDefs.set(name, { line: i, type: 'unknown' });
      }
    }
    // Pass 2: find usages
    for (let i = 0; i < lines.length; ++i) {
      const line = lines[i];
      let m;
      while ((m = callRegex.exec(line))) {
        const name = m[1];
        const args = m[2].split(',').map(s => s.trim()).filter(Boolean);
        funcCalls.push({ name, args, line: i });
        // Mark function as used
        if (funcDefs.has(name)) {
          funcDefs.get(name).used = true;
        }
      }
      // Find variable usages (simple heuristic: any word that matches a var name)
      for (const v of varDefs.keys()) {
        const idx = line.indexOf(v);
        if (idx !== -1 && !/^\s*#/.test(line)) {
          if (!varUses.has(v)) varUses.set(v, []);
          varUses.get(v).push(i);
        }
      }
    }
    // Pass 3: diagnostics
    // Unused variables
    for (const [v, def] of varDefs.entries()) {
      const uses = varUses.get(v) || [];
      if (uses.length <= 1) { // only defined, never used elsewhere
        diags.push(new vscode.Diagnostic(new vscode.Range(def.line, 0, def.line, lines[def.line].length), `Variable '${v}' is defined but never used`, vscode.DiagnosticSeverity.Warning));
      }
    }
    // Undefined variables (used but not defined)
    for (const [v, uses] of varUses.entries()) {
      if (!varDefs.has(v)) {
        for (const line of uses) {
          diags.push(new vscode.Diagnostic(new vscode.Range(line, 0, line, lines[line].length), `Variable '${v}' is used but not defined`, vscode.DiagnosticSeverity.Warning));
        }
      }
    }
    // Type mismatches for built-in functions (if possible)
    for (const call of funcCalls) {
      const builtin = BUILTINS.find(b => b.name === call.name && b.kind === 'function');
      if (builtin && builtin.parameters) {
        if (call.args.length < builtin.parameters.length) {
          diags.push(new vscode.Diagnostic(new vscode.Range(call.line, 0, call.line, lines[call.line].length), `Function '${call.name}' expects at least ${builtin.parameters.length} arguments`, vscode.DiagnosticSeverity.Warning));
        }
      }
      // User-defined function: check arity
      if (funcDefs.has(call.name)) {
        const def = funcDefs.get(call.name);
        if (call.args.length !== def.params.length) {
          diags.push(new vscode.Diagnostic(new vscode.Range(call.line, 0, call.line, lines[call.line].length), `Function '${call.name}' expects ${def.params.length} arguments, got ${call.args.length}`, vscode.DiagnosticSeverity.Warning));
        }
      }
    }
    diagnostics.set(doc.uri, diags);
  }
import * as vscode from 'vscode';
import { spawn } from 'child_process';
import { tokenize } from './lexer';
import { BUILTINS, KEYWORDS } from './builtins';
import * as path from 'path';
import * as fs from 'fs';
import * as os from 'os';
import { spawnSync } from 'child_process';
import { Buffer } from 'buffer';

export function activate(context: vscode.ExtensionContext) {
  // Add a CodeLens at the top of .ms files to run the file
  context.subscriptions.push(vscode.languages.registerCodeLensProvider({ language: 'ms' }, {
    provideCodeLenses(document: vscode.TextDocument) {
      if (document.lineCount === 0) return [];
      return [
        new vscode.CodeLens(new vscode.Range(0, 0, 0, 0), {
          title: '▶ Run File',
          command: 'ms.runFile',
          arguments: [],
        })
      ];
    }
  }));
  // Inline result CodeLens provider for .ms files
  context.subscriptions.push(vscode.languages.registerCodeLensProvider({ language: 'ms' }, {
    async provideCodeLenses(document: vscode.TextDocument) {
      const lenses: vscode.CodeLens[] = [];
      for (let i = 0; i < document.lineCount; ++i) {
        const line = document.lineAt(i);
        const text = line.text.trim();
        // Only show for non-empty, non-comment, non-assignment lines
        if (!text || text.startsWith('#') || text.includes('=') || text.startsWith('function')) continue;
        // Try to evaluate using the interpreter
        const exePath = await resolveRuntime();
        if (!exePath) continue;
        try {
          const tmp = os.tmpdir();
          const tmpFile = path.join(tmp, `ms_inline_${Date.now()}_${Math.random().toString(36).slice(2)}.ms`);
          fs.writeFileSync(tmpFile, text);
          const result = spawnSync(exePath, [tmpFile], { encoding: 'utf8' });
          fs.unlinkSync(tmpFile);
          let output = (result.stdout || '').trim();
          if (!output) output = (result.stderr || '').trim();
          if (output.length > 80) output = output.slice(0, 80) + '...';
          if (output) {
            lenses.push(new vscode.CodeLens(line.range, { title: `= ${output}`, command: '', arguments: [] }));
          }
        } catch {}
      }
      return lenses;
    }
  }));
  // Download Interpreter command
  const downloadCmd = vscode.commands.registerCommand('ms.downloadInterpreter', async () => {
    const owner = 'Julieisbaka';
    const repo = 'Hackathon';
    const assetName = 'ms.exe';
    const api = `https://api.github.com/repos/${owner}/${repo}/releases`;
  const releases = await (globalThis as any).fetch(api).then((r: any) => r.json());
    if (!Array.isArray(releases) || releases.length === 0) {
      vscode.window.showErrorMessage('No releases found on GitHub.');
      return;
    }
    // Find the latest release with ms.exe
    let assetUrl = '';
    for (const rel of releases) {
      const asset = (rel.assets||[]).find((a:any) => a.name === assetName);
      if (asset) { assetUrl = asset.browser_download_url; break; }
    }
    if (!assetUrl) {
      vscode.window.showErrorMessage('No ms.exe asset found in the latest releases.');
      return;
    }
    // Ask user where to save
    const ws = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
  const defaultPath = ws ? path.join(ws, assetName) : undefined;
    const uri = await vscode.window.showSaveDialog({
      defaultUri: defaultPath ? vscode.Uri.file(defaultPath) : undefined,
      saveLabel: 'Save ms.exe',
      filters: { 'Executable': ['exe'] }
    });
    if (!uri) return;
    // Download
  const res = await (globalThis as any).fetch(assetUrl);
    if (!res.ok) {
      vscode.window.showErrorMessage('Failed to download ms.exe: ' + res.statusText);
      return;
    }
  const buf = await res.arrayBuffer();
  fs.writeFileSync(uri.fsPath, Buffer.from(buf));
    vscode.window.showInformationMessage('ms.exe downloaded to ' + uri.fsPath);
    // Optionally update runtimePath
    const cfg = vscode.workspace.getConfiguration('ms');
    await cfg.update('runtimePath', uri.fsPath, vscode.ConfigurationTarget.Workspace);
  });

  // Status bar button
  const status = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
  status.text = '$(cloud-download) Download MS Interpreter';
  status.command = 'ms.downloadInterpreter';
  status.tooltip = 'Download the latest ms.exe from GitHub releases';
  status.show();
  context.subscriptions.push(status);
  context.subscriptions.push(downloadCmd);
  const resolveRuntime = async (): Promise<string | undefined> => {
    const cfg = vscode.workspace.getConfiguration('ms');
    const explicit = cfg.get<string>('runtimePath');
    if (explicit && explicit.trim().length > 0) return explicit;
    const auto = cfg.get<boolean>('autoDetectRuntime', true);
    if (!auto) return undefined;
    const searchPaths = cfg.get<string[]>('searchPaths', []);
    const wsFolders = vscode.workspace.workspaceFolders || [];
    const candidates: string[] = [];
    const exeNames = process.platform === 'win32' ? ['syntax_interpreter.exe', 'ms.exe'] : ['syntax_interpreter', 'ms'];
    for (const ws of wsFolders) {
      for (const p of searchPaths) {
        const expanded = p.replace('${workspaceFolder}', ws.uri.fsPath);
        for (const exe of exeNames) {
          candidates.push(vscode.Uri.joinPath(vscode.Uri.file(expanded), exe).fsPath);
        }
      }
    }
    // Also search PATH
    const pathEnv = (process.env.PATH || '').split(process.platform === 'win32' ? ';' : ':');
    for (const dir of pathEnv) {
      for (const exe of exeNames) {
        candidates.push(path.join(dir, exe));
      }
    }
    for (const c of candidates) {
      try { if (fs.existsSync(c)) return c; } catch {}
    }
    return undefined;
  };

  const runCmd = vscode.commands.registerCommand('ms.runFile', async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor) { return; }

    const document = editor.document;
    if (document.isUntitled) {
      await document.save();
    }

    const filePath = document.fileName;

  const cfg = vscode.workspace.getConfiguration('ms');
  const exePath = await resolveRuntime();

    if (!exePath) {
      vscode.window.showErrorMessage('Set ms.runtimePath to the ms interpreter executable.');
      return;
    }

    const output = vscode.window.createOutputChannel('ms');
    output.clear();
    output.show(true);

    const extraArgs = cfg.get<string[]>('args', []);
    const child = spawn(exePath, [...(extraArgs||[]), filePath], { shell: true });

    function styleLog(line: string): string {
      if (/^ERROR:/i.test(line)) return `\x1b[31m${line}\x1b[0m`; // red
      if (/^WARN:/i.test(line)) return `\x1b[33m${line}\x1b[0m`; // yellow
      if (/^INFO:/i.test(line)) return `\x1b[36m${line}\x1b[0m`; // cyan
      if (/^DEBUG:/i.test(line)) return `\x1b[90m${line}\x1b[0m`; // gray
      return line;
    }

    function appendStyled(data: Buffer|string) {
      const lines = data.toString().split(/\r?\n/);
      for (const line of lines) {
        if (line.trim().length === 0) continue;
        output.appendLine(styleLog(line));
      }
    }

    child.stdout.on('data', appendStyled);
    child.stderr.on('data', appendStyled);
    child.on('close', (code: number) => {
      if (cfg.get<boolean>('showProcessExit', true)) {
        output.appendLine(`\nProcess exited with code ${code}`);
      }
    });
  });

  // Register a basic semantic token provider using our JS lexer
  const legend = new vscode.SemanticTokensLegend([
    'keyword','string','number','operator','function','variable','comment'
  ], []);

  const provider: vscode.DocumentSemanticTokensProvider = {
  provideDocumentSemanticTokens(doc: vscode.TextDocument) {
      const text = doc.getText();
      const tokens = tokenize(text);
      const builder = new vscode.SemanticTokensBuilder(legend);
      for (const t of tokens) {
        const start = doc.positionAt(t.start);
        const length = t.end - t.start;
        builder.push(start.line, start.character, length, tokenTypeIndex(t.type, legend));
      }
      return builder.build();
    }
  };

  context.subscriptions.push(runCmd);
  if (vscode.workspace.getConfiguration('ms').get<boolean>('enableSemanticTokens', true)) {
    context.subscriptions.push(vscode.languages.registerDocumentSemanticTokensProvider({ language: 'ms' }, provider, legend));
  }

  // Completion provider for built-ins
  if (vscode.workspace.getConfiguration('ms').get<boolean>('enableCompletions', true)) {
    context.subscriptions.push(vscode.languages.registerCompletionItemProvider({ language: 'ms' }, {
  provideCompletionItems(): vscode.CompletionItem[] {
      const items: vscode.CompletionItem[] = [];
      for (const b of BUILTINS) {
        const item = new vscode.CompletionItem(b.name, b.kind === 'function' ? vscode.CompletionItemKind.Function : vscode.CompletionItemKind.Constant);
        item.detail = b.detail;
        item.documentation = new vscode.MarkdownString(b.documentation);
        items.push(item);
      }
      for (const k of KEYWORDS) {
        items.push(new vscode.CompletionItem(k, vscode.CompletionItemKind.Keyword));
      }
      return items;
    }
    }));
  }

  // Hover provider with docs
  if (vscode.workspace.getConfiguration('ms').get<boolean>('enableHoverDocstrings', true)) {
    context.subscriptions.push(vscode.languages.registerHoverProvider({ language: 'ms' }, {
  provideHover(doc: vscode.TextDocument, pos: vscode.Position): vscode.Hover | undefined {
      const range = doc.getWordRangeAtPosition(pos, /[A-Za-z_][A-Za-z0-9_]*/);
      if (!range) return undefined;
      const word = doc.getText(range);
      const b = BUILTINS.find(x => x.name === word);
      const md = new vscode.MarkdownString();
      if (b) {
        md.appendCodeblock(`${b.name}${b.parameters ? '('+b.parameters.join(', ')+')' : ''}${b.returnType ? ': '+b.returnType : ''}`, 'ms');
        md.appendMarkdown('\n');
        md.appendMarkdown(b.documentation);
      }
      // Scan previous lines for a docstring or comments
      let line = range.start.line - 1;
      let foundDoc = false;
      while (line >= 0 && (range.start.line - line) <= 20) {
        const text = doc.lineAt(line).text.trimEnd();
        if (text.trim().startsWith('#')) {
          md.appendMarkdown(`\n${text.trim()}`);
          foundDoc = true;
          line--;
          continue;
        }
        if (text.includes('"""')) {
          // naive: collect until previous """
          let docLines: string[] = [];
          let ll = line;
          let seen = 0;
          while (ll >= 0) {
            const t = doc.lineAt(ll).text;
            docLines.unshift(t);
            if (t.includes('"""')) { seen++; if (seen >= 2) break; }
            ll--;
          }
          md.appendMarkdown('\n');
          md.appendCodeblock(docLines.join('\n'), 'ms');
          foundDoc = true;
          break;
        }
        if (text.trim() === '') { line--; continue; }
        break;
      }
      if (!b && !foundDoc) return undefined;
      return new vscode.Hover(md, range);
    }
    }));
  }

  // Signature help for built-ins and user-defined functions
  if (vscode.workspace.getConfiguration('ms').get<boolean>('enableSignatureHelp', true)) {
    context.subscriptions.push(vscode.languages.registerSignatureHelpProvider({ language: 'ms' }, {
      provideSignatureHelp(doc: vscode.TextDocument, pos: vscode.Position): vscode.SignatureHelp | null {
        const linePrefix = doc.lineAt(pos.line).text.slice(0, pos.character);
        const m = /([A-Za-z_][A-Za-z0-9_]*)\s*\($/.exec(linePrefix);
        if (!m) return null;
        const name = m[1];
        // 1. Check built-ins
        const b = BUILTINS.find(x => x.name === name && x.kind === 'function');
        if (b) {
          const sig = new vscode.SignatureInformation(`${b.name}(${(b.parameters||[]).join(', ')})${b.returnType ? ': '+b.returnType : ''}`, new vscode.MarkdownString(b.documentation));
          sig.parameters = (b.parameters||[]).map(p => new vscode.ParameterInformation(p));
          const help = new vscode.SignatureHelp();
          help.signatures = [sig];
          help.activeSignature = 0;
          help.activeParameter = 0;
          return help;
        }
        // 2. Check user-defined functions in the document
        // Match lines like: fname(x, y) = ...
        const funcRegex = /^([A-Za-z_][A-Za-z0-9_]*)\s*\(([^)]*)\)\s*=/gm;
        let match: RegExpExecArray | null;
        while ((match = funcRegex.exec(doc.getText())) !== null) {
          if (match[1] === name) {
            const params = match[2].split(',').map(s => s.trim()).filter(Boolean);
            const sig = new vscode.SignatureInformation(`${name}(${params.join(', ')})`, new vscode.MarkdownString('User-defined function'));
            sig.parameters = params.map(p => new vscode.ParameterInformation(p));
            const help = new vscode.SignatureHelp();
            help.signatures = [sig];
            help.activeSignature = 0;
            help.activeParameter = 0;
            return help;
          }
        }
        return null;
      }
    }, '('));
  }

  // Optional: run on save
  if (vscode.workspace.getConfiguration('ms').get<boolean>('runOnSave', false)) {
  context.subscriptions.push(vscode.workspace.onDidSaveTextDocument((doc: vscode.TextDocument) => {
      if (doc.languageId === 'ms') {
        vscode.commands.executeCommand('ms.runFile');
      }
    }));
  }
}

export function deactivate() {}

function tokenTypeIndex(type: string, legend: vscode.SemanticTokensLegend): number {
  const idx = legend.tokenTypes.indexOf(type);
  return idx >= 0 ? idx : 0;
}
