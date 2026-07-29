# Testing Strategy

Testing is layered so most behavior can be verified without a desktop window, network, or real production host.

## Test pyramid

### Unit tests

Test domain models, validation, settings precedence, layout operations, migration logic, command parsing, capability checks, and error classification in isolation. These should be fast and deterministic.

### Contract tests

Test IPC schemas, transport interfaces, plugin protocol messages, theme schemas, and repository interfaces against fixtures. Contract tests protect the boundaries that multiple modules depend on.

### Integration tests

Use local fake shells, temporary directories, a test SQLite database, and a controlled SSH/SFTP fixture. Verify process cleanup, cancellation, reconnect behavior, transfer integrity, and migration upgrades.

### End-to-end tests

Run a small stable suite against the packaged desktop app: launch, create workspace, open terminal, split pane, save setting, add host, and restore layout. Keep selectors semantic and avoid asserting internal DOM structure.

### Compatibility tests

Run terminal behavior fixtures against common shells and interactive programs where licensing and CI environments allow. Maintain a manual matrix for platform-specific input, fonts, IME, clipboard, file pickers, keychains, and window behavior.

### Performance tests

Measure cold start, first usable prompt, pane creation, sustained output, 10-pane idle, history search, large log view, plugin activation, and memory after long sessions. Record platform, hardware profile, build mode, and dataset size.

## Test policy

- a bug fix adds a regression test at the lowest useful layer;
- security-sensitive behavior has both positive and negative tests;
- flaky tests are quarantined with an owner and expiry date, not ignored indefinitely;
- network-dependent tests use deterministic fixtures and never real user credentials;
- terminal output fixtures must be reviewed for secrets before committing;
- accessibility checks are part of UI feature completion.

## Release gates

A release candidate requires passing unit, contract, integration, smoke E2E, security-sensitive, and migration tests on the supported platform matrix. Performance regressions beyond the documented budget require an explicit decision record or a rollback.
