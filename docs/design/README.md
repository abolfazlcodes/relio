# Relio UX Design Specification

- Status: v1 design baseline
- Audience: product design, engineering, security, accessibility, and quality
- Scope: desktop Relio v1

This directory defines the complete user experience to be reviewed before
application implementation. It translates the product, architecture, and
security decisions into observable interface behavior. When a design rule
conflicts with a security or architecture invariant, the focused architecture
or security document remains authoritative and this specification must be
corrected.

## Experience model

Relio is a local-first operations workspace. It brings terminals, remote files,
transfers, tunnels, hosts, and reusable commands into one calm workbench while
keeping the shell and remote system authoritative.

The primary nouns are:

- **workspace:** a local composition of sessions, views, settings, snippets,
  and references to global hosts;
- **host:** reusable connection metadata and credential references;
- **session:** one live local shell or SSH connection, or restorable metadata
  for creating a new one;
- **pane:** a visual container for a session or core tool surface;
- **operation:** a connect, transfer, remote save, tunnel, or other action with
  a visible target, lifecycle, and result.

## Documents

1. [Product design philosophy](product-design-philosophy.md)
2. [User personas](personas.md)
3. [Information architecture](information-architecture.md)
4. [Core user flows](core-user-flows.md)
5. [Application layout](application-layout.md)
6. [Design system](design-system.md)
7. [Theme system UX](theme-system-ux.md)
8. [Keyboard-first experience](keyboard-first-experience.md)
9. [Security UX](security-ux.md)
10. [Low-fidelity wireframes](wireframes.md)

## Normative language

- **Must** and **must not** describe release requirements.
- **Should** describes the expected default; exceptions need a documented
  reason.
- **May** describes an optional implementation choice inside the stated
  boundary.

## Fixed v1 boundaries

The v1 experience includes SSH, local and remote terminals, verified SFTP-based
transfer semantics, host management, workspaces, sessions, split panes,
history, snippets, port forwarding, remote file management, recording controls,
and data-only theme customization.

It includes no account requirement, hosted state, runtime-loaded application
functionality, external service connection, or collaborative surface. Future
ideas do not reserve navigation, settings, commands, permissions, or empty
states in this design.

## Review checklist

Every implemented screen or flow should be reviewed against:

- target, identity, and environment visibility;
- keyboard and screen-reader completion;
- loading, empty, unavailable, error, cancellation, and success states;
- narrow and large window behavior;
- dark, light, high-contrast, reduced-motion, and text-scaled presentation;
- hostile, long, non-Latin, and non-round-trippable remote content;
- privacy and retention disclosure;
- trusted safety chrome invariance;
- no silent command submission, reconnect, overwrite, broad network bind, or
  credential fallback.
