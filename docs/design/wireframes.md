# Low-Fidelity Wireframes

## Purpose and conventions

These wireframes specify hierarchy, persistent context, action placement, and
state behavior. They do not prescribe final visual styling.

Legend:

- `[Action]` — button or selectable action;
- `( )` / `(x)` — radio choice;
- `[ ]` / `[x]` — checkbox;
- `▼` — menu or selector;
- `…` — bounded additional content;
- `!` — warning or state needing attention;
- `◆` — trusted Relio safety surface;
- `│` and `─` — region boundaries, not final borders.

All screens inherit the keyboard, responsive, accessibility, and trusted-chrome
rules from the other design documents.

## 1. Welcome screen

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ RELIO                                                     [Settings] [?] │
├──────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│                         Secure remote work, locally                       │
│                                                                          │
│        Terminals, SSH, remote files, and tunnels in one workspace.       │
│        Your profile stays encrypted on this device.                      │
│                                                                          │
│                      [ Open local terminal ]                              │
│                                                                          │
│                 [ Set up an SSH host ]  [ Create workspace ]             │
│                                                                          │
│        ┌──────────────────────────────────────────────────────────┐      │
│        │  No account required                                    │      │
│        │  Credentials use your OS secure store or SSH agent       │      │
│        │  Remote connections begin only when you ask              │      │
│        └──────────────────────────────────────────────────────────┘      │
│                                                                          │
├──────────────────────────────────────────────────────────────────────────┤
│ Local shell: Ready        Protected profile: Ready        Offline        │
└──────────────────────────────────────────────────────────────────────────┘
```

Primary focus starts on `Open local terminal`. The security commitments are
visible but do not become a mandatory tour.

### Protected-store unavailable variant

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Protected profile unavailable                                           │
│                                                                          │
│ Relio could not access this system's protected credential store.         │
│ It did not create a replacement or plaintext profile.                    │
│                                                                          │
│ Platform status: Secret Service is locked                                │
│                                                                          │
│ [ Try again ]  [ View local help ]  [ Open temporary local terminal ]    │
│                                                                          │
│ Temporary mode does not save hosts, workspaces, history, or settings.     │
└──────────────────────────────────────────────────────────────────────────┘
```

## 2. Main workspace

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Relio  API development ▼    api-staging · deploy       [Search] [⌘/Ctrl] │
├────┬───────────────────┬──────────────────────────────────────┬───────────┤
│ ◫  │ API DEVELOPMENT   │ api-staging          +              │ SESSION   │
│    │                   ├──────────────────────────────────────┤           │
│ ▣  │ Overview          │ ┌─ api-staging · deploy ─ Connected┐│ SSH       │
│    │ Sessions       3  │ │                                  ││ deploy@…  │
│ ≣  │ Hosts          4  │ │ $ _                              ││ Staging   │
│    │ Remote files      │ │                                  ││           │
│ ⚙  │ Port forwards  1  │ │                                  ││ Via bast. │
│    │ Activity           │ │                                  ││           │
│    │                   │ └──────────────────────────────────┘│ [Details] │
│    │ PINNED HOSTS      │                                      │           │
│    │ ◆ api-prod        │                                      │           │
│    │ ○ worker-stage    │                                      │           │
│    │                   │                                      │           │
│    │ [+ Add]           ├──────────────────────────────────────┴───────────┤
│    │                   │ Operations  Transfers  Problems                  │
│    │                   │ 1 loopback forward active                        │
├────┴───────────────────┴──────────────────────────────────────────────────┤
│ API development  │ api-staging · deploy │ Connected │ 1 forward │ REC off│
└──────────────────────────────────────────────────────────────────────────┘
```

The terminal is dominant. The right inspector and bottom panel may be
collapsed. Selecting navigation does not change the active terminal input
target.

## 3. Host manager

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Relio  Hosts                                         [Search] [Palette]  │
├────┬───────────────────┬──────────────────────────────────────────────────┤
│ ◫  │ HOSTS             │ Hosts                              [ + Add host ]│
│    │ [Search hosts]    │ [Environment ▼] [Group ▼] [Tags ▼] [Sort ▼]   │
│ ▣  │                   ├───┬─────────────────┬─────────┬────────┬─────────┤
│    │ All hosts     128 │ ★ │ Name / address  │ User    │ Env    │ Status  │
│ ≣  │ Favorites      8 │ ☆ │ api-dev         │ deploy  │ DEV    │ Ready   │
│    │ Recent         12 │ ★ │ api-staging     │ deploy  │ STAGE  │ Ready   │
│ ⚙  │                   │ ★ │ api-prod        │ ops     │ PROD   │ ! Key   │
│    │ GROUPS            │ ☆ │ worker-04       │ admin   │ PROD   │ Offline │
│    │ Web            24 │   │ …               │         │        │         │
│    │ Databases      10 ├───┴─────────────────┴─────────┴────────┴─────────┤
│    │                  │ api-prod                                             │
│    │ ENVIRONMENTS     │ ops@api.example:22 · Production                     │
│    │ Development      │                                                      │
│    │ Staging          │ [ Connect ] [ Open files ] [ More ▼ ]               │
│    │ Production       │                                                      │
│    │                  │ Identity needs review · 3 workspace references       │
└────┴──────────────────┴──────────────────────────────────────────────────────┘
```

