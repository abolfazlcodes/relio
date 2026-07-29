# Product Design Philosophy

## What Relio should feel like

Relio should feel like a well-organized operations desk: quiet until needed,
fast under pressure, explicit about the system being touched, and dependable
enough to leave open all day.

It is neither a decorative terminal nor a general infrastructure dashboard.
The terminal remains direct and familiar. Relio adds context around it so the
user can answer, at a glance:

1. Which workspace am I in?
2. Which host and environment am I touching?
3. Which identity and connection path are active?
4. What operation is running or waiting for me?
5. What will happen if I continue?

The default experience should be approachable to a developer with a few hosts,
but should not slow an operator managing many concurrent sessions.

## Design character

| Quality | Expression in the product |
| --- | --- |
| Calm | Restrained chrome, one dominant work area, status shown near its owner |
| Precise | Exact host, identity, path, port, scope, and operation language |
| Fast | Immediate focus, low-latency palette, stable shortcuts, no startup network work |
| Grounded | Raw terminal behavior stays authoritative; derived views disclose limits |
| Trustworthy | Secure defaults, recognizable trusted prompts, no surprise execution |
| Local | No account framing, online status, sharing affordance, or service dependency |
| Repairable | Errors explain what failed, what remained safe, and the next useful action |

## Design principles

### 1. Context before controls

The current workspace, host, environment, identity, and connection state appear
before session-specific actions. A button labeled only “Run,” “Save,” or
“Connect” is insufficient when the target could be ambiguous.

Context should be persistent but compact. Detail expands on demand through the
inspector or connection details rather than crowding the terminal.

### 2. Terminal truth

Relio must not visually rewrite the shell into a different execution model.
Command history, shell integration, and recording indexes are derived views and
may be incomplete. They never alter terminal output, claim a command succeeded
without transport evidence, or submit input automatically.

Snippet and history actions end at reviewed insertion into the active terminal.
The user submits the command using the shell’s normal input behavior.

### 3. Progressive disclosure

Frequent actions are visible; advanced actions are close, searchable, and
contextual. The design should not expose every SSH option, pane command,
transfer state, and tunnel setting at the same time.

Use this order:

1. safe useful default;
2. concise explanation;
3. optional advanced section;
4. command-palette access for repeat use.

Progressive disclosure must never hide the consequence of a dangerous action.

### 4. Stable spatial memory

Global navigation stays in one place. Workspace context stays in one place.
Tabs remain attached to the editor area. Operation status stays in the bottom
panel/status bar. Trusted confirmation never appears inside terminal content.

Labels, focus behavior, and keyboard order should remain consistent across
terminal, file, editor, and forwarding surfaces.

### 5. Reveal system truth

If a capability is unavailable, say why. If a transfer has indeterminate
progress, do not invent a percentage. If a remote write cannot be atomic, show
the weaker guarantee. If a workspace restores metadata rather than a live
process, use “Reconnect” or “Start new session,” not “Resume.”

### 6. Safe reversibility

Prefer archive, cancel, stop, reset-at-scope, and keep-previous-known-good
patterns. Before destructive work, describe what belongs to Relio, what belongs
to the OS or remote system, and what cannot be undone.

Undo should be offered only when the product can actually guarantee it.

### 7. Accessible by construction

Keyboard completion, visible focus, semantic labels, non-color status cues,
reduced motion, text scaling, and screen-reader announcements are component
requirements rather than later enhancements.

### 8. Density is adjustable, not accidental

Relio should use a comfortable default density. Compact density may reduce
padding and row height within accessibility bounds; it must not reduce target
context, focus visibility, trusted prompt prominence, or minimum hit areas
below the design-system floor.

## UX priorities

When priorities compete, use this order:

1. prevent ambiguous or unintended remote action;
2. preserve terminal input, output, and session integrity;
3. make the user’s target and system state understandable;
4. enable complete keyboard operation;
5. keep common workflows fast and uncluttered;
6. support recovery and useful diagnosis;
7. preserve visual polish and customization.

Visual consistency never justifies hiding an important platform difference or
weakening a security interaction.

## Security-first UX principles

### Make trust evidence legible

Fingerprint, credential, bind, overwrite, and destructive-operation reviews
must use plain language plus exact technical detail. Plain language explains
the decision; technical detail allows independent verification.

### Bind consent to one operation

Confirmation shows the exact target and scope. A previous approval must not
appear to authorize a different host, changed fingerprint, path, port, or
session.

### Distinguish state from content

Trusted prompts and safety indicators are rendered in reserved application
chrome. Terminal text, remote filenames, file contents, theme names, and
diagnostics cannot imitate them convincingly or mark themselves trusted.

### Fail closed without abandoning the user

An uncertain identity, unavailable credential store, unsupported OpenSSH
version, or unverified transfer semantic blocks the unsafe action. The screen
still explains the reason, preserves entered non-secret metadata where safe,
and offers valid remediation.

### Keep warnings proportional

Routine secure behavior should be friction-light. Interrupt only when the user
must make a decision that the system cannot safely choose:

- first-seen host identity;
- changed or revoked host key;
- broad network exposure;
- non-atomic overwrite;
- destructive deletion;
- high-risk credential delegation;
- enabling retention that may capture secrets.

Do not use warning dialogs for normal success, recoverable inline validation,
or passive status.

### Never rely on color alone

Environment, trust, recording, error, and connection state use icon, label,
shape, and accessible name in addition to color.

## Power-user workflow principles

### One action registry

Menu items, buttons, context actions, and keyboard shortcuts resolve to the same
named actions shown in the command palette. Actions expose current availability
and, when disabled, a reason.

### Focus is a first-class state

Every pane has one active surface. Shortcut behavior acts on the active pane
unless the command explicitly names a wider scope. Relio always shows which
terminal will receive inserted text.

### Keep hands on the keyboard

Users can create and switch workspaces, find a host, connect, move focus,
split/close panes, open files, insert snippets, search, inspect operations, and
change settings without a mouse.

### Optimize repeat paths without hiding review

Recently used workspaces, hosts, and actions appear in the palette. Routine
known-host connections can be quick. Review steps remain for identity changes,
dangerous binds, overwrites, credential changes, and command insertion.

### Preserve user orientation at scale

Large inventories use search, filters, favorites, tags, groups, and
virtualization. They do not turn into nested navigation deeper than two
organizational levels. A host may be found by many filters but retains one
stable identity.

### Make configuration source visible

For scoped settings, show effective value and source: built-in, user,
workspace, host, session, or constrained by safety policy. Power should not
require guessing why a value took effect.

## Experience anti-patterns

Relio must not:

- auto-submit a snippet or history command;
- silently trust or replace a host key;
- imply a restored layout contains resurrected processes;
- silently reconnect or restart a tunnel after a crash;
- represent SFTP-based SCP compatibility as legacy SCP support;
- hide host or environment context to gain terminal space;
- place secrets in ordinary editable settings;
- use a generic “Something went wrong” when a safe typed cause exists;
- show false transfer precision;
- use remote content as application navigation or trusted instruction;
- reserve UI for capabilities outside v1.
