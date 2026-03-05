const vscode = require("vscode");

const TOKEN_RULES = [
  {
    scope: "keyword.control.xis16",
    settings: { foreground: "#6edfce", fontStyle: "" }
  },
  {
    scope: "variable.language.xis16",
    settings: { foreground: "#9094e8", fontStyle: "" }
  },
  {
    scope: "constant.numeric.hex.xis16",
    settings: { foreground: "#dc6e6e", fontStyle: "" }
  },
  {
    scope: "constant.numeric.binary.xis16",
    settings: { foreground: "#dc6e6e", fontStyle: "" }
  },
  {
    scope: "constant.numeric.decimal.xis16",
    settings: { foreground: "#dc6e6e", fontStyle: "" }
  },
  {
    scope: "comment.line.double-slash.xis16",
    settings: { foreground: "#686a6c", fontStyle: "" }
  },
  {
    scope: "entity.name.function.xis16",
    settings: { foreground: "#e9c685", fontStyle: "bold" }
  },
  {
    scope: "keyword.other.flag.xis16",
    settings: { foreground: "#da7dc9", fontStyle: "" }
  }
];

async function activate(context) {
  const config = vscode.workspace.getConfiguration("editor");
  const current = config.inspect("tokenColorCustomizations");
  const existing = (current?.globalValue) ?? {};
  const existingRules = existing.textMateRules ?? [];

  const filtered = existingRules.filter(
    (r) => !String(r.scope).includes("xis16")
  );
  const merged = { ...existing, textMateRules: [...filtered, ...TOKEN_RULES] };

  await config.update(
    "tokenColorCustomizations",
    merged,
    vscode.ConfigurationTarget.Global
  );
}

function deactivate() {}

module.exports = { activate, deactivate };
