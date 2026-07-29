# Plugin Security

Plugins are a major attack surface because they expand the code and network that a user chooses to run near production credentials. The plugin model must assume a plugin can be compromised, malicious, buggy, or abandoned.

## Trust model

- the Relio core is trusted to enforce policy;
- the plugin host is a containment layer, not a guarantee against a fully compromised device;
- installed plugins are untrusted until verified and approved;
- first-party plugins are subject to the same public contracts and review expectations as external plugins;
- a marketplace is a distribution catalog, not a trust decision on behalf of the user;
- a valid signature identifies a publisher/artifact relationship but does not prove benevolence.

## Permission model

Plugins request named capabilities in their manifest. The UI presents purpose, scope, persistence, and risk before approval. No plugin automatically receives access to:

- SSH keys or private key material;
- passwords, tokens, certificates, or raw keychain values;
- arbitrary filesystem paths;
- arbitrary network destinations or listening sockets;
- process spawning or shell execution;
- raw terminal input/output;
- workspace writes or destructive operations.

Examples of narrower grants include `workspace.read`, `host.metadata.read`, `session.observe`, `filesystem.read(user-selected-path)`, `network.connect(approved-provider)`, or `session.input.request` with a confirmation boundary.

## Isolation strategy

The initial runtime is an out-of-process plugin host using versioned JSON-RPC. The host enforces:

- manifest and protocol validation;
- capability checks on every privileged call;
- request deadlines and cancellation;
- message and output size limits;
- plugin-specific storage namespaces;
- activation and shutdown lifecycle;
- crash containment and disable/restart controls;
- diagnostic attribution to the plugin ID.

Where supported, add OS-level sandboxing, restricted environment variables, filesystem allowlists, network policy, user separation, and resource limits. A future WebAssembly runtime may provide a more constrained class of plugin, but it does not replace the process-boundary model for all integrations.

## UI security

Plugins cannot access the workbench DOM or inject arbitrary CSS or scripts. UI contributions use declared commands, schemas, view slots, and constrained rendering data. This protects the workbench from both visual spoofing and internal DOM coupling.

## Signing and installation

Plugin packages should include publisher, version, compatibility, requested permissions, license, hash, and signature metadata. Installation verifies integrity before activation. Signature failures block installation; unknown publishers produce a clear trust warning; permission changes on update require renewed consent.

Local development plugins can be enabled with a visible development-mode warning and should never be silently treated as production-trusted.

## Updates and rollback

- verify package integrity and signature before replacement;
- download through validated TLS;
- preserve the previous known-good version until activation succeeds;
- support disable and rollback without deleting plugin-owned user data;
- do not allow an update to widen capabilities silently;
- record publisher, version, permission changes, and result in local diagnostics without secrets.

## Remaining risks

Process isolation may not stop a malicious plugin from abusing permissions the user approved or exploiting an OS vulnerability. Users should install only plugins they trust, and production use should support organizational allowlists or a plugin-disabled mode.
