# Terminal Architecture

## Responsibilities

The terminal subsystem is responsible for:

- starting and stopping local shells;
- attaching to a remote interactive channel;
- allocating and resizing a PTY where supported;
- moving bytes with backpressure;
- exposing terminal metadata and lifecycle state;
- connecting a renderer without coupling the runtime to a particular UI library;
- optional recording, search indexing, and shell integration.

It is not responsible for workspace layout, host CRUD, plugin discovery, or visual design.

## Data path

```text
shell / remote channel
        ↓ bytes
transport adapter
        ↓ normalized stream
session runtime
        ↓ bounded, batched IPC stream
terminal view
        ↓ user input and resize events
session runtime → transport adapter → shell / remote channel
```

The backend owns the session and remains alive if the view is temporarily unmounted. The view may reconnect to a session stream after navigation or a renderer failure.

## Terminal fidelity

The first renderer uses xterm.js and its maintained addons where appropriate. Compatibility testing must cover:

- ANSI/VT sequences and alternate screen;
- Unicode width, combining characters, emoji, and right-to-left edge cases;
- mouse reporting, bracketed paste, focus reporting, and application key modes;
- resize behavior and `TERM` negotiation;
- common shells and interactive programs such as bash, zsh, fish, PowerShell, tmux, and Neovim;
- high-volume output and long-running commands.

## Performance rules

- do not send one IPC message per byte;
- batch output on a bounded schedule while preserving order;
- apply flow control when the renderer cannot keep up;
- avoid copying large buffers unnecessarily;
- dispose listeners and renderer resources when a pane closes;
- benchmark cold start, first usable prompt, 10-pane idle, and sustained output.

Initial targets are hypotheses to validate on representative hardware: first usable local prompt within two seconds on a warm development build, no visible input lag during sustained output, and bounded memory growth during long sessions.

## Session restore

Restore the workspace layout, session identity, host reference, working directory when known, and user-selected profile. Do not claim to restore a process’s exact execution state unless a supported remote multiplexer or separate persistence mechanism is attached.

## Recording and derived features

Recording is opt-in per session or workspace. The raw stream is the source; command blocks, history entries, summaries, and detectors are derived and may be incomplete. Recordings must display their retention policy, support deletion, and warn that passwords and tokens can appear in terminal output.

## Shell integration

Shell integration is an optional enhancement using a small, reviewable script or supported escape-sequence protocol. It should never be required for basic terminal operation and must have a clear install/remove path.
