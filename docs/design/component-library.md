# Component Library Implementation Contract

## Purpose

This document maps the normative [design system](design-system.md) to the
Milestone 05 React component baseline. It defines usage boundaries; feature
composition and visual customization remain in their owning milestones.

## Ownership and imports

Reusable components live under `apps/desktop/src/ui`. Feature views import from
the directory index and must not duplicate controls to obtain different colors,
spacing, focus behavior, or keyboard semantics.

Components consume semantic variables from `tokens.css`. Ordinary feature CSS
must not use palette literals. The bundled dark and light appearances, forced
colors, reduced motion, spacing, typography, shape, and focus roles are defined
there. The future theme engine may provide validated ordinary semantic values;
it may not write trusted-confirmation variables.

## Component contracts

| Component | Intended use | Required behavior |
| --- | --- | --- |
| `Button` | Named user action | Explicit variant, native button semantics, disabled duplicate activation while loading |
| `TextField` | Ordinary non-secret text | Persistent label, linked description and error, retained valid value |
| `Tabs` | Sibling views | Roving tab stop, arrow/Home/End navigation, linked tab panel |
| `Dialog` | Interrupting application-owned decision | Native modal semantics, Escape cancellation, title focus, caller-owned action labels |
| `StatusMessage` | Inline status or failure | Icon and text, appropriate live-region role, no color-only meaning |
| `TrustedConfirmation` | Core-issued material security decision | Reserved invariant frame, exact target/evidence, safe Cancel, explicit action label, original challenge returned unchanged |

Icon-only buttons are not part of the baseline. When introduced, they require
an accessible name and visible tooltip. Secret fields require a separately
reviewed protected-input component and must not reuse `TextField`.

## Trusted confirmation boundary

Only an application controller handling a live core-issued
`ConfirmationChallenge` may render `TrustedConfirmation`. Terminal output,
remote files, imported text, plugin views, theme data, and ordinary
notifications cannot request an approval result or render within the reserved
surface. Display strings are React text nodes with automatic escaping and use
`dir="auto"` where evidence can contain bidirectional or non-Latin text.

The component is not an authorization boundary. Rust validates the nonce,
displayed digest, operation state, expiry, policy, and one-time consumption.
Closing or pressing Escape is equivalent to Cancel; Enter never defaults to
the consequential action.

## Keyboard and focus conventions

- Native controls remain in document order; positive `tabindex` is prohibited.
- `:focus-visible` uses the semantic focus role with two-pixel separation.
- Background completion does not move focus.
- Tabs use one tab stop and arrow navigation.
- A modal initially focuses its heading or first safe field, never a
  destructive or trust action.
- On modal close, the controller restores focus to the invoking control.
- Feature shortcuts must not intercept required dialog cancellation or
  navigation.

## Accessibility and visual fixtures

`DesignSystemFixture` is the deterministic visual-review surface. It includes
ordinary controls, statuses, trusted chrome, long text, bidirectional controls,
 non-Latin text, and markup-shaped hostile strings. Review it in:

- bundled dark and light appearances;
- forced-colors/high-contrast mode;
- reduced motion;
- 200% text scaling at the 720 × 480 minimum content area;
- Windows, macOS, and Linux system webviews.

Automated tests cover labels/descriptions, status semantics, loading behavior,
tab keyboard behavior, escaped hostile evidence, challenge identity, and
baseline WCAG AA contrast pairs. Platform screenshot baselines are recorded by
Tier 1 CI when the native visual harness is available; reviewers must still
perform keyboard, screen-reader, truncation, and spoofing checks.

## Extension checklist

Before adding or changing a shared component:

- document states, keyboard behavior, accessible naming, and focus restoration;
- use semantic tokens and test both bundled appearances;
- include loading, disabled, unavailable, invalid, and error states where
  relevant;
- add long, localized, bidirectional, and hostile text fixtures;
- keep infrastructure side effects in feature controllers;
- require a security review for trusted chrome, secret input, external
  navigation, rich text, or rendering of remote data.
