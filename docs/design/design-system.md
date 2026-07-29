# Relio Design System

## Design-system goals

The Relio design system must support dense technical work without visual noise,
remain accessible across Tier 1 platform webviews, and prevent themes or remote
content from weakening trusted safety UI.

Components consume semantic roles rather than raw palette values. Measurements
below are design targets and must be verified with platform text rendering,
zoom, high contrast, and assistive technology before implementation.

## Foundations

### Base unit

The layout grid uses a 4 px base unit with an 8 px default rhythm. Two-pixel
steps exist only for compact optical alignment, borders, and small icon gaps.
Spacing is not reduced below interactive or readability minimums by a compact
theme.

### Density modes

| Mode | Intended use | Row target | Control target |
| --- | --- | --- | --- |
| Comfortable | Default, mixed input methods | 36–40 px | At least 32 px visual control, 40 px effective hit area where space allows |
| Compact | High-density inventory and sessions | 28–32 px | At least 28 px visual control, 36 px effective hit area |

Trusted dialogs and touch-oriented platform behavior ignore compact reductions
that would make controls difficult to read or activate.

## Typography

### Font families

| Role | Default | Rules |
| --- | --- | --- |
| UI sans | Platform system UI font | Must render all interface labels reliably; user selection comes from a diagnosed installed-font list |
| UI mono | Platform system monospace | Fingerprints, ports, paths, short commands, and structured technical values |
| Terminal | Diagnosed platform monospace | User-selectable from an approved installed-font list; fallback is explicit when unavailable |

Theme records may name only a core or approved installed font. They cannot
contain font files, file paths, URLs, or font-feature code.

### Type scale

| Token | Nominal size / line height | Use |
| --- | --- | --- |
| `display` | 28 / 36 | Welcome heading only |
| `title-1` | 22 / 30 | Page and major dialog title |
| `title-2` | 18 / 26 | Section and panel title |
| `body` | 14 / 21 | Default interface copy |
| `body-strong` | 14 / 21, semibold | Labels and emphasized facts |
| `small` | 12 / 18 | Metadata and compact tables |
| `caption` | 11 / 16 | Timestamps and secondary status; never required decision text |
| `mono` | 13 / 20 | Paths, fingerprints, ports, commands |

UI text should not use all caps except short group labels whose accessible name
uses normal casing. Letter spacing and font weight remain within legibility
bounds. Under OS text scaling, layouts reflow instead of clipping labels.

### Terminal typography

- Default nominal size: 13–14 px according to platform rendering.
- User range: 9–32 px.
- Line-height range: 1.1–1.8.
- Font weight range: regular through semibold when the chosen font supports it.
- Bold text must remain distinguishable without relying on color brightening.
- Ligatures are an explicit terminal preference because they can change the
  appearance of commands and diagnostics.

## Spacing scale

| Token | Value | Typical use |
| --- | --- | --- |
| `space-0` | 0 | Intentional edge joining |
| `space-0.5` | 2 px | Optical adjustment |
| `space-1` | 4 px | Icon/text micro-gap |
| `space-1.5` | 6 px | Compact internal gap |
| `space-2` | 8 px | Standard internal gap |
| `space-3` | 12 px | Control padding |
| `space-4` | 16 px | Card or panel padding |
| `space-5` | 20 px | Related section separation |
| `space-6` | 24 px | Page section separation |
| `space-8` | 32 px | Major group separation |
| `space-10` | 40 px | Empty-state rhythm |
| `space-12` | 48 px | Welcome composition |
| `space-16` | 64 px | Large-screen outer breathing room |

Theme density may apply a bounded multiplier to ordinary component spacing but
not to trusted safety chrome, focus separation, dialog action spacing, or
minimum hit regions.

## Grid and sizing

### Window grid

- Activity rail: 44–52 px.
- Primary sidebar: 240 px default; 200–360 px resizable range.
- Context inspector: 288 px default; 248–420 px range.
- Bottom panel: 220 px default; minimum 120 px; may maximize.
- Main content gutters: 16 px standard, 12 px compact.
- Reading/form column: 640–760 px maximum for continuous explanatory text.
- Dialog widths:
  - small: 400–480 px;
  - standard: 560–640 px;
  - evidence-heavy trusted dialog: 680–760 px, bounded by viewport.

