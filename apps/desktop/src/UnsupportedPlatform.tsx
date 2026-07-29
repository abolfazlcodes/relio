const tierOnePlatformPattern = /Windows|Macintosh|Mac OS X|Linux/u;

export function isSupportedPlatform(userAgent: string): boolean {
  return !userAgent.includes("Android") && tierOnePlatformPattern.test(userAgent);
}

export function UnsupportedPlatform() {
  return (
    <main className="fatal" role="alert">
      <p className="eyebrow">Unsupported platform</p>
      <h1>Relio cannot run safely on this operating system.</h1>
      <p>
        Use the Windows, macOS, or Linux build from an official Relio release.
        No remote connection was attempted.
      </p>
      <button type="button" onClick={() => window.close()}>
        Exit Relio
      </button>
    </main>
  );
}
