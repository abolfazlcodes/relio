export function App() {
  return (
    <main className="shell">
      <header className="titlebar">
        <span className="wordmark">Relio</span>
        <span className="phase">Foundation preview</span>
      </header>
      <section className="empty-state" aria-labelledby="welcome-title">
        <div className="mark" aria-hidden="true">R</div>
        <p className="eyebrow">Local-first operations workspace</p>
        <h1 id="welcome-title">The secure application shell is ready.</h1>
        <p className="description">
          Terminal and infrastructure features will appear as their milestones
          are completed. This build performs no startup network activity.
        </p>
      </section>
      <footer className="statusbar">
        <span>Local mode</span><span>No profile open</span>
      </footer>
    </main>
  );
}
