# Security UX

## Security experience goal

Relio should help users make correct trust and authority decisions without
turning every connection into a warning ritual. Security interactions are
specific, proportional, and bound to one target. The interface explains both
the practical consequence and the technical evidence.

Security UX cannot prove that a user-approved command, remote host, or file is
benign. It can prevent ambiguity, preserve evidence, avoid unsafe fallback, and
make consequential decisions deliberate.

## Security interaction levels

| Level | Pattern | Examples |
| --- | --- | --- |
| Inform | Persistent status or inline fact | Known verified host, keychain source, encrypted-profile state |
| Review | Inline/side-panel summary before action | Normal SFTP transfer, known-host connection detail |
| Confirm | Trusted modal requiring explicit action | First-seen fingerprint, broad bind, overwrite, recording enable |
| Block | Trusted error with no continue action | Changed/revoked key, unavailable protected store, legacy SCP semantics |
| Re-authenticate | OS/core confirmation plus explicit decision | Credential change/removal, key replacement, high-risk delegation |

Re-authentication proves access to the protected local authority; it does not
replace understanding of the action. The consequence remains visible after
re-authentication.

## Trusted safety chrome

### Anatomy

Every trusted confirmation includes:

1. invariant shield icon and `Relio security check` label;
2. action-specific title;
3. exact target and environment;
4. identity/credential/path/port evidence as applicable;
5. consequence and available safety guarantee;
6. optional technical detail;
7. Cancel and explicit action label.

The trusted frame uses reserved design tokens and cannot be themed. Dialog
placement is outside terminal/file content and inside application-owned chrome.
The underlying untrusted surface is inert while the modal is active.

### Spoof resistance

- Terminal output cannot create working Relio buttons.
- Remote text cannot register a command, tooltip, toast, or notification.
- Theme names and remote filenames are escaped and visually separated from the
  trusted heading.
- Security decisions never occur inside a terminal prompt styled by the remote
  host.
- The dialog binds displayed evidence to an operation ID; changing host data
  invalidates it.
- Replayed decisions fail.

No visual treatment can prevent all social engineering. Relio teaches users to
look for the invariant frame and exact target, not merely a color.

## Confirmation grammar

Security copy follows:

```text
Action
Target
Why Relio is asking
What will happen
What Relio can and cannot guarantee
Explicit decision
```

Preferred labels:

- `Trust once`;
- `Trust and save`;
- `Replace stored host key`;
- `Bind to all interfaces`;
- `Replace remote file`;
- `Delete credential reference`;
- `Start recording`.

Avoid `OK`, `Yes`, `Proceed anyway`, or reassuring language that hides the
specific risk.

Cancel is always safe and available. Closing the dialog equals Cancel. Enter
does not activate a trust/destructive action unless that action has explicit
focus.

## SSH fingerprint verification

