# Theme System

## Goal

Make the workspace deeply customizable without forcing every component or plugin to understand the internal CSS structure. Themes describe semantic roles and design tokens.

## Token layers

```text
platform defaults
  -> base design tokens
  -> light/dark appearance mapping
  -> user theme
  -> workspace override
  -> component state tokens
```

Terminal colors, UI colors, typography, spacing, radii, borders, shadows, motion, transparency, and icon styles should be represented as tokens where the platform supports them.

## Semantic roles

Themes should name roles such as:

- background and elevated surface;
- primary, secondary, and muted text;
- accent and focus ring;
- success, warning, danger, and information;
- active and inactive session;
- local, development, staging, and production environment;
- terminal foreground, background, cursor, selection, and ANSI palette.

Components consume roles. A theme may map roles to colors, gradients, or platform effects without knowing component class names.

## Validation

The theme loader validates schema version, token types, required roles, contrast, motion bounds, and resource limits. Invalid themes fall back to the last valid theme and provide a diagnostic. A theme cannot inject arbitrary script or stylesheet code.

## Distribution

Themes are local packages first. A future marketplace may distribute them, but users can create and share a theme package without an account. Theme metadata includes author, license, preview, compatibility, and inherited base theme.

## Accessibility and safety

- theme validation warns about insufficient contrast;
- reduced motion can override theme motion tokens;
- transparency is disabled or reduced when the operating system requests it;
- the theme never communicates security state through color alone;
- production environment roles must remain distinguishable in monochrome or high-contrast modes.
