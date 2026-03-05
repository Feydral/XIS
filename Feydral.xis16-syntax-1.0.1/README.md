# Xis16 Assembly Syntax Extension
**Author:** Feydral

## Installation
1. Copy the `Feydral.xis16-syntax-1.0.1/` folder into your VSCode extensions directory:
   - **Windows:** `C:\Users\<n>\.vscode\extensions\`
   - **Mac/Linux:** `~/.vscode/extensions/`
2. Restart VSCode
3. `.xis16` files will be automatically detected and highlighted

## Supported Syntax Elements

| Element       | Example                |
|---------------|------------------------|
| Mnemonics     | `ADD`, `SUB`, `JMP`    |
| Registers     | `r0` – `r7`            |
| Numbers (hex) | `0xFF`                 |
| Numbers (bin) | `0b1010`               |
| Numbers (dec) | `42`                   |
| Labels        | `.main`, `.loop`       |
| Comments      | `// this is a comment` |
