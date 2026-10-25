import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const read = (path) => readFileSync(resolve(root, path), "utf8");
const version = JSON.parse(read("package.json")).version;
assert.equal(
  JSON.parse(read("src-tauri/tauri.conf.json")).version,
  version,
  "Tauri and npm versions differ",
);
assert.equal(
  read("src-tauri/Cargo.toml").match(/^version = "([^"]+)"/m)?.[1],
  version,
  "Rust and npm versions differ",
);
if (process.env.GITHUB_REF_TYPE === "tag") {
  assert.equal(
    process.env.GITHUB_REF_NAME,
    `v${version}`,
    "Release tag does not match the application version",
  );
}
console.log(`Release version ${version} is consistent.`);