The selected-host preview exposes the primary state and actions. Full detail
opens as a surface or inspector; it does not turn the table row into a dense
form.

### Empty host manager

```text
No SSH hosts yet

Save connection metadata once, then reference the same host from any workspace.

[ Add SSH host ]    [ Review supported SSH configuration import ]
```

## 4. Connection dialog

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Add SSH host                                                        [×] │
├──────────────────────────────────────────────────────────────────────────┤
│  1 Target  ─────  2 Authentication  ─────  3 Connection path  ─ 4 Review│
│                                                                          │
│  Display name                                                            │
│  [ api-staging______________________________________________________ ]   │
│                                                                          │
│  Hostname or address                         Port                         │
│  [ api.staging.example__________________ ]   [ 22____ ]                  │
│                                                                          │
│  Username                                                                │
│  [ deploy___________________________________________________________ ]   │
│                                                                          │
│  Environment                         Group                               │
│  [ Staging ▼ ]                       [ Web ▼ ]                           │
│                                                                          │
│  Resolved target: deploy@api.staging.example:22                          │
│                                                                          │
├──────────────────────────────────────────────────────────────────────────┤
│ [ Cancel ]                                           [ Next: Authentication ]│
└──────────────────────────────────────────────────────────────────────────┘
```

### Review step

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Review connection                                                        │
├──────────────────────────────────────────────────────────────────────────┤
│ Target          deploy@api.staging.example:22                            │
│ Environment     STAGING                                                  │
│ Credential      SSH Agent · ED25519 SHA256:ABCD…                         │
│ Connection path local → bastion.example → api.staging.example            │
│ Provider        OpenSSH 9.x · Supported                                  │
│ Capabilities    Terminal · SFTP · SCP-compatible via SFTP · Forwarding   │
│ Workspace       API development                                          │
│                                                                          │
│ Testing may open a host-identity review. No terminal command will run.    │
├──────────────────────────────────────────────────────────────────────────┤
│ [ Back ] [ Save host ]                  [ Test connection ] [ Save & connect ]│
└──────────────────────────────────────────────────────────────────────────┘
```

