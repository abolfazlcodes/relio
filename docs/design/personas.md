# User Personas

## Purpose

These personas represent recurring operating contexts, not demographic
segments. A single user may move between them during a day. Design decisions
should serve their goals without assuming that experience removes the need for
clear target and security context.

## Primary persona: Maya, DevOps engineer

### Context

Maya operates development, staging, and production environments across dozens
of hosts. She keeps several sessions and tunnels open while investigating
deployments and incidents. Interruptions are frequent and mistakes have a large
blast radius.

### Goals

- reach the correct host and environment quickly;
- keep related terminals, files, and tunnels together;
- compare output across hosts without losing pane focus;
- reuse common commands while reviewing the final input;
- understand connection and transfer failures without reconstructing raw SSH
  errors.

### Problems

- similar hostnames make environment mistakes easy;
- terminal tabs lose context after a few concurrent tasks;
- tunnel commands are hard to audit and easy to leave running;
- SSH keys and jump-host paths differ by environment;
- conventional warnings are either vague or frequent enough to ignore.

### Common workflows

1. Open an incident workspace and restore its pane layout.
2. Reconnect staging and production sessions deliberately.
3. Split terminals to compare logs and system state.
4. Open a loopback tunnel to an internal service.
5. Browse and download a remote configuration file.
6. Insert a parameterized diagnostic snippet, review it, then submit.
7. Stop tunnels and close sessions at the end of the incident.

### Important features

- workspaces and environment labels;
- host search, tags, groups, favorites, and jump-chain visibility;
- split panes, session status, and clear active-pane indication;
- snippets, retained history when enabled, and command palette;
- port-forward manager with visible endpoints and ownership;
- typed connection diagnostics and host-key review.

### Design implications

Production context must remain visible even in compact mode. Keyboard switching
must be deterministic. Confirmation should be rare but unmissable when the
target identity or exposure changes.

## Primary persona: Leo, backend developer

### Context

Leo works locally most of the day and connects to a few development or staging
servers for logs, migrations, and configuration checks. He values a polished
terminal but does not remember every SSH or SFTP option.

### Goals

- start a local terminal immediately;
- connect to saved development hosts without repeated setup;
- move a file or make a small remote text edit safely;
- find a command used earlier;
- understand what Relio stores locally.

### Problems

- remote file transfer interrupts the terminal workflow;
- command-line SCP behavior and quoting are easy to get wrong;
- fingerprint prompts are technical and difficult to verify;
- broad settings pages make simple preferences hard to find;
- terminal apps often require configuration-file knowledge for customization.

### Common workflows

1. Launch directly into a local shell.
2. Open a saved development workspace.
3. Connect to one host with an agent identity.
4. upload a build artifact or download a log with visible progress;
5. edit a small UTF-8 configuration file and review a conflict before save;
6. search history and insert a previous one-line command;
7. adjust terminal font size or choose a bundled theme.

### Important features

- fast first launch and local terminal;
- guided host creation and sensible authentication defaults;
- remote file browser, transfers, and bounded editor;
- searchable settings, theme presets, and accessible defaults;
- plain-language security guidance with optional technical detail.

### Design implications

The common path should not expose every SSH control. Advanced settings stay
available without becoming prerequisites. File operations must describe remote
paths and overwrite behavior without expecting protocol expertise.

## Primary persona: Priya, system administrator

### Context

Priya maintains a large mixed-age server inventory. Some systems have unusual
ports, constrained authentication, or temporary legacy algorithm exceptions.
She needs reliable records and explicit failure behavior more than visual
novelty.

### Goals

- organize many hosts without duplicating them across workspaces;
- know which credential source and fingerprint apply to a host;
- diagnose agent, key, DNS, permission, and algorithm failures;
- transfer files without wildcard or remote-shell ambiguity;
- inspect and retire old hosts and credentials safely.

### Problems

- host definitions drift when copied across tools;
- deleting an entry can leave hidden references;
- older servers tempt users to weaken security globally;
- changed host keys are hard to investigate under time pressure;
- non-atomic remote filesystems make writes risky.

### Common workflows

1. Filter the host manager by group, tag, environment, or unavailable
   credential.
2. Review a host’s workspace references and verification history.
3. Connect through a jump host using a selected agent or key file.
4. Add a narrowly scoped, visible legacy algorithm exception.
5. perform a verified SFTP transfer;
6. review permissions, symlinks, and atomicity before replacing a remote file;
7. rotate or remove a credential and repair unresolved references.

### Important features

- global host manager with reference impact;
- credential manager with source, last use, scope, and status;
- fingerprint history and changed-key blocking;
- capability diagnosis and precise error remediation;
- structured SFTP transfer and remote metadata;
- scoped settings with source and expiry visibility.

### Design implications

Bulk visibility matters, but bulk destructive actions should be tightly
limited. Legacy exceptions are per host, named, reviewable, and never hidden in
general settings.

## Primary persona: Andre, infrastructure engineer

### Context

Andre moves between many environments, jump chains, service tunnels, and
long-running investigation sessions. He uses shortcuts heavily and expects
layout restoration, strong search, and low idle resource use.

### Goals

- build reusable workspaces for services or incidents;
- navigate dozens of panes without reaching for the mouse;
- maintain local, remote, and dynamic forwards with explicit ownership;
- retain selected sessions for later analysis without recording everything;
- customize density and terminal appearance without destabilizing safety UI.

### Problems

- large session sets become visually indistinguishable;
- keyboard shortcuts conflict with shells and full-screen terminal programs;
- background recording can consume storage or capture secrets;
- reconnect behavior can accidentally repeat operational work;
- theme customization can reduce readability or obscure severity.

### Common workflows

1. Switch workspace through the command palette.
2. Reconnect only the sessions needed for the current task.
3. Arrange and resize a multi-pane layout.
4. Start and monitor several owned tunnels.
5. Enable recording for one session with a retention limit.
6. Search retained metadata and export a selected diagnostic record locally.
7. tune shortcuts and a workspace-specific theme.

### Important features

- complete keyboard workflow and conflict-aware shortcut editor;
- durable layout with honest reconnect placeholders;
- searchable operation, session, and recording metadata;
- per-session recording state, quota, retention, and deletion;
- safe themes with stable security indicators.

### Design implications

The application must expose focus, ownership, and lifecycle clearly at high
density. Shortcut handling must distinguish application commands from bytes
sent to the terminal.

## Cross-persona needs

| Need | Product response |
| --- | --- |
| Avoid the wrong target | Persistent host, environment, identity, and active-pane context |
| Recover after interruption | Durable layout and metadata, explicit reconnect/start actions |
| Work quickly | Search, palette, recent items, favorites, and stable shortcuts |
| Understand safety | Plain language plus exact technical evidence |
| Keep data private | Local encrypted profile, OS credential store, opt-in retention |
| Diagnose failure | Typed cause, preserved safe state, next useful action |
| Control density | Comfortable and compact modes within accessibility limits |

## Excluded persona assumptions

The v1 design does not model workspace sharing, organizational administration,
cloud account recovery, third-party service setup, runtime add-ons, or automated
assistant behavior. No persona depends on those capabilities.