### Layout alignment

Forms use a single-column flow by default. Two-column label/control forms are
allowed only when the left labels remain readable at text scale and keyboard
order stays linear. Evidence comparisons such as old/new fingerprints may use
two columns on wide screens and stack on narrow screens.

## Shape, borders, and elevation

| Role | Default |
| --- | --- |
| Small control radius | 4 px |
| Standard control/card radius | 6 px |
| Dialog radius | 8 px |
| Pill/badge radius | Full only for short status chips |
| Standard border | 1 px |
| Strong/focus border | 2 px equivalent visual weight |
| Focus ring | 2 px with 2 px separation from control edge |

Elevation is restrained:

1. canvas;
2. surface/sidebar;
3. raised menu/card;
4. overlay/dialog.

Shadow is never the only boundary. High contrast and reduced transparency
replace shadow with explicit borders.

## Color architecture

### Semantic roles

Components use:

- `canvas`, `surface`, `surface-raised`, `surface-overlay`;
- `border`, `border-strong`, `divider`;
- `text-primary`, `text-secondary`, `text-muted`, `text-disabled`;
- `accent`, `accent-hover`, `accent-pressed`, `on-accent`;
- `focus`;
- `success`, `warning`, `danger`, `info` plus matching subtle surfaces and
  on-color text;
- `selection`, `hover`, `pressed`, `disabled`;
- `session-active`, `session-inactive`;
- environment roles `local`, `development`, `staging`, `production`;
- terminal foreground/background/cursor/selection and ANSI palette.

Every resolved pair must meet the required contrast. Status roles always include
an icon and text label where the meaning matters.

### Dark theme baseline

| Role | Value |
| --- | --- |
| Canvas | `#0B0F14` |
| Surface | `#111821` |
| Raised surface | `#17212C` |
| Overlay | `#1D2936` |
| Border | `#2B3A4A` |
| Strong border | `#52677D` |
| Primary text | `#F1F5F9` |
| Secondary text | `#C2CCD8` |
| Muted text | `#94A3B5` |
| Disabled text | `#687789` |
| Accent | `#6DAAFF` |
| Accent hover | `#8BBCFF` |
| Accent pressed | `#4B91ED` |
| On accent | `#07111E` |
| Focus | `#9AD1FF` |
| Success | `#55D697` |
| Warning | `#F2BE5C` |
| Danger | `#FF7A85` |
| Information | `#6FD0EF` |
| Selection surface | `#17385F` |

### Light theme baseline

| Role | Value |
| --- | --- |
| Canvas | `#F4F7FA` |
| Surface | `#FFFFFF` |
| Raised surface | `#EDF2F7` |
| Overlay | `#FFFFFF` |
| Border | `#CBD5E1` |
| Strong border | `#8797AA` |
| Primary text | `#172033` |
| Secondary text | `#3D4B5F` |
| Muted text | `#627187` |
| Disabled text | `#8A97A8` |
| Accent | `#1769D2` |
| Accent hover | `#0D5BBC` |
| Accent pressed | `#094B9E` |
| On accent | `#FFFFFF` |
| Focus | `#0756B3` |
| Success | `#137A4B` |
| Warning | `#815400` |
| Danger | `#B4232E` |
| Information | `#036783` |
| Selection surface | `#DCEBFF` |

These values are the bundled baseline, not permission for components to use hex
values directly. Automated contrast checks and manual visual review remain
required.

### Environment roles

Environment color helps orientation but never carries authority:

- `LOCAL` with device icon;
- `DEV` with flask/code icon;
- `STAGING` with stacked-layer icon;
- `PRODUCTION` with filled diamond/critical-target icon.

The full label appears in connection reviews. Users may alter environment
colors within validation bounds, but not remove labels/icons from material
operation reviews.

## Trusted safety tokens

Trusted safety surfaces use reserved tokens outside user themes:

