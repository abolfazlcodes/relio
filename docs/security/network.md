# Network Security

## General rule

Network access is explicit, authenticated, observable, and validated. Relio must not disable certificate or host-key verification to “make a connection work.”

## TLS and certificates

For HTTPS services such as optional sync, marketplace, update, or AI providers:

- use the platform trust store or an audited trust configuration;
- require certificate and hostname validation;
- prefer modern TLS versions and disable obsolete protocols and algorithms;
- do not provide a global “accept invalid certificates” switch;
- make test/development exceptions scoped, visible, and unavailable in stable builds where possible;
- validate redirects and prevent credential leakage across origins.

Certificate pinning is not a default substitute for normal trust validation; if a provider requires pinning, document rotation and failure recovery before shipping it.

## Proxies

Proxy configuration is treated as security-sensitive. Relio should support platform proxy settings and explicit authenticated proxies without embedding credentials in URLs or logs. Proxy commands require explicit review because they can execute local programs. A proxy must not silently downgrade TLS or SSH verification.

## SSH and tunneling

SSH host-key verification, cipher policy, jump-host display, and agent-forwarding consent are defined in [SSH security](ssh.md). Tunnels show local bind, remote destination, transport host, direction, state, and lifecycle. Loopback is the default; broad binds require an explicit warning and confirmation.

## MITM and downgrade protection

- reject invalid certificates and changed SSH host keys;
- keep protocol verification independent from UI convenience settings;
- do not retry through an alternate route with weaker security;
- surface algorithm and verification state in connection details;
- keep a reviewable record of user-approved exceptions with scope and expiry where possible.

## Connection logging

Connection diagnostics may record endpoint metadata, timing, protocol, algorithm, verification state, proxy/jump path, and error class. They must not record secret values, authorization headers, private keys, passwords, or raw payloads by default.

## Optional cloud providers

Sync, marketplace, telemetry, and AI are separate providers. Each must declare what data leaves the device, use validated TLS, provide disable/offline behavior, and fail without blocking the local terminal or host manager.
