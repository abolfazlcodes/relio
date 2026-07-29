# Theme System

## Goal and boundary

Relio v1 provides a built-in theme engine for a modern, customizable workspace.
Themes are local appearance settings created through Relio's UI. They are
validated data, never executable code, CSS, HTML, or a distribution package.

V1 includes bundled presets and user-created local themes. It has no dynamic
code loading, remote theme source, or installable theme package format.

## Token resolution

```text
platform/accessibility constraints
  constrain
base design tokens
  -> light/dark/high-contrast appearance
  -> user theme
  -> optional workspace selection
  -> component state mapping
```

Platform and accessibility constraints are safety constraints, not normal
overrides. Reduced motion, forced colors, minimum contrast, text scaling, and
trusted safety chrome can replace theme values.

## Token groups

V1 supports bounded values for:

- semantic UI colors;
- terminal foreground/background, cursor, selection, and ANSI palette;
- typography from a core or user-approved installed-font list;
- font sizes, weights, and line heights;
- spacing, radii, borders, and shadows within allowed ranges;
- motion durations and easing within accessibility bounds;
- opacity and transparency where the platform permits them.

Components consume semantic roles such as:

- background, elevated surface, and border;
- primary, secondary, muted, and disabled text;
- accent and focus indicator;
- success, warning, danger, and information;
- selected, hovered, pressed, and disabled control;
- active and inactive session;
- local, development, staging, and production environment;
- terminal foreground, background, cursor, selection, and ANSI colors.

Environment and security state never depend on color alone.

## Storage and schema

A user theme is a versioned record in the encrypted local profile:

- stable local ID and user-visible name;
- base appearance;
- token overrides;
- schema version;
- last validation result.

Theme selection may be global or workspace-scoped. Workspace export records
only the selected local theme name after preview, not a portable theme
definition. V1 has no theme import format.

Rules:

- unknown required tokens fail validation;
- every resolved theme is complete and immutable before activation;
- missing or invalid values do not partially apply;
- token values have type, length, numeric, and syntax bounds;
- no arbitrary file path, remote URL, script, stylesheet, active image, or font
  file is accepted.

## Validation

The theme service validates:

- schema version and resolved token completeness;
- token syntax and range;
- WCAG 2.2 AA contrast for required UI text and focus states;
- distinguishability of critical states in monochrome/forced-color modes;
- motion and transparency bounds;
- terminal palette usability, including cursor and selection.

A warning may allow a personal non-critical contrast preference, but trusted
security chrome, focus indicators, and required accessibility modes always use
safe values. A theme cannot suppress or restyle them below the baseline.

## Activation and fallback

1. Validate the draft off-screen.
2. Resolve it into a complete token set.
3. Render a bounded core preview.
4. Apply atomically.
5. Record the selection only after successful paint.
6. Retain the previous known-good resolved theme.

On startup failure, use the last known-good theme; if that fails, use the
bundled default. Theme failure never blocks settings or terminal access.

## Testing

- schema fixtures for every supported version;
- oversized, malformed, unknown-token, and invalid-value tests;
- snapshot/visual regression across light, dark, high contrast, reduced motion,
  and text scaling;
- automated contrast plus manual keyboard/focus review;
- terminal palette fixtures;
- trusted safety-chrome invariance;
- failed activation, startup fallback, reset, and deletion behavior;
- cross-platform rendering in every Tier 1 webview.
