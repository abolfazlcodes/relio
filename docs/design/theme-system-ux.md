# Theme System UX

## Product intent

Themes let users make an all-day technical workspace comfortable and
recognizable without creating a new code, asset, or trust-loading path. A theme
is a local versioned record of bounded semantic values.

The theme experience must feel expressive within visible safety rails. Users
should understand which values are theirs and which are protected by Relio.

## Customizable properties

### Workspace colors

- canvas, surface, raised surface, and borders;
- primary, secondary, muted, and disabled text;
- accent, selection, hover, pressed, and inactive states;
- success, warning, danger, and information roles;
- environment roles for local, development, staging, and production;
- active and inactive session treatments.

### Terminal

- foreground and background;
- cursor color and shape from supported values;
- selection color;
- ANSI 16-color palette;
- approved installed font;
- font size, weight, line height, and optional ligature preference.

### Shape and density

- comfortable or compact base density;
- spacing multiplier within 0.85–1.20 of the baseline;
- control/card radius within 0–12 px;
- border emphasis within supported 1–2 px roles;
- bounded shadow/elevation strength;
- supported transparency where the platform and contrast permit it.

### Motion

- reduced, subtle, or standard motion preset;
- bounded transition duration and easing.

OS reduced-motion and forced-color settings always constrain or replace theme
motion/color choices.

## Properties themes cannot customize

Themes cannot change:

- trusted safety frame, shield mark, or `Relio security check` label;
- security-dialog wording, hierarchy, button order, minimum size, or focus
  behavior;
- verified, changed, revoked, secure-input, broad-bind, and destructive
  security semantics;
- minimum focus-ring visibility;
- control hit-area and readable-text minimums;
- content sanitization or URI policy;
- layout structure, navigation destinations, command availability, or
  permissions;
- recording label/icon semantics;
- environment text labels in material operation reviews;
- application icons, logos, arbitrary images, or animated assets;
- raw CSS, HTML, JavaScript, shaders, shell expressions, or template code;
- local/remote URLs, file paths, font files, or network resources;
- remote content or separately loaded application surfaces.

The trusted tokens are shown in the editor as `Protected by Relio` and are not
editable.

## Safe value boundaries

| Property | Boundary |
| --- | --- |
| UI font size | 12–20 px nominal, with OS scaling applied afterward |
| Terminal font size | 9–32 px |
| Line height | 1.1–1.8 |
| Spacing multiplier | 0.85–1.20 |
| Radius | 0–12 px |
| Border role | 1–2 px equivalent |
| Ordinary motion duration | 0–300 ms; OS reduced motion may force 0 |
| Content opacity | Must preserve required contrast; trusted surfaces ignore it |

Exact parser bounds are schema-owned. The UX presents constrained controls, not
free-form text for structured numeric or color values.

## Theme library

Settings > Appearance opens with:

1. current theme and scope;
2. bundled themes;
3. user-created local themes;
4. `Create theme` and `Duplicate` actions;
5. density, UI font, and motion preferences.

Bundled themes cannot be edited or deleted. `Duplicate` creates an independent
local draft. A deleted local theme falls back to the previous known-good
selection or bundled default; workspace references are previewed before delete.

## Theme editor

### Layout

```text
┌──────────────────────────────────────────────────────────────────────────┐
│ Theme: Midnight Slate                     Draft saved  [Reset] [Apply]  │
├───────────────────┬──────────────────────────┬───────────────────────────┤
│ TOKEN GROUPS      │ STRUCTURED CONTROLS      │ LIVE PREVIEW              │
│                   │                          │                           │
│ Foundations       │ Canvas       [color]     │ Workbench                 │
│ Text              │ Surface      [color]     │ Terminal and ANSI         │
│ Accent            │ Primary text [color]     │ Forms and table           │
│ Status            │ ...                      │ Empty and error states    │
│ Environments      │                          │ Focus and selection       │
│ Terminal          │ Validation               │                           │
│ Typography        │ ✓ Required contrast      │ SECURITY CHECK            │
│ Shape & density   │ ! Muted text is low      │ [Protected by Relio]      │
│ Motion            │                          │                           │
└───────────────────┴──────────────────────────┴───────────────────────────┘
```

On narrow windows, Token groups, Edit, and Preview become three keyboard
reachable tabs. The preview never becomes the only indication of validation
failure.

### Control patterns

- Color control: swatch, hex value, accessible name, contrast relationship,
  and reset.
