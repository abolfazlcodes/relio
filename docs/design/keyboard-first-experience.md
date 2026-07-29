# Keyboard-First Experience

## Goals

A user must be able to complete every core Relio workflow without a pointer:

- navigate workspaces, hosts, library, settings, panels, tabs, and panes;
- create and connect sessions;
- browse and transfer files;
- edit and save a supported remote text file;
- create and insert snippets/history;
- manage port forwards;
- inspect operation failures;
- manage credentials, recording, themes, and settings;
- review and cancel security decisions.

Keyboard efficiency must not steal ordinary shell input unpredictably.

## Shortcut model

Relio defines logical actions with platform defaults. The shortcut editor
stores the logical binding and displays the correct platform glyphs.

Terms:

- **Command** means `⌘` on macOS.
- **Ctrl** means Control.
- **Alt** means Option on macOS and Alt on Windows/Linux.
- **Terminal-safe modifier** means the platform combination chosen to avoid
  common shell control sequences, usually Command on macOS and Ctrl+Shift on
  Windows/Linux.

Platform defaults may differ where native convention or terminal safety makes a
single cross-platform chord misleading.

## Default global shortcuts

| Action | macOS | Windows / Linux |
| --- | --- | --- |
| Open command palette | `⌘⇧P` | `Ctrl+Shift+P` |
| Global search | `⌘⇧F` | `Ctrl+Shift+G` |
| Switch workspace | `⌘⇧O` | `Ctrl+Shift+O` |
| Focus activity rail | `⌃1` | `Ctrl+Alt+1` |
| Focus sidebar | `⌃2` | `Ctrl+Alt+2` |
| Focus active surface | `⌃3` | `Ctrl+Alt+3` |
| Focus inspector | `⌃4` | `Ctrl+Alt+4` |
| Focus bottom panel | `⌃5` | `Ctrl+Alt+5` |
| Cycle regions forward/backward | `F6` / `⇧F6` | `F6` / `Shift+F6` |
| Toggle sidebar | `⌘B` | `Ctrl+Shift+B` |
| Toggle operations panel | `⌘J` | `Ctrl+Shift+J` |
| Open Settings | `⌘,` | `Ctrl+,` |
| Cancel current overlay/operation prompt | `Esc` | `Esc` |

Region-number defaults must be checked against OS-reserved shortcuts during
platform implementation. If a default is unavailable, F6 remains the universal
fallback and the shortcut editor reports the platform conflict.

## Session and surface shortcuts

| Action | macOS | Windows / Linux |
| --- | --- | --- |
| New local terminal | `⌘T` | `Ctrl+Shift+T` |
| Connect to host | `⌘⇧H` | `Ctrl+Shift+H` |
| Close active surface | `⌘W` | `Ctrl+Shift+W` |
| Next tab | `⌘⌥Right` | `Ctrl+PageDown` |
| Previous tab | `⌘⌥Left` | `Ctrl+PageUp` |
| Open tab/session switcher | `⌘⇧E` | `Ctrl+Shift+E` |
| Split right | `⌘D` | `Ctrl+Shift+\\` |
| Split down | `⌘⇧D` | `Ctrl+Shift+-` |
| Focus pane by direction | `⌘⌥Arrow` | `Ctrl+Alt+Arrow` |
| Resize pane | `⌘⌥⇧Arrow` | `Ctrl+Alt+Shift+Arrow` |
| Maximize/restore active pane | `⌘⇧M` | `Ctrl+Shift+M` |
| Open active-surface details | `⌘I` | `Ctrl+Shift+I` |

OS window-management conflicts take precedence. Relio diagnoses conflicts and
does not register a shortcut it cannot own reliably.

## Terminal shortcuts

| Action | macOS | Windows / Linux |
| --- | --- | --- |
| Copy selection | `⌘C` | `Ctrl+Shift+C` |
| Paste | `⌘V` | `Ctrl+Shift+V` |
| Find in active terminal | `⌘F` | `Ctrl+Shift+F` |
| Find next / previous | `⌘G` / `⌘⇧G` | `Enter` / `Shift+Enter` while find is open |
| Increase terminal font | `⌘+` | `Ctrl++` |
| Decrease terminal font | `⌘-` | `Ctrl+-` |
| Reset terminal font | `⌘0` | `Ctrl+0` |
| Clear visible terminal through shell | No application binding | No application binding |
| Send interrupt | Shell receives `Ctrl+C` | Shell receives `Ctrl+C` |

Relio does not remap ordinary `Ctrl+C`, `Ctrl+R`, `Ctrl+W`, `Ctrl+K`, arrow,
function, or escape sequences used by shells/full-screen programs on
Windows/Linux. Application defaults use additional modifiers when necessary.

Paste policy:

- a single safe text line passes to the terminal;
- multiline or control-character paste opens a preview by default;
- the preview shows the active target and complete bounded text;
- the user may paste, cancel, or copy to a local plain-text review;
- Relio never adds Enter after paste.

## Workspace shortcuts

| Action | macOS | Windows / Linux |
| --- | --- | --- |
| Switch workspace | `⌘⇧O` | `Ctrl+Shift+O` |
| Create workspace | Palette default | Palette default |
| Next recently used workspace | `⌘⌃Right` | `Ctrl+Alt+PageDown` |
| Previous recently used workspace | `⌘⌃Left` | `Ctrl+Alt+PageUp` |
| Open workspace Overview | `⌘⇧U` | `Ctrl+Shift+U` |

Number-key workspace assignments are optional user bindings, not defaults.
This avoids collision with tab navigation and makes large workspace sets
search-first.

## Search shortcuts

Three search scopes remain distinct:

