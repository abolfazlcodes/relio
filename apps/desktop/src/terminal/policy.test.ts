import { describe, expect, it } from "vitest";
import { approvedExternalUri, reviewPaste, sanitizeRemoteTitle } from "./policy";

describe("terminal trust-boundary policy", () => {
  it("keeps remote titles inert and bounded", () => {
    expect(sanitizeRemoteTitle("\u001b]0;prod\u0007\n server")).toBe("]0;prod server");
    expect(sanitizeRemoteTitle("x".repeat(500))).toHaveLength(160);
  });

  it("requires confirmation for multiline or control-bearing paste", () => {
    expect(reviewPaste("echo safe").requiresConfirmation).toBe(false);
    expect(reviewPaste("echo one\necho two")).toMatchObject({
      preview: "echo one↵\necho two",
      requiresConfirmation: true,
    });
    expect(reviewPaste("a\u0003b").preview).toBe("a\\u{0003}b");
  });

  it("accepts only credential-free HTTP destinations", () => {
    expect(approvedExternalUri("https://example.test/path?q=1")).toBe(
      "https://example.test/path?q=1",
    );
    expect(approvedExternalUri("javascript:alert(1)")).toBeNull();
    expect(approvedExternalUri("ssh://host")).toBeNull();
    expect(approvedExternalUri("https://user:secret@example.test")).toBeNull();
  });
});
