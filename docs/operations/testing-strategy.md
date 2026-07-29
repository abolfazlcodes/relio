# Testing Strategy

Testing is layered so most behavior can be verified without a desktop window, network, or real production host.

## Test pyramid

### Unit tests

Test domain models, validation, settings/policy precedence, workspace reference
ownership, layout operations, migration logic, command parsing, capability
checks, key-version state, and error classification in isolation. These should
be fast and deterministic.

### Contract tests

Test IPC schemas and per-window allowlists, transport capability interfaces,
SFTP packet framing and request correlation, theme schemas, encrypted blob
formats, workspace export formats, and repository interfaces against fixtures.
Contract tests protect the boundaries that multiple modules depend on.

### Integration tests

Use local fake shells, protected temporary directories, encrypted test profiles,
a controlled SSH/SFTP/SCP fixture, fake keychain states, and malformed SSH
configuration, theme, and remote-file fixtures. Verify process cleanup, cancellation,
reconnect behavior, transfer integrity, encryption/tamper behavior, path-scope
enforcement, and migration upgrades.

### End-to-end tests

Run a small stable suite against the packaged desktop app: launch,
create/archive a workspace, open a terminal, split a pane, save a scoped
setting, add a host, exercise a host-key fixture, perform one SFTP and one SCP
transfer, detect a remote-edit conflict, create and stop a loopback tunnel,
review and insert a snippet without submitting it, enable/delete a synthetic
recording, invoke an action through the command palette, apply a valid theme,
restore layout, enter
recovery mode, and verify no startup network request. Keep selectors semantic
and avoid asserting internal DOM structure.

### Compatibility tests

Run terminal behavior fixtures against common shells and interactive programs
where licensing and CI environments allow. Maintain the Tier 1 matrix for
platform-specific input, fonts, IME, clipboard, file pickers, keychains,
webviews, OpenSSH versions, signing, and window behavior.

### Performance tests

Measure cold start, first usable prompt, pane creation, sustained output,
10-pane idle, history search, large log view, encrypted query/migration cost,
remote file-list first page, transfer throughput, and memory after long
sessions. Use the datasets and budgets in
[performance and capacity](../architecture/performance-and-capacity.md).

## Test policy

- a bug fix adds a regression test at the lowest useful layer;
- security-sensitive behavior has both positive and negative tests;
- flaky tests are quarantined with an owner and expiry date, not ignored indefinitely;
- network-dependent tests use deterministic fixtures and never real user credentials;
- terminal output fixtures must be reviewed for secrets before committing;
- accessibility checks are part of UI feature completion;
- security fixtures use synthetic identities and run only inside the intended
  sandbox/containment;
- every parser exposed to SSH configuration, remote, theme, or IPC input has
  malformed and size-limit tests;
- tests inspect database, WAL/journal, blob, backup, logs, and serialized IPC for
  plaintext canaries.

## Release gates

A release candidate requires passing unit, contract, integration, smoke E2E,
security-sensitive, update/rollback, and migration tests on the supported
platform matrix. Tier 1 failures block release. A performance budget failure
requires a measured decision record with owner/expiry or rollback of the
regression.