| Scope | Shortcut and behavior |
| --- | --- |
| Command palette | Global shortcut; searches actions and destinations |
| Global retained-data search | Global Search shortcut; labels enabled data sources |
| In-surface find | Surface-specific shortcut; searches current terminal/file/list |

Search does not route plain typing away from a focused terminal. Invoking search
is always an explicit shortcut or action.

## Command palette

### Result groups

- Actions;
- Workspaces;
- Hosts;
- Open sessions and tabs;
- Snippets;
- Settings.

### Optional filters

The palette works without query syntax. Power users may filter with:

- `>` actions;
- `@` hosts;
- `#` workspaces;
- `/` open sessions and tabs;
- `:` snippets and retained history.

The filter characters are search syntax only. They never become shell commands
or remote input.

### Keyboard behavior

1. Open palette; focus lands in search.
2. Type to filter; selection follows the first available result.
3. Up/Down changes result.
4. Right Arrow opens non-mutating detail where available.
5. Enter invokes the selected action.
6. Escape closes and restores prior focus.

For actions requiring a target, Enter moves into a second target step within
the palette. For actions requiring security/destructive review, the palette
closes and the trusted dialog opens. Selecting an item never bypasses review.

### Availability

Disabled results remain searchable when discoverability is useful and show a
short reason such as `No active terminal` or `SSH provider unavailable`.
Commands that cannot exist in the current product scope are absent rather than
disabled.

## Navigation behavior

### Activity rail and sidebar

- Arrow keys move within one navigation list.
- Home/End move to first/last item.
- Enter activates.
- Right Arrow expands a collapsible group; Left Arrow collapses or moves to its
  parent.
- Type-ahead finds visible list labels without entering a persistent search
  mode.
- Escape clears a filter first, then returns focus to the owning region.

Groups are at most two levels deep. Keyboard navigation never requires
traversing thousands of off-screen records; search and virtualization provide
bounded results.

### Tabs

- Tab key moves among controls, not among every tab panel’s hidden content.
- Arrow keys move between tabs when the tab strip has focus.
- Enter/Space activates the focused tab.
- Delete or platform close shortcut invokes the normal close policy.
- Reorder uses an explicit keyboard move command with announced new position.

### Panes

Directional focus chooses the nearest pane in that direction. When several
overlap spatially, choose the pane with greatest shared edge, then most recent
focus. If there is no neighbor, keep focus and announce `No pane to the right`
without cycling unpredictably.

Keyboard resizing uses small repeated increments and announces approximate
relative size. Escape cancels an in-progress resize mode.

## Focus rules

1. Only one region and control owns focus.
2. Active pane is persistent workspace state; keyboard focus may temporarily
   move to chrome without changing it.
3. Returning to the editor restores the last focused element inside the active
   pane.
4. A user-initiated new session may receive focus when ready only if the user
   has not moved elsewhere.
5. Background completion never steals focus.
6. Modal close returns focus to the invoking control or a safe logical
   successor if it no longer exists.
7. Focus remains visible in every theme and density.

## Input-routing priority

Keyboard input is resolved in this order:

1. OS-owned secure input, picker, or system menu;
2. active trusted modal;
3. command palette or explicit overlay;
4. focused form/editor control;
5. registered application shortcut;
6. focused terminal byte stream.

Plain printable keys in a focused terminal always reach the terminal. A remote
process cannot register an application shortcut or consume a trusted-dialog
decision.

## Shortcut customization

Settings > Keyboard presents:

- searchable action list grouped by domain;
- current platform binding;
- scope: global or terminal-focused;
- conflict and OS-reserved status;
- reset action;
- whether a change applies immediately.

### Recording a binding

1. Select `Change`.
2. Relio enters a clearly labeled capture state.
3. Press the desired chord or Escape to cancel.
4. The editor shows exact keys and conflicts.
5. Resolve with `Replace existing`, `Keep current`, or `Cancel`.

Relio does not allow:

- plain printable keys as global shortcuts;
- a single unmodified Escape/Enter/Space for mutating actions;
- bindings that intercept required secure-dialog cancellation/navigation;
- two active commands with the same scope and chord;
- invisible layout/focus commands without an accessible alternative.

### Reset and export

Reset may target one action, one category, or all user bindings after preview.
The normal redacted settings export may include non-secret logical bindings.
It does not activate imported executable behavior; settings import is outside
the current v1 design unless separately specified by the settings architecture.

## Screen-reader operation

- Shortcut glyphs have spoken equivalents.
- Palette result count and group changes use polite announcements.
- Pane focus announces session name, host/environment, connection, and
  recording state.
- Split creation/closure announces the new pane count and active pane.
- Long fingerprints and paths provide readable grouping plus copy.
- Live terminal accessibility uses the maintained renderer’s accessible mode
  without duplicating every output byte into the workbench state.

## Full-screen terminal programs

Relio offers a `Send all keys to terminal` temporary mode for programs with
heavy shortcut use:

- entered/exited through a user-configured chord and the pane menu;
- status bar clearly shows the mode;
- trusted system Escape/cancel and the exit chord remain available;
- global palette can still be reached through the application menu;
- mode is per session and not persisted silently.

This mode changes input routing only. It cannot bypass recording status,
trusted confirmation, or security policy.

## Keyboard acceptance scenarios

Before release, test mouse-free completion of:

- first launch to local terminal;
- create workspace, add host, verify fingerprint, and connect;
- create four panes, switch, resize, and close them;
- upload/download and resolve a conflict;
- open, edit, conflict-check, save, and close a remote file;
- create and insert a snippet without submission;
- find and insert one-line history;
- start/stop a tunnel and session recording;
- edit a theme and repair contrast;
- rotate/remove a credential;
- diagnose a failed jump host;
- operate at 200% text scaling and in high-contrast mode.
