export function FatalBootstrap() {
  return (
    <main className="fatal" role="alert">
      <p className="eyebrow">Relio could not start</p>
      <h1>The bundled interface failed to initialize.</h1>
      <p>Close Relio and start it again. No remote connection was attempted.</p>
      <button type="button" onClick={() => window.close()}>Exit Relio</button>
    </main>
  );
}