- trust frame and shield mark;
- trusted heading and evidence labels;
- warning/critical trust surface;
- destructive action emphasis;
- verified/changed/revoked identity state;
- secure-input treatment;
- minimum focus and contrast.

The frame includes an invariant shield icon, `Relio security check` label, and
strong inset border. Theme preview shows this surface as locked. Remote content
and normal cards cannot use the trust frame.

## Icons

- Use one coherent outlined icon family bundled with the product.
- Default icon size: 16 px; rail and prominent status: 20 px.
- Icon-only controls require an accessible label and tooltip.
- Destructive, recording, trusted, and environment icons have distinct shapes.
- Avoid brand/service logos and decorative infrastructure metaphors.
- Spinners indicate indeterminate activity only; determinate work uses progress.

## Motion

- Default micro-transition: 120–160 ms.
- Panel/dialog transition: 160–220 ms.
- Progress animation never implies measured progress when none exists.
- Reduced motion removes translation/scale and uses immediate state or subtle
  opacity within platform guidance.
- Trusted confirmations never pulse, shake, or use urgency theater.

## Component principles

1. Components expose semantic states: default, hover, focus, active, disabled,
   loading, invalid, warning, destructive, and unavailable where relevant.
2. Disabled controls explain why through adjacent text, tooltip, or inspector.
3. Loading preserves layout to avoid focus and pointer shifts.
4. Content strings are treated as untrusted and support long/non-Latin values.
5. Components do not infer security approval from visual state.
6. Destructive intent is explicit in label, not communicated only by color.
7. Each component documents keyboard behavior and accessible name.

## Buttons

### Hierarchy

| Type | Use | Limit |
| --- | --- | --- |
| Primary | One preferred forward action in a region/dialog | One per action group |
| Secondary | Alternative or supporting action | May be multiple |
| Quiet | Low-emphasis toolbar/context action | Must retain clear focus state |
| Destructive | Irreversible or high-impact removal | Uses explicit verb and review |
| Icon | Conventional compact action | Requires label and tooltip |

Button labels use verbs plus objects: `Create workspace`, `Connect`, `Stop
forward`, `Replace remote file`. Avoid `Yes`, `OK`, or `Continue` when the
consequence can be named.

Loading buttons preserve width, show a progress marker, and prevent duplicate
submission while leaving Cancel available elsewhere.

## Inputs

### Text input

An input includes persistent label, optional description, value, and reserved
validation space when validation is likely. Placeholder is example content,
never the sole label.

Validation occurs:

- on blur for ordinary formatting;
- immediately for forbidden control characters or unsafe ranges;
- on submit for cross-field and provider checks.

Error text states the requirement and keeps valid input intact.

### Secure input

Secure inputs:

- use trusted safety chrome;
- never preserve value through navigation/crash recovery;
- never offer ordinary copy;
- use platform secure input when available;
- disclose reveal behavior and use deliberate press-and-hold where supported;
- do not show secret strength theater for credentials Relio does not define.

### Search input

Search labels its scope, supports clear, announces result count after a short
debounce, and preserves keyboard position in virtualized results.

## Dropdowns, selects, and menus

- Use a select for one choice from a short stable set.
- Use a searchable combobox for hosts, workspaces, credentials, fonts, or other
  larger sets.
- Use a menu for actions, not stored values.
- Current selection is marked by text and check icon.
- Menus do not contain form-sized advanced configuration.
- Submenus are avoided; one level is the maximum in v1.
- Menu action labels include the target when selection could be ambiguous.

## Dialogs

Dialogs follow the layout and trust rules in
[application layout](application-layout.md).

Additional requirements:

- focus moves to the title or first safe field, not automatically to a
  destructive/approval action;
- Escape means Cancel unless a non-cancellable OS transition is underway;
- Enter submits only from a valid form or explicitly focused action;
- destructive confirmations show consequence before the destructive button;
- evidence can scroll, but title, trust marker, and actions remain reachable;
- nested dialogs are prohibited except a platform picker opened from a Relio
  dialog.

## Cards

Cards group summary content such as a recent workspace or host capability.
They are not the default container for every section.