### Trusted first-seen fingerprint overlay

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ ◆ RELIO SECURITY CHECK                                                   │
│ Verify host identity                                                     │
├──────────────────────────────────────────────────────────────────────────┤
│ Relio has not seen a key for this host and port before.                  │
│                                                                          │
│ Host          api.staging.example:22                                     │
│ Identity      deploy · via bastion.example                               │
│ Environment   STAGING                                                    │
│ Algorithm     ED25519                                                    │
│ Fingerprint   SHA256:abcd efgh ijkl mnop qrst uvwx yz12 3456             │
│ Source        Presented by the current connection                        │
│                                                                          │
│ Compare this value through an independent trusted channel.  [Copy]       │
│ [ ] I have compared the fingerprint or accept this identity              │
├──────────────────────────────────────────────────────────────────────────┤
│ [ Cancel connection ]                    [ Trust once ] [ Trust and save ]│
└──────────────────────────────────────────────────────────────────────────┘
```

The trusted frame, label, evidence hierarchy, and actions do not inherit
user-theme styling.

## 5. Terminal view

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ API development ▼    api-prod · ops · PRODUCTION      [Files] [Split] [•••]│
├──────────────────────────────────────────────────────────────────────────┤
│ api-prod · ops                         Connected · via bastion       REC ●│
├──────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ops@api-prod:~$ systemctl status relio-api                              │
│  ● relio-api.service - Relio API                                         │
│     Active: active (running)                                              │
│                                                                          │
│  ops@api-prod:~$ _                                                       │
│                                                                          │
│                                                                          │
│                                                                          │
│                                                                          │
├──────────────────────────────────────────────────────────────────────────┤
│ API development │ api-prod · ops │ Connected │ Recording 00:12:48 │ UTF-8│
└──────────────────────────────────────────────────────────────────────────┘
```

Remote context and recording state stay visible. Terminal output does not enter
the top bar, pane header, or status bar as trusted instruction.

### Disconnected state

```text
┌─ api-prod · ops ─ Disconnected at 14:32 ─────────────────────────────────┐
│                                                                          │
│ The SSH connection ended during an active session.                       │
│ No command or tunnel will restart automatically.                         │
│                                                                          │
│ [ Reconnect in a new session ] [ View connection details ] [ Close ]     │
└──────────────────────────────────────────────────────────────────────────┘
```

Existing scrollback remains a terminal-derived view and is clearly separated
from the reconnect action.

## 6. Split terminal view

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Incident 248 ▼                                 [Layout ▼] [New split]    │
├──────────────────────────────────────┬───────────────────────────────────┤
│ api-prod · ops · PROD    Connected   │ worker-prod · ops · PROD Connected│
├──────────────────────────────────────┤                                   │
│                                      │ $ journalctl -f                    │
│ $ kubectl get pods                   │ …                                 │
│ …                                    │                                   │
│                                      ├───────────────────────────────────┤
│                                      │ REMOTE FILES · worker-prod        │
│                                      │ /etc/relio/                       │
│                                      │  config.toml                      │
│                                      │  workers.toml                     │
│                                      │                                   │
├──────────────────────────────────────┴───────────────────────────────────┤
│ Active pane: api-prod · ops       3 panes       2 sessions       REC off │
└──────────────────────────────────────────────────────────────────────────┘
```

The active pane uses an unmistakable focus boundary in the visual design.
Keyboard navigation and resize operate on the same tree shown here.

## 7. File manager

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Remote files · api-staging · deploy · STAGING                            │
├──────────────────────────────────────────────────────────────────────────┤
│ [←] [→] [↑]  /var/www/app/releases/2026-07-29  [Refresh] [Upload] [More]│
├────────────────────────────────────────────┬─────────────────────────────┤
│ Name                         Size   Mode   │ SELECTED                    │
│ 📁 ..                                      │ config.toml                 │
│ 📁 bin/                       —     0755   │ Regular UTF-8 text          │
│ 📁 public/                    —     0755   │ 12.4 KiB · 0640             │
│ 📄 config.toml             12 KiB   0640   │ Owner deploy:app            │
│ 📄 relio-api              42 MiB   0755   │ Modified 14:12              │
│ ↗ current → releases/…       —            │                             │
│                                            │ [ Open as text ]            │
│                                            │ [ Download ] [ More ▼ ]     │
├────────────────────────────────────────────┴─────────────────────────────┤
│ Separate authenticated SFTP connection · 1 transfer active              │
└──────────────────────────────────────────────────────────────────────────┘
```

Non-round-trippable names use a visible warning and core-owned exact-path
handling. The UI never asks the user to reconstruct a shell-escaped path.

