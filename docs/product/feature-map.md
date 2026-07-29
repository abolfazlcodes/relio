# Feature Map

This map separates the trusted core from optional providers and future plugins. “Core” means the feature must work without installing a plugin or enabling a cloud service.

| Capability | Initial home | First useful milestone | Notes |
| --- | --- | --- | --- |
| Local terminal | Core | 2 | PTY, terminal rendering, input, resize, copy/paste |
| Tabs and split panes | Core | 2 | Layout is a workspace concern; panes host sessions |
| Session restore | Core | 2 | Restore metadata and layout; do not promise process resurrection |
| SQLite local data | Core | 3 | Hosts, workspaces, settings, snippets, history metadata |
| Host manager | Core | 4 | Profiles, groups, tags, favorites, connection state |
| SSH terminal | Core transport | 4 | OpenSSH-compatible path first; native provider remains replaceable |
| SFTP and file browser | Core transport | 5 | Shared file-operation contract for local and remote providers |
| Remote file editing | Core transport | 5 | Download, edit, explicit save/upload, conflict warning |
| Port forwarding | Core transport | 6 | Visual lifecycle, target/source clarity, stop/restart controls |
| Workspaces | Core | 7 | Projects organize hosts, sessions, files, snippets, and detectors |
| Infrastructure detection | Provider/plugin | 7 | Read-only detection first; actions require explicit scope |
| Command snippets and history | Core | 8 | Searchable, parameterized, workspace-aware |
| Session recording and logs | Core | 8 | Explicit opt-in, sensitive-data warnings, retention controls |
| Search everywhere | Core | 8 | Indexed local metadata and derived session content |
| Theme engine | Core | 9 | Declarative tokens for UI and terminal appearance |
| Plugin runtime and SDK | Core host + SDK | 9 | Out-of-process, capability-scoped, versioned contracts |
| Plugin marketplace | Optional service | 11 | Never required for local plugin installation or core use |
| AI assistant | Optional provider/plugin | 10 | Explain, draft, summarize, troubleshoot; never implicit execution |
| Settings sync | Optional provider | 11 | Encrypted, opt-in, secrets excluded by default |
| Cloud collaboration | Optional future service | 12+ | Must not distort the local-first core |

## Capability boundaries

- The terminal runtime owns bytes, PTY lifecycle, terminal dimensions, and transport errors.
- The workspace owns layout, active pane, metadata, and restoration.
- The host manager owns connection profiles and references to secret handles.
- Providers own protocol-specific operations such as SSH, SFTP, or Kubernetes API calls.
- Plugins own optional integrations and must declare the capabilities they need.
- The AI layer consumes user-approved context and returns suggestions or explanations; it is not a privileged execution path.
