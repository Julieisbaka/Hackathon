
# Math Script (MS)

> A modern, math-focused scripting language and VS Code extension for mathematical computation, visualization, and learning.

---

## Features

- **Custom Math Language**: Write scripts in `.ms` files with intuitive math syntax (see [Syntax.md](./Syntax.md)).
- **VS Code Extension**: Syntax highlighting, code snippets, and one-click run support for `.ms` files.
- **Powerful Interpreter**: Supports variables, functions, conditionals, arrays, matrices, calculus, and more.
- **Easy Setup**: Cross-platform interpreter and seamless VS Code integration.

---

## Quick Start

1. **Install the VS Code Extension**

- Search for `Math Script` in the VS Code Extensions Marketplace and install it.

2. **Download the MS Interpreter**

- Use the extension's command palette (`MS: Download Interpreter`) or download the latest release from [GitHub Releases](https://github.com/Julieisbaka/Hackathon/releases).

3. **Configure the Interpreter Path**

- The extension will auto-detect the interpreter if possible. To set manually, go to VS Code settings and set `ms.runtimePath` to the path of your downloaded interpreter executable.

4. **Create and Run Math Script Files**

- Create a new file with the `.ms` extension and start writing math scripts!
- Use the command `MS: Run Current File` to execute your script.

---

## Example

```MS
f(x) = 4x + 2 {x > 2}
print(f(3))
```

---

## Building from Source

### 1. Clone the repository

```sh
git clone https://github.com/Julieisbaka/Hackathon.git
cd Hackathon
```

### 2. Build the Interpreter

```sh
cd syntax_interpreter
cargo build --release
```

The binary will be in `syntax_interpreter/target/release/`.

### 3. Build the VS Code Extension

```sh
cd ../vscode-ms
npm install
npm run compile
npx vsce package
```

This will generate a `.vsix` file in the `vscode-ms` directory for manual installation.

---

## Documentation

- [Syntax Reference](./Syntax.md): Full language syntax and features
- [VS Code Extension Usage](./vscode-ms/README.md): Extension features and settings

---

## Contributing

Contributions are welcome! Please open issues or pull requests on [GitHub](https://github.com/Julieisbaka/Hackathon).

---

## License

This project is licensed under the MIT License. See [LICENSE.md](./vscode-ms/LICENSE.md) for details.
