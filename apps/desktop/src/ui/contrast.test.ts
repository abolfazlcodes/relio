import { describe, expect, it } from "vitest";

const pairs = [
  ["#f1f5f9", "#0b0f14"],
  ["#c2ccd8", "#111821"],
  ["#07111e", "#6daaff"],
  ["#172033", "#f4f7fa"],
  ["#3d4b5f", "#ffffff"],
  ["#ffffff", "#1769d2"],
] as const;

describe("bundled appearance contrast", () => {
  it.each(pairs)("%s on %s meets WCAG AA normal-text contrast", (foreground, background) => {
    expect(contrastRatio(foreground, background)).toBeGreaterThanOrEqual(4.5);
  });
});

function contrastRatio(foreground: string, background: string): number {
  const [lighter, darker] = [
    relativeLuminance(foreground),
    relativeLuminance(background),
  ].sort((left, right) => right - left);
  return ((lighter ?? 0) + 0.05) / ((darker ?? 0) + 0.05);
}

function relativeLuminance(hex: string): number {
  const channels = hex
    .slice(1)
    .match(/.{2}/gu)
    ?.map((channel) => Number.parseInt(channel, 16) / 255)
    .map((channel) =>
      channel <= 0.04045
        ? channel / 12.92
        : ((channel + 0.055) / 1.055) ** 2.4,
    );
  if (!channels || channels.length !== 3) throw new Error("Invalid fixture color");
  return 0.2126 * channels[0]! + 0.7152 * channels[1]! + 0.0722 * channels[2]!;
}
