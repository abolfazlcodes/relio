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

It is not responsible for workspace layout, host CRUD, or visual design.

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

The backend owns the session. The frontend terminal model owns emulation and
scrollback and may remain alive while its DOM renderer is detached. If the
entire frontend disconnects, the backend keeps only a bounded replay window and
applies transport backpressure. Reconnection can replay that window; it does not
promise unlimited output or exact reconstruction after a renderer/webview
failure.

## Terminal fidelity

The first renderer uses xterm.js and its maintained addons where appropriate. Compatibility testing must cover:

- ANSI/VT sequences and alternate screen;
- Unicode width, combining characters, emoji, and right-to-left edge cases;
- mouse reporting, bracketed paste, focus reporting, and application key modes;
- resize behavior and `TERM` negotiation;
- common shells and interactive programs such as bash, zsh, fish, PowerShell, tmux, and Neovim;
- high-volume output and long-running commands.

Remote and local terminal output is untrusted. Renderer integration must also
test and constrain:

- OSC hyperlinks and URI scheme allowlists;
- title, working-directory, notification, and shell-integration sequences;
- clipboard read/write sequences, disabled unless a deliberate feature policy
  permits them;
- file/image protocols, disabled by default and separately threat-modeled;
- parser hooks and addons so output cannot invoke core operations;
- oversized, malformed, and adversarial escape-sequence streams.

## Performance rules

- do not send one IPC message per byte;
- batch output on a bounded schedule while preserving order;
- apply flow control when the renderer cannot keep up;
- avoid copying large buffers unnecessarily;
- dispose listeners and renderer resources when a pane closes;
- benchmark cold start, first usable prompt, 10-pane idle, and sustained output.
- bound replay, write, scrollback, search, and recording queues;
- show an explicit output-gap marker if a provider cannot pause and bytes must
  be discarded.

The release budgets, datasets, and regression policy are defined in
[performance and capacity](performance-and-capacity.md).

## Session restore

Restore the workspace layout, session identity, host reference, working
directory when known, and user-selected profile. Do not claim to restore a
process’s exact execution state. Reconnecting creates a new process or remote
channel and never replays commands automatically.

## Recording and derived features

Recording is opt-in per session or workspace. The raw stream is the source;
command blocks, history entries, and search indexes are derived and may be
incomplete. Recordings must display their retention policy, support deletion,
and warn that passwords and tokens can appear in terminal output.

Recordings use encrypted immutable segments. Recording, indexing, and rendering
are separate bounded consumers so a slow indexer cannot block terminal input or
cause unbounded memory growth.

## Local PTY runtime

The v1 native adapter is `portable-pty 0.9.0` behind Relio-owned ports. It selects ConPTY on Windows and POSIX PTYs on macOS and Linux; dependency types never cross into application or IPC contracts. Shell programs are absolute executable paths with structured argument vectors. Relio clears the inherited environment and restores only a bounded non-secret allowlist plus `TERM`, `COLORTERM`, and an opaque session ID. It never builds a user command string.

The runtime owns one writer pump, reader pump, and waiter per session. Input frames are at most 64 KiB, ordered by an exact monotonic sequence, limited to 1 MiB pending and 64 queued frames. Output is read only against receiver-granted credit: a session may have at most 4 MiB credit, emits at most 64 KiB per chunk, and has a 16-chunk bounded delivery channel. Backpressure therefore reaches the PTY instead of growing application memory. PTY bytes remain opaque and potentially hostile.

Closing first drops the PTY writer and waits up to three seconds. Forced cleanup kills the PTY process group with `SIGKILL` on POSIX. Windows assigns the ConPTY child to a kill-on-close Job Object through the safe `win32job` adapter. The waiter records one terminal exit fact and reaps the child. Runtime drop invokes the same cleanup path. Children that deliberately escape a POSIX session remain an operating-system limitation and are covered by release conformance and residual-risk documentation.

Shell discovery uses the current absolute executable shell and known platform locations, deduplicates candidates, and validates executability. An explicit override must be absolute, executable, bounded, NUL-free, and use no more than 16 structured arguments; a working directory must already be an absolute directory. Unsupported or invalid profiles fail before PTY allocation.

Named v1 limits are enforced in `pty.rs`; changing them requires capacity, latency, shutdown, and security evidence. Native conformance covers start, output, ordered input, resize, normal exit, pressure, forced process-group cleanup, and sibling survival on every Tier 1 target.

## Shell integration

Shell integration is an optional enhancement using a small, reviewable script or supported escape-sequence protocol. It should never be required for basic terminal operation and must have a clear install/remove path.

Integration messages are authenticated to the active session where practical,
versioned, size-limited, and treated as hints. A remote process can print the
same bytes as the shell; therefore shell-integration output cannot authorize an
operation, mark content safe, or bypass user confirmation.

## Snippets and history reuse

V1 command snippets are single-line text templates with named, typed text
parameters. Snippet bodies and parameter values reject newline, NUL, escape,
and terminal control characters. Parameters have no secret defaults and are not
retained unless the user deliberately saves a non-secret value in the snippet.

Expanding a snippet or reusing a history entry:

1. shows the active local/remote target and identity;
2. shows the complete resulting line;
3. inserts the line into the active terminal input buffer;
4. never sends Enter or otherwise submits the command.

The shell remains responsible for parsing. Relio does not claim that quoting or
string inspection can prove a command safe. Multiline history can be viewed and
copied as text but is not eligible for direct terminal insertion in v1. A
single-line history entry must pass the same control-character validation
before insertion.
