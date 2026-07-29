# Platform Support

## Support policy

“Cross-platform” means one product model with tested platform adapters. It does
not mean identical OS behavior or support for every distribution.

The exact stable-release matrix is published with each release. The provisional
implementation matrix is:

| Tier | Platform | Architecture | Scope |
| --- | --- | --- | --- |
| 1 | Windows 11 | x86_64 | Full CI build; PTY, keychain, SSH, packaging, update, and E2E release gates |
| 1 | macOS currently receiving Apple security updates | arm64 | Full CI build; PTY, Keychain, SSH, signing/notarization, update, and E2E release gates |
| 1 | Ubuntu 24.04 LTS desktop | x86_64 | GNOME on Wayland and X11 smoke coverage; Secret Service, PTY, SSH, package/update policy |
| 2 | macOS currently receiving Apple security updates | x86_64 | Build and smoke test while toolchain and hardware capacity remain available |
| 2 | Windows 11 | arm64 | Build and smoke test after dependencies prove native support |
| 2 | Current Fedora Workstation | x86_64 | Community-supported compatibility and smoke tests |

Tier 1 failures block release. Tier 2 failures are documented and may block a
feature when data loss or a security boundary is involved. Other Linux
distributions are best effort until added by an ADR with maintainers and CI.

Minimum versions must be validated against Tauri, webview, encrypted SQLite,
OpenSSH, signing, and installer requirements before Phase 1 is
declared complete. This table does not override an upstream end-of-life policy.

## Adapter boundaries

| Capability | Windows | macOS | Linux |
| --- | --- | --- | --- |
| Local terminal | ConPTY | POSIX PTY | POSIX PTY |
| Process tree stop | Job Object or equivalent supervised tree | process group/session | process group/session |
| Secret storage | Windows credential/protection APIs | Keychain Services | Secret Service-compatible provider |
| Webview | WebView2 | WKWebView | WebKitGTK |
| SSH | supported system OpenSSH client or diagnosed unavailable | system OpenSSH within supported matrix | supported OpenSSH package within distro matrix |
| SFTP/SCP | supported OpenSSH tools with diagnosed capabilities | supported OpenSSH tools with diagnosed capabilities | supported OpenSSH tools with diagnosed capabilities |
| Signing | Authenticode | Developer ID and notarization | package/repository signature as distribution permits |
| Update | direct installer/updater policy | signed/notarized app updater | direct-download artifacts only; distro packages use distro updates |

Each adapter exposes availability and version information. The UI shows a
capability diagnosis instead of failing later with a generic error.

## Terminal and shell behavior

- Discover the user's default shell through platform APIs/configuration and
  provide an explicit profile override.
- Do not assume a POSIX shell on Windows or PowerShell semantics on Unix.
- Represent process arguments structurally; never use one shell command string
  as the cross-platform abstraction.
- Resize, signal, close, and process-tree termination have platform-specific
  implementations and contract tests.
- Working-directory restoration validates existence, permissions, and path
  representation.
- Test UTF-8, non-UTF-8 filesystem boundaries where applicable, IME, dead keys,
  AltGr, compose keys, CJK input, emoji width, and right-to-left edge cases.

## Filesystem behavior

- Preserve the platform's path type in the core. Convert to display strings
  lossily only at the UI boundary and mark non-round-trippable display.
- Test case sensitivity, reserved names, path length, drive/UNC paths, symlinks,
  permissions, executable bits, and atomic rename assumptions.
- Never assume owner-only POSIX modes are meaningful on Windows; use the native
  ACL adapter.
- Network shares and removable drives may not support local locking or atomic
  operations and are not valid profile database locations in v1.
- Temporary sensitive files use platform-protected runtime directories and
  restrictive native ACLs.

## Webview and UI behavior

- Bundle all application code and required fonts/assets.
- Maintain a restrictive CSP on every platform.
- Test clipboard permissions, drag/drop, context menus, file pickers, window
  chrome, accessibility APIs, display scaling, color management, and reduced
  motion.
- Do not depend on a web API until it is verified in all Tier 1 webviews or has
  an adapter/fallback.
- Record the webview version in diagnostics because behavior and security
  patches are supplied by the OS/runtime.

## Credential-store degradation

Linux desktop sessions may have no usable Secret Service, and enterprise
systems may deny keychain access on any platform. In that state:

- persistent profiles remain locked;
- Relio never falls back to plaintext;
- an explicitly selected temporary local-terminal mode may run without
  persistence;
- remote credentials may use an existing external agent if no secret bytes need
  to enter Relio;
- diagnostics explain which platform facility is missing.

## Packaging and updates

- Build artifacts separately for each OS and architecture; no artifact is
  relabeled across targets.
- OS signatures and Relio updater signatures are separate required controls for
  direct downloads.
- Linux distribution packages follow the distribution's update mechanism.
  Relio must not create a competing self-update path inside a repository-managed
  package.
- Installation, upgrade, rollback, and uninstall tests verify data-directory
  retention and clear disclosure.

## Feature support

A feature is:

- **supported** when it passes Tier 1 automated and required manual tests;
- **experimental** when the UI labels limitations and it is excluded from
  stable compatibility promises;
- **unavailable** when capability diagnosis blocks it with remediation;
- never silently emulated with weaker security.

Platform exceptions belong in the feature document and release notes, not only
inside conditional code.
