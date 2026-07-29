# Feature Map

Every v1 capability is part of the reviewed Relio application. The architecture
is local-first: core workflows work without an account or hosted service. The
product does not load separately distributed application code.

| Capability | Owner | First useful milestone | Security and scope notes |
| --- | --- | --- | --- |
| Application shell and command palette | Workbench | 1 | Allowlisted IPC; command registry contains only bundled actions |
| Local terminal | Session runtime | 2 | PTY, terminal rendering, input, resize, deliberate clipboard policy |
| Tabs and split panes | Workspace/workbench | 2 | Bounded versioned layout tree |
| Session restore | Workspace/session runtime | 2 | Restore metadata and layout; never promise process resurrection |
| Encrypted local profile | Persistence service | 3 | One writer; random key rooted in the OS secret store |
| Workspaces and settings | Workspace/settings services | 3 | Local aggregates, scoped settings, versioned redacted export |
| Secure credential management | Credential service | 3 | Opaque handles; secret bytes never enter ordinary frontend state |
| Host management | Host service | 4 | Profiles, groups, tags, favorites, identity and key evidence |
| SSH terminal | Remote transport | 4 | OpenSSH-compatible adapter first; host-key review is mandatory |
| SFTP, SCP, and remote file browser | File-transfer service | 5 | Structured paths, bounded progress, cancellation; SCP requires diagnosed SFTP semantics |
| Remote file editing | File-transfer/editor services | 5 | Bounded built-in text editor, no local plaintext draft, explicit conflict-aware save |
| Port forwarding | Forwarding service | 6 | Loopback default, visible bind/target, owned listener lifecycle |
| Command history and snippets | History/snippet services | 7 | Opt-in retention; single-line reviewed insertion, never synthetic Enter |
| Search | Search service | 7 | Local indexes, permissions and retention follow source data |
| Logging and session recording | Recording service | 7 | Opt-in encrypted segments, quotas, warnings, deletion controls |
| Complete keyboard workflow | Workbench | 7 | Searchable actions, stable shortcuts, accessible focus behavior |
| Theme engine and customizable UI | Theme/workbench services | 8 | Data-only semantic tokens; no scripts, remote assets, or arbitrary styles |

## Capability boundaries

- The session runtime owns PTY lifecycle, terminal bytes, dimensions,
  backpressure, and transport errors.
- The workspace aggregate owns composition, layout, active pane, workspace
  metadata, and restoration. Global hosts and credential handles are references.
- The host service owns connection profiles and references to credential
  handles.
- The remote transport owns SSH session and forwarding lifecycle.
- The file-transfer service owns SFTP/SCP operations, structured remote paths,
  progress, cancellation, and transfer verification.
- The persistence service owns the encrypted profile, migrations, writer lock,
  indexes, and encrypted recording metadata.
- The workbench owns presentation and user intent. It cannot read secret bytes
  or bypass core confirmation and policy.
- The theme engine accepts bounded data tokens only and never executable or
  remotely loaded content.
