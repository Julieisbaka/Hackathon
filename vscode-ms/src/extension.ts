import * as vscode from 'vscode';
import { spawn } from 'child_process';
import { tokenize } from './lexer';
import { BUILTINS, KEYWORDS } from './builtins';
import fetch from 'node-fetch';
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
  const releases = await fetch(api).then((r: any) => r.json());
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
  const res = await fetch(assetUrl);
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
        candidates.push(require('path').join(dir, exe));
      }
    }
    const fs = await import('fs');
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

  // Signature help (basic, based on built-ins only)
  if (vscode.workspace.getConfiguration('ms').get<boolean>('enableSignatureHelp', true)) {
    context.subscriptions.push(vscode.languages.registerSignatureHelpProvider({ language: 'ms' }, {
  provideSignatureHelp(doc: vscode.TextDocument, pos: vscode.Position): vscode.SignatureHelp | null {
      const linePrefix = doc.lineAt(pos.line).text.slice(0, pos.character);
      const m = /([A-Za-z_][A-Za-z0-9_]*)\s*\($/.exec(linePrefix);
      if (!m) return null;
      const name = m[1];
      const b = BUILTINS.find(x => x.name === name && x.kind === 'function');
      if (!b) return null;
      const sig = new vscode.SignatureInformation(`${b.name}(${(b.parameters||[]).join(', ')})${b.returnType ? ': '+b.returnType : ''}`, new vscode.MarkdownString(b.documentation));
      sig.parameters = (b.parameters||[]).map(p => new vscode.ParameterInformation(p));
      const help = new vscode.SignatureHelp();
      help.signatures = [sig];
      help.activeSignature = 0;
      help.activeParameter = 0;
      return help;
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
