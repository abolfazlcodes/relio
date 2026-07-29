import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";

const desktopRoot = resolve(import.meta.dirname, "..");
const tauriConfig = JSON.parse(
  readFileSync(resolve(desktopRoot, "src-tauri/tauri.conf.json"), "utf8"),
) as {
  app: {
    security: {
      capabilities: string[];
      csp: string;
    };
  };
};
const capability = JSON.parse(
  readFileSync(
    resolve(desktopRoot, "src-tauri/capabilities/main.json"),
    "utf8",
  ),
) as { permissions: string[] };
const html = readFileSync(resolve(desktopRoot, "index.html"), "utf8");

describe("desktop shell security configuration", () => {
  it("grants only the bounded local-terminal capability permissions", () => {
    expect(tauriConfig.app.security.capabilities).toEqual(["main"]);
    expect(capability.permissions).toEqual([
      "allow-terminal-list-shell-profiles",
      "allow-terminal-start-local",
      "allow-terminal-grant-output-credit",
      "allow-terminal-send-input",
      "allow-terminal-resize",
      "allow-terminal-stop",
    ]);
  });

  it("blocks remote and executable content in production", () => {
    const csp = tauriConfig.app.security.csp;
    expect(csp).toContain("default-src 'self'");
    expect(csp).toContain("object-src 'none'");
    expect(csp).toContain("frame-src 'none'");
    expect(csp).toContain("form-action 'none'");
    expect(csp).not.toContain("https:");
    expect(csp).not.toContain("unsafe-eval");
  });

  it("loads no remote assets from the entry document", () => {
    expect(html).not.toMatch(/(?:src|href)=["']https?:/u);
  });
});
