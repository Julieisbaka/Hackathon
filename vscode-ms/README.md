# MS Language VS Code Extension

Features:

- Syntax highlighting via TextMate grammar and a lightweight lexer for semantic tokens
- Run current `.ms` file using the MS interpreter

Setup:

1. Download the MS interpreter executable.
2. In VS Code, set `ms.runtimePath` to that executable path. Or leave it empty and keep `ms.autoDetectRuntime: true` to let the extension find it automatically in common build paths or on PATH.
3. Install this extension

Settings:

- `ms.runtimePath`: Explicit path to interpreter exe.
- `ms.autoDetectRuntime`: Try common build dirs and PATH for exe.
- `ms.searchPaths`: Extra folders to search for the exe.
- `ms.args`: Extra args passed to the interpreter.
- `ms.runOnSave`: Automatically run `.ms` file on save.
- `ms.showProcessExit`: Append process exit code to output.
- `ms.enableSemanticTokens`: Toggle semantic tokens.
- `ms.enableCompletions`: Toggle completions.
- `ms.enableHoverDocstrings`: Toggle hover docstrings.
- `ms.enableSignatureHelp`: Toggle signature help.

Notes:

- Trig functions expect radians.
- Output appears only via `print` or `log` in your `.ms` code.