### Transfer review

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Review upload                                                            │
├──────────────────────────────────────────────────────────────────────────┤
│ From       Local selection · 3 files · 18.2 MiB                          │
│ To         api-staging · /var/www/app/releases/2026-07-29/               │
│ Identity   deploy · STAGING · via bastion.example                        │
│ Semantics  SFTP                                                          │
│ Conflicts  1 existing file                                               │
│                                                                          │
│ config.toml   [ Replace ▼ ]   Atomic replacement available               │
│ app.tar.gz    [ Upload ]                                                  │
│ notes.txt     [ Upload ]                                                  │
├──────────────────────────────────────────────────────────────────────────┤
│ [ Cancel ]                                              [ Start upload ] │
└──────────────────────────────────────────────────────────────────────────┘
```

## 8. Remote editor

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ config.toml ●  · api-staging · /var/www/app/config.toml                  │
├──────────────────────────────────────────────────────────────────────────┤
│ [Save]  UTF-8  LF  0640  deploy:app       Version checked 14:12 [•••]   │
├──────┬───────────────────────────────────────────────────────────────────┤
│   1  │ [server]                                                          │
│   2  │ port = 8080                                                       │
│   3  │ workers = 8                                                       │
│   4  │                                                                   │
│   5  │ [logging]                                                         │
│   6  │ level = "info"                                                    │
│      │                                                                   │
│      │                                                                   │
├──────┴───────────────────────────────────────────────────────────────────┤
│ api-staging · STAGING │ Modified in memory │ No local recovery copy      │
└──────────────────────────────────────────────────────────────────────────┘
```

### Conflict state

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Remote file changed                                                      │
├──────────────────────────────────────────────────────────────────────────┤
│ api-staging · /var/www/app/config.toml                                   │
│                                                                          │
│ The remote file changed after this editor loaded it. Your unsaved         │
│ changes remain in memory. Relio will not overwrite the new version.       │
│                                                                          │
│ Loaded version      14:12 · 12.4 KiB · id ABCD                           │
│ Current remote      14:21 · 12.7 KiB · id EFGH                           │
│                                                                          │
│ [ Compare ] [ Save as another path ] [ Discard mine ] [ Keep editing ]  │
└──────────────────────────────────────────────────────────────────────────┘
```

## 9. Settings

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Settings                                             [Search settings]   │
├────┬──────────────────────┬───────────────────────────────────────────────┤
│ ◫  │ Appearance           │ Appearance                                    │
│    │ Terminal             │ Scope: User default ▼                         │
│ ▣  │ Keyboard             │                                               │
│    │ Connections          │ THEME                                         │
│ ≣  │ Credentials          │ Current: Midnight Slate       [Choose] [Edit] │
│    │ Files and transfers  │ Workspace API development overrides this      │
│ ⚙  │ History & recording  │                                               │
│    │ Privacy & data       │ DENSITY                                       │
│    │ Advanced             │ (x) Comfortable   ( ) Compact                 │
│    │ About & diagnostics  │                                               │
│    │                      │ UI FONT                                       │
│    │                      │ System UI ▼     Effective: SF Pro / Segoe UI  │
│    │                      │                                               │
│    │                      │ MOTION                                        │
│    │                      │ Follow system ▼                               │
│    │                      │                                               │
│    │                      │ [Reset Appearance at User scope]              │
└────┴──────────────────────┴───────────────────────────────────────────────┘
```

Every setting row can expose effective source, inheritance, safety constraint,
and reconnect/restart effect.

