# ADR-011: Minimal xterm.js 6 Renderer Dependency Set

- **Status:** Accepted
- **Date:** 2026-07-29
- **Owner:** Terminal UI maintainers (`@owner` until replaced)

## Context

Relio needs a mature browser terminal emulator with VT parsing, Unicode and IME
behavior, selection, accessibility, and sustained-output performance. Building
that parser and renderer is outside the product's differentiating scope and
would create a large security and compatibility burden.

xterm.js is already the accepted renderer direction. Before implementation,
the exact packages and addon attack surface must be fixed. Addons can observe
terminal data, create browser actions, register parser hooks, or add GPU
resource behavior, so installing the whole addon ecosystem is unacceptable.

## Decision

Pin:

- `@xterm/xterm 6.0.0`;
- `@xterm/addon-fit 0.11.0`.

Use the built-in DOM renderer first. The project-owned model adapter owns
terminal construction, output writes, bounded scrollback, disposal, detached
state, title policy, URI recognition, clipboard decisions, input sequencing,
resize coalescing, and backend stream credit.

The fit addon only calculates rows and columns for an already trusted container.
It receives no network, filesystem, clipboard, or process capability.

Do not install v1 addons for attach, clipboard, web links, images, serialization,
search, ligatures, progress, Unicode graphemes, web fonts, or WebGL in this
milestone:

- transport is Relio's typed bounded IPC stream, not WebSocket attach;
- clipboard and URI actions require project-owned consent policy;
- image/file protocols remain disabled;
- serialization could turn sensitive scrollback into a new export path;
- Unicode grapheme support is currently marked experimental upstream;
- WebGL adds GPU/context-loss lifecycle and is adopted only if profiling shows
  the DOM renderer cannot meet the budget.

## Dependency review

- **Capability:** standards-compatible terminal emulation and container fitting.
- **Why not platform primitives:** webviews provide no terminal emulator.
- **Maintenance:** packages are maintained by the xterm.js project and widely
  deployed in developer tools.
- **License:** MIT; both packages have no runtime dependencies at the pinned
  versions.
- **Scripts/native code/network:** no native module or runtime network service;
  lockfile/install-script policy remains enforced.
- **Impact:** renderer code enters the webview bundle only on the terminal
  feature path; startup and empty workbench remain network-free.
- **Trust boundary:** all output is hostile bytes. The renderer may alter only
  its terminal model and DOM subtree; it cannot invoke privileged core actions.
- **Test seam:** `TerminalModel` wraps xterm and accepts a fake backend stream;
  policy functions for title, URI, clipboard, and dimensions are pure.
- **Replacement:** preserve the renderer-neutral backend stream and replace the
  model adapter after compatibility, accessibility, and performance parity.

## Alternatives

- **Project-owned emulator:** rejected as a specialized multi-year security and
  compatibility effort.
- **Native terminal widget:** rejected because it would fragment the React
  workbench and Tier 1 behavior.
- **Headless model plus custom renderer:** rejected because it retains parser
  dependency cost while recreating rendering and accessibility.
- **All maintained addons:** rejected because maintained does not imply needed
  or authorized.

## Consequences

Terminal bytes never enter React state. One model instance can outlive DOM
mounts, while the view adapter owns fit observation and event listeners.
Feature code must explicitly dispose model and view resources.

DOM rendering is the safe baseline, not a permanent performance assumption.
WebGL adoption requires a separate dependency review, context-loss fallback,
GPU memory evidence, Tier 1 rendering tests, and CSP review.

## Security, privacy, and accessibility

OSC 52 clipboard operations, automatic URI opening, remote title promotion,
images, notifications, and file protocols are blocked or reduced to inert
display facts. Copy requires a user selection and explicit action; paste
requires an explicit user gesture and inserts bytes only into the active
terminal. External URIs are normalized against a strict scheme policy and
shown for confirmation outside terminal-controlled chrome.

Screen-reader mode, semantic labeling, minimum contrast, reduced motion,
Unicode, IME composition, and keyboard escape paths are required. Remote text
is never assigned as HTML.

## Compatibility and migration

The terminal model is runtime-only. No xterm data structure is persisted or
crosses IPC, so replacement requires no data migration.

## Reversal cost

Medium. VT behavior and accessibility have broad compatibility fixtures, but
the backend and product policy are renderer-neutral.

## Review triggers

- an xterm.js security advisory or maintenance change;
- an addon proposal;
- parser, clipboard, link, image, notification, or title policy changes;
- DOM renderer failure against measured Tier 1 budgets;
- experimental API adoption or a major-version upgrade.
