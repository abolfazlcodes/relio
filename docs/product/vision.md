# Vision and Product Principles

## Vision

Build the best open-source desktop workspace for secure terminal and remote
host operations: fast, polished, keyboard-first, local-first, and trustworthy
enough for production systems.

The terminal remains the most compatible and powerful interaction surface. The
workspace adds host, file, transfer, tunnel, history, and session context
without hiding what the user is doing.

## Problem

Remote-system work is spread across terminal tabs, SSH profiles, text files,
snippets, port-forward commands, log viewers, and file-transfer tools. Existing
products are often strong in one dimension:

- powerful but difficult to discover;
- attractive but restrictive;
- configurable but fragmented;
- broad but inconsistent;
- convenient but dependent on a hosted service.

Relio treats hosts, sessions, files, tunnels, and workflows as related pieces
of one local workspace.

## Primary users

1. Application developers who occasionally operate remote environments.
2. DevOps, SRE, and platform engineers who manage many hosts and sessions.
3. Developers who need a polished terminal plus secure remote file and tunnel
   workflows.
4. Open-source contributors who prefer a focused, well-documented core.

## Product promises

- Open a useful terminal quickly.
- Find the right host or workspace without remembering where it was configured.
- Browse, transfer, and edit remote files with visible target context.
- Keep advanced workflows available without making defaults intimidating.
- Make dangerous or irreversible actions visible and deliberate.
- Keep user state and credentials under local control.
- Prefer a smaller reviewed codebase over runtime-loaded customization.

## Non-goals

- Reimplement every infrastructure console.
- Replace a full IDE or general-purpose text editor.
- Hide shell behavior behind an opaque command abstraction.
- Require an account or hosted service for any v1 workflow.
- Support every remote protocol in the first release.
- Load remote or separately distributed executable application code.
- Promise identical native behavior where operating systems differ.

## Product principles

### Local-first

All durable product state is local. Settings, hosts, workspaces, retained
history, and session metadata live in an encrypted profile. Credentials remain
in the operating-system credential store. Recording and history retention are
user-controlled, and startup does not depend on a network service.

### Secure by design

Relio minimizes the trusted computing base, network origins, process types, and
privileged interfaces. All v1 behavior is reviewed and signed together.
Theme records and SSH configuration are bounded data, never executable
authority.

### Progressive disclosure

The default surface is calm. Power features remain reachable through the
command palette, context actions, and advanced views rather than appearing
everywhere.

### Terminal truth

The raw terminal stream is authoritative. Command blocks, retained history, and
search indexes are derived views and must never silently alter terminal input
or output.

### Safe remote operations

The UI shows target, identity, command or operation, and affected path or port
before material impact. Host-key changes, broad port binds, remote overwrites,
and destructive actions require core-owned review flows.

### Small trusted product

Core modules expose narrow typed interfaces to each other, but Relio v1 is one
cohesive signed product. Customization is limited to settings, shortcuts,
layouts, snippets, and safe theme tokens. New application behavior goes through
normal design, review, testing, and release.

### Observable quality

Performance, reliability, accessibility, privacy, and security are acceptance
criteria. They are not polish added after feature completion.

## Initial success signals

These are product hypotheses, not launch commitments:

- a new user reaches a working local shell in under one minute;
- a returning user restores a previous workspace without manual reconstruction;
- a user can connect to a saved SSH host without re-entering routine details;
- a user can complete SFTP/SCP transfer and remote-edit workflows without
  constructing shell commands;
- common actions are discoverable through search and the command palette;
- idle CPU and memory remain low enough for all-day use;
- no v1 workflow requires an account or background service.