- Typography: approved installed-font combobox with fallback diagnosis.
- Numeric values: slider for exploration plus exact bounded field.
- ANSI palette: named grid with foreground/background samples and common
  terminal fixtures.
- Density/motion: safe preset first, bounded advanced values second.

The editor never accepts pasted stylesheet or unstructured theme JSON.

## Preview content

The bounded preview contains only synthetic bundled fixtures:

- rail, sidebar, tabs, and status bar;
- active/inactive local, development, staging, and production contexts;
- button, input, dropdown, table, tooltip, and notification states;
- terminal text, ANSI palette, cursor, selection, bold, and Unicode samples;
- file browser and remote editor;
- success, warning, error, disabled, and focus states;
- trusted fingerprint review shown inside an invariant frame.

The trusted preview explicitly says that its style is protected and does not
sample user-provided names or remote content.

## Validation model

### Hard failures

Apply is blocked when:

- a required token is missing or malformed;
- required UI text, focus, or selected states fail WCAG 2.2 AA;
- trusted safety chrome or forced-color distinguishability would be weakened;
- terminal foreground/background, cursor, or selection becomes unusable;
- a value exceeds schema bounds;
- the theme references a missing/unapproved font or any path/URL/content;
- the schema version is unknown.

### Warnings

The user may apply after acknowledgement when a non-critical personal
preference has reduced but still usable contrast, dense spacing near the lower
bound, or an ANSI pair likely to be difficult to distinguish. Warnings name the
affected samples and offer automatic repair.

Warnings can never waive trusted UI, focus, required text, forced-color, or
minimum-size failures.

### Automatic repair

`Suggest accessible value` makes the smallest bounded adjustment and shows the
before/after token. `Repair all` creates a reviewable set of changes; it does
not silently overwrite the draft.

## Draft, save, preview, and apply

1. Changes update a local draft and bounded preview.
2. Draft state may persist in the encrypted profile but is never active
   appearance until validated.
3. `Apply` validates the complete resolved token set off-screen.
4. Relio paints the complete theme atomically.
5. Selection is recorded only after successful paint.
6. The previous known-good theme remains available.

`Cancel editing` leaves the last applied theme active and offers to discard or
retain the local draft. Theme failure never blocks terminal or settings access.

## Scope

Theme selection supports:

- User default;
- Current workspace override.

The scope control shows `Inherited from User` or `Set for this workspace`.
Resetting a workspace theme reveals the current user theme. Host- or
session-specific full UI themes are not part of v1; terminal settings may still
resolve at supported narrower scopes through the settings system.

## Import rules

Relio v1 does not import theme definitions, packages, CSS, JSON, URLs, or
clipboard payloads. This is an explicit product and security boundary, not a
disabled control waiting for a service.

- There is no `Import theme` action.
- Opening or dropping a theme-like file does not parse or activate it.
- Workspace export cannot activate a theme definition.
- A same-profile encrypted recovery backup may restore theme records only
  through the authenticated profile recovery flow.

## Export rules

Relio v1 does not provide a portable theme-definition export because there is
no matching safe ingestion contract.

- A redacted settings/workspace export may record the selected local theme name
  and base appearance after preview.
- It does not contain an active theme package, secret handles, font files,
  paths, assets, or executable content.
- Screenshots are ordinary user-controlled OS output, not a Relio theme format.
- Future portable interchange requires a versioned schema and separate design,
  security, migration, and hostile-input review.

## Reset, delete, and fallback

- `Reset token` returns one token to its inherited/base value.
- `Reset group` previews affected tokens before applying.
- `Reset theme` returns the draft to its bundled base.
- `Delete theme` previews workspace selections that will fall back.
- Startup uses the last known-good resolved theme, then the bundled default if
  necessary.
- A persistent `Start with default appearance` recovery action is available
  when theme painting fails.

## Accessibility behavior

- Color controls expose numeric values and contrast results to screen readers.
- Validation is text and icon based, not color-only.
- Preview regions have named landmarks and logical tab order.
- Theme editing itself respects current OS high contrast and reduced motion.
- A user theme cannot opt out of forced colors, text scaling, or focus
  visibility.

## Security test scenarios

The UX specification must be verified with themes that attempt to:

- match remote content to dialog surfaces;
- hide borders, focus, recording, or environment labels;
- set transparent foreground/background pairs;
- make selection/cursor indistinguishable;
- use oversized values, invalid colors, paths, and URLs;
- select missing fonts;
- imitate danger/trust tokens;
- fail during preview, activation, and startup.

The expected result is validation failure or substitution by protected
platform/safety values, with the prior known-good theme preserved.
