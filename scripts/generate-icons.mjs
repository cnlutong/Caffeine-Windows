import { execFileSync } from "node:child_process";
import {
  copyFileSync,
  mkdtempSync,
  readFileSync,
  readdirSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const cli = join(root, "node_modules/@tauri-apps/cli/tauri.js");
const icons = join(root, "src-tauri/icons");
const stage = mkdtempSync(join(tmpdir(), "caffeine-icons-"));
try {
  execFileSync(
    process.execPath,
    [cli, "icon", join(icons, "app.svg"), "--output", stage],
    { stdio: "pipe", cwd: root },
  );
  for (const file of [
    "32x32.png",
    "128x128.png",
    "128x128@2x.png",
    "icon.ico",
    "icon.icns",
  ]) {
    copyFileSync(join(stage, file), join(icons, file));
  }
  for (const mac of [false, true]) {
    for (const mode of ["off", "system", "display"]) {
      const color = mac
        ? "#000000"
        : { off: "#748071", system: "#d78c27", display: "#218963" }[mode];
      const active = mode !== "off";
      const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="64" height="64" viewBox="0 0 64 64"><g stroke="${color}" stroke-width="3.5" fill="none" stroke-linejoin="round" stroke-linecap="round"><path fill="${active ? color : "none"}" d="M10 24h35l-2 19c-1 9-8 14-16 14S12 52 12 43Z"/><path d="M45 28h4c13 0 13 18-5 18M11 62h33"/>${active ? '<path d="M21 17c-5-6 6-10 1-16M34 17c-5-6 6-10 1-16"/>' : ""}${mode === "display" ? '<circle cx="54" cy="12" r="6"/>' : ""}</g></svg>`;
      const input = join(stage, "tray.svg");
      writeFileSync(input, svg);
      execFileSync(
        process.execPath,
        [cli, "icon", input, "--output", join(stage, "tray"), "--png", "44"],
        { stdio: "pipe", cwd: root },
      );
      copyFileSync(
        join(stage, "tray/44x44.png"),
        join(icons, `tray-${mac ? "mac-" : ""}${mode}.png`),
      );
    }
  }
  // Fail early on a malformed source rather than committing empty image assets.
  if (!readFileSync(join(icons, "app.svg"), "utf8").includes("<svg"))
    throw new Error("Missing vector source");
  console.log(
    `Generated ${readdirSync(icons).filter((file) => /\.(png|ico|icns)$/.test(file)).length} desktop icons.`,
  );
} finally {
  rmSync(stage, { recursive: true, force: true });
}
