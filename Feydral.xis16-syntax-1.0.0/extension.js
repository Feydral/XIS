const vscode = require("vscode");

const TOKEN_RULES = [
  {
    scope: "keyword.control.xis16",
    settings: { foreground: "#6edfce", fontStyle: "" }
  },
  {
    scope: "variable.language.xis16",
    settings: { foreground: "#a890ee", fontStyle: "" }
  },
  {
    scope: "constant.numeric.hex.xis16",
    settings: { foreground: "#f78056", fontStyle: "" }
  },
  {
    scope: "constant.numeric.binary.xis16",
    settings: { foreground: "#f78056", fontStyle: "" }
  },
  {
    scope: "constant.numeric.decimal.xis16",
    settings: { foreground: "#f78056", fontStyle: "" }
  },
  {
    scope: "comment.line.double-slash.xis16",
    settings: { foreground: "#686a6c", fontStyle: "" }
  },
  {
    scope: "entity.name.label.xis16",
    settings: { foreground: "#ffdd88", fontStyle: "italic" }
  },
  {
    scope: "keyword.other.flag.xis16",
    settings: { foreground: "#82aaff", fontStyle: "" }
  },
  {
    scope: "keyword.other.define.xis16",
    settings: { foreground: "#82aaff", fontStyle: "" }
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
