import { mkdtempSync, readdirSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";

const root = resolve(import.meta.dirname, "..");
const generated = resolve(root, "apps/desktop/src/generated/ipc");
const write = process.argv.includes("--write");
const output = write ? generated : mkdtempSync(resolve(tmpdir(), "relio-contracts-"));

try {
  const result = spawnSync(
    "cargo",
    ["test", "--package", "relio-desktop", "--lib"],
    {
      cwd: root,
      env: { ...process.env, TS_RS_EXPORT_DIR: output },
      stdio: "inherit",
    },
  );
  if (result.error) throw result.error;
  if (result.status !== 0) process.exit(result.status ?? 1);
  if (!write) compareDirectories(generated, output);
} finally {
  if (!write) rmSync(output, { recursive: true, force: true });
}

function compareDirectories(expectedDirectory, actualDirectory) {
  const expectedFiles = readdirSync(expectedDirectory).sort();
  const actualFiles = readdirSync(actualDirectory).sort();
  if (JSON.stringify(expectedFiles) !== JSON.stringify(actualFiles)) {
    fail("Generated IPC file list is stale.");
  }
  for (const file of expectedFiles) {
    const expected = readFileSync(resolve(expectedDirectory, file), "utf8");
    const actual = readFileSync(resolve(actualDirectory, file), "utf8");
    if (expected !== actual) fail(`Generated IPC contract is stale: ${file}`);
  }
  console.log("Generated IPC contracts are current.");
}

function fail(message) {
  console.error(message);
  console.error("Run `pnpm contracts:generate` and review the generated diff.");
  process.exit(1);
}
