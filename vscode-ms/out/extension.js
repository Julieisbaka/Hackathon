"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = require("vscode");
const child_process_1 = require("child_process");
const lexer_1 = require("./lexer");
const builtins_1 = require("./builtins");
function activate(context) {
    const runCmd = vscode.commands.registerCommand('ms.runFile', async () => {
        const editor = vscode.window.activeTextEditor;
        if (!editor) {
            return;
        }
        const document = editor.document;
        if (document.isUntitled) {
            await document.save();
        }
        const filePath = document.fileName;
        const exeConfig = vscode.workspace.getConfiguration('ms');
        const exePath = exeConfig.get('runtimePath');
        if (!exePath) {
            vscode.window.showErrorMessage('Set ms.runtimePath to your compiled Rust interpreter executable.');
            return;
        }
        const output = vscode.window.createOutputChannel('ms');
        output.clear();
        output.show(true);
        const child = (0, child_process_1.spawn)(exePath, [filePath], { shell: true });
        child.stdout.on('data', (data) => output.append(data.toString()));
        child.stderr.on('data', (data) => output.append(data.toString()));
        child.on('close', (code) => output.appendLine(`\nProcess exited with code ${code}`));
    });
    // Register a basic semantic token provider using our JS lexer
    const legend = new vscode.SemanticTokensLegend([
        'keyword', 'string', 'number', 'operator', 'function', 'variable', 'comment'
    ], []);
    const provider = {
        provideDocumentSemanticTokens(doc) {
            const text = doc.getText();
            const tokens = (0, lexer_1.tokenize)(text);
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
    context.subscriptions.push(vscode.languages.registerDocumentSemanticTokensProvider({ language: 'ms' }, provider, legend));
    // Completion provider for built-ins
    context.subscriptions.push(vscode.languages.registerCompletionItemProvider({ language: 'ms' }, {
        provideCompletionItems() {
            const items = [];
            for (const b of builtins_1.BUILTINS) {
                const item = new vscode.CompletionItem(b.name, b.kind === 'function' ? vscode.CompletionItemKind.Function : vscode.CompletionItemKind.Constant);
                item.detail = b.detail;
                item.documentation = new vscode.MarkdownString(b.documentation);
                items.push(item);
            }
            for (const k of builtins_1.KEYWORDS) {
                items.push(new vscode.CompletionItem(k, vscode.CompletionItemKind.Keyword));
            }
            return items;
        }
    }));
    // Hover provider with docs
    context.subscriptions.push(vscode.languages.registerHoverProvider({ language: 'ms' }, {
        provideHover(doc, pos) {
            const range = doc.getWordRangeAtPosition(pos, /[A-Za-z_][A-Za-z0-9_]*/);
            if (!range)
                return undefined;
            const word = doc.getText(range);
            const b = builtins_1.BUILTINS.find(x => x.name === word);
            const md = new vscode.MarkdownString();
            if (b) {
                md.appendCodeblock(`${b.name}${b.parameters ? '(' + b.parameters.join(', ') + ')' : ''}${b.returnType ? ': ' + b.returnType : ''}`, 'ms');
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
                    let docLines = [];
                    let ll = line;
                    let seen = 0;
                    while (ll >= 0) {
                        const t = doc.lineAt(ll).text;
                        docLines.unshift(t);
                        if (t.includes('"""')) {
                            seen++;
                            if (seen >= 2)
                                break;
                        }
                        ll--;
                    }
                    md.appendMarkdown('\n');
                    md.appendCodeblock(docLines.join('\n'), 'ms');
                    foundDoc = true;
                    break;
                }
                if (text.trim() === '') {
                    line--;
                    continue;
                }
                break;
            }
            if (!b && !foundDoc)
                return undefined;
            return new vscode.Hover(md, range);
        }
    }));
    // Signature help (basic, based on built-ins only)
    context.subscriptions.push(vscode.languages.registerSignatureHelpProvider({ language: 'ms' }, {
        provideSignatureHelp(doc, pos) {
            const linePrefix = doc.lineAt(pos.line).text.slice(0, pos.character);
            const m = /([A-Za-z_][A-Za-z0-9_]*)\s*\($/.exec(linePrefix);
            if (!m)
                return null;
            const name = m[1];
            const b = builtins_1.BUILTINS.find(x => x.name === name && x.kind === 'function');
            if (!b)
                return null;
            const sig = new vscode.SignatureInformation(`${b.name}(${(b.parameters || []).join(', ')})${b.returnType ? ': ' + b.returnType : ''}`, new vscode.MarkdownString(b.documentation));
            sig.parameters = (b.parameters || []).map(p => new vscode.ParameterInformation(p));
            const help = new vscode.SignatureHelp();
            help.signatures = [sig];
            help.activeSignature = 0;
            help.activeParameter = 0;
            return help;
        }
    }, '('));
}
function deactivate() { }
function tokenTypeIndex(type, legend) {
    const idx = legend.tokenTypes.indexOf(type);
    return idx >= 0 ? idx : 0;
}
//# sourceMappingURL=extension.js.map