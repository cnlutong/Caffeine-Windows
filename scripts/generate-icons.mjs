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
  if (!process.argv.includes("--tray-only")) {
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
  }
  const source = readFileSync(join(icons, "tray.svg"), "utf8");
  for (const mac of [false, true]) {
    for (const mode of ["off", "system", "display"]) {
      const color = mac
        ? "#000000"
        : { off: "#77828e", system: "#a26813", display: "#2c8c69" }[mode];
      let svg = source.replaceAll("currentColor", color);
      if (mode !== "system")
        svg = svg.replace(/<g id="steam">[\s\S]*?<\/g>/, "");
      if (mode !== "off")
        svg = svg.replace('<path id="cup"', `<path id="cup" fill="${color}"`);
      if (mode !== "display")
        svg = svg.replace(/<g id="display">[\s\S]*?<\/g>/, "");
      // macOS renders tray images at 18 points; use an exact 2x raster.
      const size = mac ? "36" : "32";
      const input = join(stage, "tray.svg");
      writeFileSync(input, svg);
      execFileSync(
        process.execPath,
        [cli, "icon", input, "--output", join(stage, "tray"), "--png", size],
        { stdio: "pipe", cwd: root },
      );
      copyFileSync(
        join(stage, `tray/${size}x${size}.png`),
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