### Credentials settings

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Credentials                                              [ + Add ]       │
├──────────────────────────────────────────────────────────────────────────┤
│ OS protected store: Ready · Last checked now                             │
│                                                                          │
│ Agent · ED25519 SHA256:ABCD…      8 hosts      Ready        [Manage]     │
│ Key file · ~/.ssh/prod_ed25519    3 hosts      ! Permissions [Repair]    │
│ Stored password · Legacy lab      1 host       Ready        [Manage]     │
│                                                                          │
│ Secret values are not displayed. External key files stay in place.       │
└──────────────────────────────────────────────────────────────────────────┘
```

## 10. Theme editor

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Midnight Slate · local draft                 Saved   [Reset] [Apply]     │
├──────────────────┬────────────────────────────┬───────────────────────────┤
│ TOKEN GROUPS     │ FOUNDATIONS                │ LIVE PREVIEW              │
│                  │                            │                           │
│ Foundations   ●  │ Canvas        [■ #0B0F14]  │ ┌───┬──────┬───────────┐ │
│ Text             │ Surface       [■ #111821]  │ │ ◫ │ Hosts│ Terminal  │ │
│ Accent           │ Raised        [■ #17212C]  │ │ ▣ │ api  │ $ echo hi│ │
│ Status           │ Border        [■ #2B3A4A]  │ │ ⚙ │ prod │           │ │
│ Environments     │                            │ └───┴──────┴───────────┘ │
│ Terminal         │ Validation                 │                           │
│ Typography       │ ✓ Required contrast        │ Buttons / form / table    │
│ Shape & density  │ ! Muted text: 4.3:1        │ ANSI / cursor / selection │
│ Motion           │   [Suggest accessible]     │                           │
│                  │                            │ ◆ RELIO SECURITY CHECK    │
│                  │                            │ Protected style [Locked]  │
├──────────────────┴────────────────────────────┴───────────────────────────┤
│ Theme data is local. Scripts, CSS, assets, paths, and URLs are not allowed.│
└──────────────────────────────────────────────────────────────────────────┘
```

Apply remains disabled for hard failures. The protected preview demonstrates
the boundary rather than pretending trusted tokens are theme properties.

## 11. Command palette

```text
              ┌──────────────────────────────────────────────────────┐
              │ > split right                                       │
              ├──────────────────────────────────────────────────────┤
              │ ACTIONS                                              │
              │  > Terminal: Split right             Ctrl+Shift+\    │
              │    Create a pane beside api-staging                  │
              │                                                      │
              │  > Terminal: Split down              Ctrl+Shift+-    │
              │                                                      │
              │ OPEN SESSIONS                                        │
              │  / api-staging · deploy · Connected                  │
              │                                                      │
              │ SETTINGS                                             │
              │  > Keyboard: Change “Split right”                    │
              ├──────────────────────────────────────────────────────┤
              │ Active: API development / api-staging / deploy       │
              └──────────────────────────────────────────────────────┘
```

### Target selection step

```text
              ┌──────────────────────────────────────────────────────┐
              │ Connect host  ›  Choose destination                  │
              ├──────────────────────────────────────────────────────┤
              │ @ api-staging       deploy · STAGING · Ready         │
              │ @ worker-staging    admin  · STAGING · Ready         │
              │ @ api-prod          ops    · PROD    · Key review    │
              ├──────────────────────────────────────────────────────┤
              │ Esc Back            Enter Select                     │
              └──────────────────────────────────────────────────────┘
```

Palette selection may begin a connection but cannot approve fingerprint,
credential, destructive, overwrite, or broad-bind decisions.

## Compact-width adaptation

At compact width:

```text
┌────┬───────────────────────────────────────────────┐
│Rail│ Active editor surface                         │
│    │                                               │
│    │                                               │
├────┴───────────────────────────────────────────────┤
│ Status                                              │
└────────────────────────────────────────────────────┘

[Sidebar] opens as a focus-managed overlay.
[Inspector] opens as a temporary side sheet.
[Operations] opens as a bottom sheet/maximized panel.
```

The active remote host/environment remains in the top bar or pane header.
Trusted dialogs stack evidence vertically and scroll their body; their title,
target, Cancel, and decision actions remain reachable.

## Cross-wireframe state checklist

Every applicable screen must have designed variants for:

- initial loading and delayed loading;
- empty and filter-empty;
- unsupported capability;
- disconnected/unavailable reference;
- inline validation;
- cancellable progress;
- success;
- failure with safe detail;
- keyboard focus;
- light, dark, high contrast, and custom theme;
- compact and 200% text-scaled layout;
- hostile, long, and non-Latin labels.
