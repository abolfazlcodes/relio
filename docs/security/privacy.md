# Privacy Principles

## Commitments

Relio is private by default:

- local workflows require no account or hosted service;
- background product analytics and automatic crash upload are absent;
- no advertising, behavioral profiling, or sale of user data;
- collect or transmit only data required for a user-selected operation;
- make remote-operation and update egress visible and controllable;
- local deletion and export remain available.

These are product constraints. A later business model cannot silently override
them.

## Data inventory

| Category | Default location | Leaves device by default | Primary control |
| --- | --- | --- | --- |
| Workspaces, hosts, settings, themes | Encrypted local profile | No | Export/delete/profile lock |
| Credentials | OS secret store, external agent, or referenced key file | Only to the selected authentication target | Credential selection/revocation |
| Terminal input/output | Memory unless retention is enabled | Only through the selected local or remote session | Session close/recording control |
| History, logs, and recordings | Encrypted local profile when enabled | No | Retention/delete/export |
| Unsaved remote-edit content | Memory only | No; changed content uploads only on explicit save | Close/save |
| Update checks | Authoritative update origin | Version, channel, platform/architecture, request metadata | Update setting/offline mode |
| Crash diagnostics | Local preview | No | Explicit local export |

Connection endpoints and timing can themselves be sensitive. Logs and UI do not
treat metadata as harmless.

## Network egress registry

Every network destination belongs to one documented class:

- user-requested SSH, SFTP, SCP, proxy, jump-host, or forwarding endpoint;
- authoritative update origin.

The codebase maintains an egress inventory with owner, purpose, data categories,
authentication, retention expectation, and disable path. Startup performs no
network request. SSH configuration, themes, remote filenames, and terminal
output cannot add an egress destination.

## User intent

Remote connection and transfer intent is bound to a visible host, identity,
path or port, and operation. Relio v1 can generate a previewed local support
bundle but has no built-in diagnostic-upload path. Sharing that export is a
separate user action outside Relio.

Privacy text does not substitute for a technical boundary.

## Product analytics and diagnostics

The stable v1 release has no background product analytics. Adding collection
would require a new scope decision, ADR, threat-model change, privacy review,
minimal event schema, retention and deletion policy, local visibility, and
release-gate tests.

Crash and support bundles:

- are generated locally;
- list every included file and data category before export;
- exclude terminal content, recordings, private paths, and host addresses by
  default;
- apply best-effort structured redaction;
- warn that automatic redaction cannot guarantee removal of every secret.

## User controls

Relio provides:

- an understandable local data-location view;
- per-category retention settings;
- deletion for workspaces, history, recordings, credentials, and caches;
- versioned redacted export;
- an offline mode that disables update checks without blocking local use;
- clear distinction between deleting local references and deleting external
  files or OS credential-store items.

Deletion cannot guarantee physical erasure from SSDs, snapshots, backups, or
filesystem journals. The UI states what was deleted and what remains outside
Relio’s control.

## Development rules

- Test fixtures and screenshots use synthetic hosts and secrets.
- Bug reports and pull requests prompt for sanitized diagnostics.
- Developers do not copy user data into issue trackers or test fixtures.
- New data collection requires an owner, classification, retention, export,
  deletion, and threat-model review.
- Privacy regressions block release with the same priority as security
  regressions.