The complete flow is defined in
[core user flows](core-user-flows.md#5-ssh-fingerprint-verification).

### Evidence presentation

- Use SHA-256 fingerprint in monospaced grouped text.
- Show algorithm, host/address, port, username, jump chain, and evidence source.
- Offer copy of the fingerprint only.
- Provide bundled guidance for obtaining a fingerprint through an independent
  trusted channel.
- Never present unauthenticated scan output as verified identity.

### First seen

The first-seen dialog is a neutral verification decision, not a generic danger
screen. It distinguishes:

- no previous Relio record;
- matching approved read-only known-hosts source;
- candidate evidence only.

`Trust once` authorizes only this connection operation. `Trust and save`
creates a Relio-managed known-host record after explicit acknowledgement.

### Changed/revoked/ambiguous

These states block the connection. Changed-key replacement requires a new
review flow showing old/new evidence, an optional local reason, and
re-authentication when policy requires it. Revoked or malformed evidence has no
trust-once path.

Warnings must not normalize likely-risk language with phrases such as “This is
probably fine.”

## Credential storage UX

### Mental model

The UI uses:

- `Credential source` for agent, external key file, or OS-stored secret;
- `Credential reference` for Relio’s opaque association;
- `Secret value` only inside a trusted input flow.

It never calls the encrypted profile a password vault. Private key files remain
user-controlled external files; Relio stores a protected reference and does not
copy or delete their bytes.

### Credential list

Display:

- label;
- type and source;
- availability;
- associated host count;
- native access-control warning;
- last successful use when retained;
- repair, rotate, and remove.

Never display password/passphrase value, private-key contents, full secret
handle, or misleading `Reveal` control.

### Secure input

- Use platform secure input and OS re-authentication where available.
- Do not persist field values across navigation, crashes, or failed unrelated
  validation.
- Reveal, when required, is press-and-hold with an obvious revealed state.
- Copy is disabled by default and warns about clipboard-manager retention when
  explicitly enabled.
- Secret entry never appears in an ordinary webview event, diagnostic, URL,
  argument, or environment display.

### OS store unavailable

The profile remains closed. The UX says:

- which platform facility is unavailable;
- whether it is locked, denied, or absent when safely known;
- that Relio did not create a replacement/plaintext profile;
- how to retry;
- whether temporary non-persistent local terminal mode is available.

It never offers `Store insecurely`.

### External keys and agents

For a key file, show:

- selected path;
- regular-file status;
- access-control warning;
- moved/replaced/unavailable state;
- statement that Relio will not copy, modify, or delete it.

For an agent identity, show fingerprint/public identity metadata and agent
availability. Agent forwarding is a separate high-risk delegation, disabled by
default.

## Agent forwarding

Before enablement, trusted confirmation shows:

- remote target and complete jump chain;
- local agent identity;
- that the remote host may request signatures while forwarding is active;
- session or host scope;
- whether reconnect would enable it again.

Default scope is the current session. Saved host scope requires a second
explicit choice and remains reviewable in host security settings. Importing SSH
configuration never enables it implicitly.

## Dangerous action confirmations

### Confirmation matrix

| Action | Required context | Interaction |
| --- | --- | --- |
| Delete workspace | Owned records, active sessions, recordings, global records left intact | Impact dialog; explicit `Delete workspace` |
| Delete global host | Referencing workspaces, credential references left intact, no remote effect | Impact dialog; explicit `Delete host` |
| Remove credential | Affected hosts, reference vs OS item vs external file | Re-auth as needed; separate deletion choices |
| Remote overwrite | Host/environment, path, permissions, conflict, atomicity | Trusted review |
| Remote delete | Host/environment, exact paths, symlink behavior, no guaranteed undo | Trusted review; no broad glob language |
| Broad port bind | Local bind, interfaces, remote destination, transport host | Trusted review, re-auth as policy requires |
| Close live session | Local/remote target and likely process effect | Context dialog when termination is possible |
| Discard remote edits | Host/path and no crash-recovery copy | Dirty-editor dialog |
| Delete recording/history | Scope, indexes, backups/physical deletion limits | Impact dialog |

### Interaction rules

- Prefer exact target selection over typing a ritual phrase.
- Use re-authentication for authority, not as the only consent signal.
- Do not preselect destructive checkboxes.
- Do not place destructive action adjacent to a routine primary action without
  separation.
- Repeat warnings may be suppressed only for low-risk unchanged routine
  behavior; broad binds, changed keys, destructive deletes, and non-atomic
  overwrite remain explicit.
- Batch remote destructive actions must list bounded exact items and cannot be
  expressed as an unreviewed wildcard.

## Remote file and transfer safety

### Always visible

- direction;
- authenticated host and environment;
- exact remote path;
- local path category;
- transfer semantics;
- overwrite policy;
- symlink behavior;
- progress/cancellation;
- terminal result.

### SCP wording

Use:

`SCP-compatible transfer (verified SFTP semantics)`

Never use:

`Legacy SCP`, `SCP protocol`, or language implying remote-shell wildcard
behavior.

If semantics cannot be proven, block that operation and offer `Use SFTP`.

### Remote editor

- Keep host, environment, and path in editor chrome.
- Dirty state is visible in icon and text.
- Revalidate identity and target before save.
- Conflicts show remote/current evidence and never auto-merge.
- Non-atomic direct overwrite requires separate confirmation.
- State plainly that unsaved buffers are memory-only and unrecoverable after
  close/crash.

## Port-forwarding safety

The forward editor visually separates:

- listening side: bind address and local/remote port;
- transport: SSH host and jump chain;
- destination side: host and port;
- direction: local, remote, or dynamic;
- owner workspace and lifecycle.

Loopback is the default. Selecting `All interfaces` or a non-loopback bind
opens trusted confirmation describing who may reach the listener. The active
forward row shows `LOOPBACK` or `BROAD BIND` in text plus icon.

Stop confirms the owned listener ended. Port conflict errors never offer to
kill a process solely by port number.

## Sensitive output warnings

### Principle

Terminal output may contain secrets, but automatic detection is imperfect.
Relio should not interrupt normal terminal work with repeated guesses or imply
that undetected output is safe.

### Warning moments

Show clear disclosure when the user:

- enables command history retention;
- starts session recording;
- includes session content in a local diagnostic export;
- copies a detected/high-sensitivity value through a Relio-owned surface;
- exports logs/history/recordings;
- pastes multiline/control-containing text into a terminal.

### During a session

When best-effort detection identifies a likely secret while recording:

- show a persistent, non-modal `Possible sensitive output recorded` status;
- provide `Stop recording`, `Review retention`, and `Dismiss for this session`;
- do not modify or redact the authoritative terminal stream;
- do not claim other output is safe;
- keep the warning out of the terminal byte stream.

Detection data is local, bounded, and not a new retained content source.

## Session recording controls

### Off by default

Recording and derived history are independent and disabled until the user
enables them. A workspace cannot start recording merely because a layout is
restored or session reconnects.

### Start review

Show:

- session, host/environment, and identity;
- content captured;
- encrypted local storage statement;
- retention and quota;
- free-space reserve;
- best-effort redaction limitation;
- whether setting is session-only or a workspace default.

### Persistent active state

Use:

- pane-header record icon plus `Recording` accessible label;
- status-bar recording item;
- palette actions `Pause recording` and `Stop recording`;
- elapsed time and stored-size detail in the inspector.

Color alone is insufficient. Themes cannot remove or imitate the recording
indicator.

### Stop/delete

Stop waits for segment finalization and reports success/failure. Delete previews
recordings, derived indexes, and retention references. It states that physical
erasure from SSDs, snapshots, or backups cannot be guaranteed.

Storage pressure stops new recording writes before the configured reserve and
does not interrupt live terminal rendering.

## Clipboard and external content

### Clipboard

- OSC clipboard read/write is disabled by default.
- Copying terminal selection is explicit.
- Secret-copy actions warn about clipboard managers and clear only when Relio
  can confirm the same value remains.
- Relio never pastes a stored credential into a terminal automatically.
- Multiline/control-character paste gets target-aware review.

### Links

Remote terminal/file text may contain a URI, but opening it:

- requires an approved scheme;
- shows the destination outside the remote content;
- never carries a credential automatically;
- uses a deliberate external navigation action;
- cannot invoke a privileged Relio command.

## Connection and authentication errors

Security-relevant failure copy distinguishes:

- unavailable source from rejected credential;
- unknown identity from changed/revoked identity;
- unsupported provider from network failure;
- blocked unsafe configuration from syntax error;
- policy-denied algorithm from ordinary negotiation failure.

Raw stderr is untrusted diagnostic data and is never the primary user message.
Retry revalidates every security decision and cannot silently choose a weaker
route, credential, algorithm, or verification mode.

## Profile lock and privacy state

- OS session lock clears active secret leases according to provider policy.
- Application lock is described as privacy/convenience, not protection from
  malware on the unlocked account.
- Temporary mode is persistently labeled.
- Offline mode disables update checks without disabling local terminal,
  workspaces, or retained local data.
- Data-location and retention controls remain inspectable in Settings >
  Privacy and data.

## Security copy guidelines

Use:

- direct verbs;
- specific noun and target;
- short first paragraph;
- expandable exact evidence;
- honest uncertainty;
- consequence before action.

Avoid:

- blame (`You entered the wrong password`);
- false certainty (`This host is safe`);
- vague alarm (`Security issue detected`);
- minimization (`Just continue`);
- implementation leakage when a useful typed cause exists;
- secret or full sensitive path disclosure in toasts.

## Accessibility and hostile-content tests

Trusted flows must be tested with:

- 200% and 400% text scaling where platform support permits;
- keyboard-only and screen reader use;
- forced colors and reduced motion;
- very long hosts, usernames, paths, and fingerprints;
- right-to-left and mixed-direction labels;
- remote text imitating Relio wording;
- theme colors matching trust/danger colors;
- filenames containing control-like glyphs;
- clipped/narrow windows;
- replayed or stale confirmations.

The exact target, decision, and Cancel action must remain understandable and
reachable in every case.