- Entire-card activation is allowed only when there is one navigation action.
- Secondary actions remain separate and keyboard reachable.
- Status, environment, and last-result metadata align consistently.
- Interactive cards have border/focus treatment, not hover-only elevation.

## Tables and lists

Use tables for comparable records; use lists for navigational or heterogeneous
items.

Tables provide:

- sticky header where scrolling is long;
- sort indicator with accessible direction;
- column chooser only for complex inventory views;
- row focus independent from selection;
- virtualization/paging for large sets;
- empty and filtered-empty states;
- horizontal overflow without truncating critical target identity.

Bulk selection is introduced only for safe, genuinely repeatable operations.
Host deletion, credential removal, host-key replacement, and remote overwrites
remain individually reviewable.

## Tabs

- Tabs represent sibling views, not sequential wizard steps.
- Active state uses shape, border, and semantics.
- Close targets do not overlap drag targets.
- Overflow opens a searchable list with full target and state.
- Keyboard supports next/previous, focus tab strip, and close active.
- Dirty, disconnected, and recording indicators use different iconography.

## Panels

Sidebars, inspector, and bottom panel:

- expose labelled landmarks;
- remember size within safe bounds;
- provide a keyboard resize path;
- return focus when collapsed;
- use a visible empty state rather than a blank region;
- never cover trusted modal actions.

## Notifications

| Pattern | Use | Persistence |
| --- | --- | --- |
| Inline message | Validation or surface-owned status | Until resolved |
| Toast | Completed low-risk action | Timed, pause on hover/focus |
| Status item | Ongoing operation/background state | While active |
| Problems item | Failure requiring review | Until dismissed/resolved |
| Modal | Decision required before safe continuation | Until decision |

Toasts have one optional action, are screen-reader announced without
interrupting terminal input, and never contain the only recovery path.

## Tooltips

- Name an unfamiliar icon or truncated value.
- Do not contain required instructions, forms, or secrets.
- Appear after 500–700 ms pointer dwell and immediately on keyboard focus.
- Remain open long enough to move the pointer onto selectable explanatory text
  only when the tooltip pattern supports it accessibly.
- Trusted evidence uses visible labels, never tooltip-only detail.

## Progress

- Determinate bar when total work is trustworthy.
- Indeterminate bar when progress cannot be measured.
- Multi-file transfer shows overall progress only when item sizes are known,
  plus current-item detail.
- Cancellation state uses `Cancelling…` and does not claim completion until the
  owner reports it.
- Completed, failed, and cancelled are terminal states with timestamps.

## Empty states

An empty state includes:

1. literal title;
2. one-sentence explanation;
3. one primary action when applicable;
4. optional secondary local documentation action.

Examples:

- `No hosts yet` — Add an SSH host or open a local terminal.
- `History is off` — Explain that prior input cannot be reconstructed; link to
  retention settings.
- `No active transfers` — Do not promote unrelated features.
- `Workspace needs repair` — List unresolved references rather than using
  generic illustration.

Illustrations are optional and subordinate to the actionable explanation.

## Error states

Error states answer:

1. What failed?
2. Which target/phase was affected?
3. What remained safe or was cleaned up?
4. Can the user retry?
5. What action is likely to help?
6. Where can safe detail be inspected?

Use typed human-readable titles such as `Authentication rejected` or `Remote
file changed`, not raw provider text or numeric codes. Error codes remain
available in diagnostic detail.

## Skeleton, loading, and unavailable states

- Skeletons are used for predictable metadata layouts, never terminal output.
- A delay prevents flashes for sub-200 ms work.
- After a bounded threshold, show the named phase and Cancel where supported.
- Unavailable capability uses disabled action plus diagnosis; it is not styled
  as a transient loading state.

## Accessibility acceptance

Every component must pass:

- WCAG 2.2 AA contrast for required states;
- keyboard operation without focus loss;
- visible focus under light, dark, and custom themes;
- 200% text scaling without loss of content/action;
- screen-reader name, role, state, and error association;
- forced-colors/high-contrast mode;
- reduced motion;
- no color-only meaning.
