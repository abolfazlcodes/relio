const MAX_REMOTE_TITLE_LENGTH = 160;
const MAX_URI_LENGTH = 2_048;
const CONTROL_CHARACTERS = /[\u0000-\u001f\u007f-\u009f]/gu;

export interface PasteReview {
  requiresConfirmation: boolean;
  preview: string;
  value: string;
}

export function sanitizeRemoteTitle(value: string): string {
  return value
    .replace(CONTROL_CHARACTERS, "")
    .replace(/\s+/gu, " ")
    .trim()
    .slice(0, MAX_REMOTE_TITLE_LENGTH);
}

export function reviewPaste(value: string): PasteReview {
  const requiresConfirmation =
    value.includes("\n") ||
    value.includes("\r") ||
    CONTROL_CHARACTERS.test(value);
  CONTROL_CHARACTERS.lastIndex = 0;

  return {
    preview: escapeForReview(value),
    requiresConfirmation,
    value,
  };
}

export function approvedExternalUri(value: string): string | null {
  if (value.length === 0 || value.length > MAX_URI_LENGTH) return null;

  try {
    const uri = new URL(value);
    if (
      (uri.protocol !== "https:" && uri.protocol !== "http:") ||
      uri.username.length > 0 ||
      uri.password.length > 0
    ) {
      return null;
    }
    return uri.href;
  } catch {
    return null;
  }
}

function escapeForReview(value: string): string {
  return [...value]
    .map((character) => {
      if (character === "\n") return "↵\n";
      if (character === "\r") return "␍";
      if (character === "\t") return "⇥";
      const codePoint = character.codePointAt(0) ?? 0;
      return codePoint < 0x20 || (codePoint >= 0x7f && codePoint <= 0x9f)
        ? `\\u{${codePoint.toString(16).padStart(4, "0")}}`
        : character;
    })
    .join("")
    .slice(0, 4_096);
}
