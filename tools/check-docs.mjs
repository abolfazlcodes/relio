import { existsSync, readFileSync, readdirSync, statSync, writeFileSync } from "node:fs";
import { dirname, extname, relative, resolve } from "node:path";
import process from "node:process";

const repositoryRoot = resolve(import.meta.dirname, "..");
const shouldFix = process.argv.includes("--fix");
const ignoredDirectories = new Set([".git", "node_modules", "target"]);
const findings = [];

function filesBelow(directory) {
  const files = [];

  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    if (entry.isDirectory() && ignoredDirectories.has(entry.name)) {
      continue;
    }

    const path = resolve(directory, entry.name);
    if (entry.isDirectory()) {
      files.push(...filesBelow(path));
    } else {
      files.push(path);
    }
  }

  return files;
}

function normalizedText(path) {
  const original = readFileSync(path, "utf8");
  const lines = original.replaceAll("\r\n", "\n").split("\n");
  const normalized = `${lines
    .map((line) => line.replace(/[ \t]+$/u, ""))
    .join("\n")
    .replace(/\n*$/u, "")}\n`;

  if (original !== normalized) {
    if (shouldFix) {
      writeFileSync(path, normalized);
    } else {
      findings.push(`${relative(repositoryRoot, path)}: formatting differs`);
    }
  }

  return shouldFix ? normalized : original;
}

function checkMarkdown(path, text) {
  let fenceCount = 0;
  for (const line of text.split("\n")) {
    if (line.startsWith("```")) {
      fenceCount += 1;
    }
  }
  if (fenceCount % 2 !== 0) {
    findings.push(`${relative(repositoryRoot, path)}: unbalanced code fence`);
  }

  const links = text.matchAll(/\[[^\]]*\]\(([^)]+)\)/gu);
  for (const [, destination] of links) {
    if (/^(?:https?:|mailto:|#)/u.test(destination)) {
      continue;
    }

    const target = destination.split("#", 1)[0];
    if (target.length > 0 && !existsSync(resolve(dirname(path), target))) {
      findings.push(
        `${relative(repositoryRoot, path)}: missing link target ${destination}`,
      );
    }
  }
}

for (const path of filesBelow(repositoryRoot)) {
  if (!statSync(path).isFile()) {
    continue;
  }

  const extension = extname(path);
  if (![".md", ".json", ".toml", ".yaml", ".yml", ".mjs"].includes(extension)) {
    continue;
  }

  const text = normalizedText(path);
  if (extension === ".md") {
    checkMarkdown(path, text);
  }
}

if (findings.length > 0) {
  process.stderr.write(`${findings.join("\n")}\n`);
  process.exitCode = 1;
} else {
  process.stdout.write(
    shouldFix ? "Documentation formatting updated.\n" : "Repository checks passed.\n",
  );
}
