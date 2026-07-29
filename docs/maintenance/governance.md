# Governance and Ownership

## Goals

Relio should remain governable when contributors, employers, and maintainers
change. Authority must be reviewable, transferable, and constrained. No
security-critical release or architectural decision should depend indefinitely
on one person.

## Roles

| Role | Responsibility | Authority |
| --- | --- | --- |
| Contributor | Issues, documentation, tests, implementation | No merge or release authority by default |
| Reviewer | Reviews an owned area after demonstrated context | Approval is advisory unless also a maintainer |
| Area maintainer | Owns health, review, roadmap, and compatibility for an area | Merge within area under branch rules |
| Release maintainer | Runs protected promotion and release verification | Release environment only |
| Security maintainer | Private triage, severity, disclosure, incident coordination | Security channels and embargoed fixes |
| Project maintainer | Cross-area governance, conflict resolution, role changes | Repository administration under this policy |

One person may hold multiple roles, but permissions remain role-scoped.
Signing, publishing, and routine code review should not be concentrated in the
same credential or workstation.

## Ownership map

Before implementation begins, add a review-enforced ownership file covering:

- desktop/Tauri composition and capabilities;
- frontend workbench and design system;
- terminal/session runtime;
- SSH, SFTP, remote files, and forwarding;
- persistence, encryption, migrations, and recovery;
- credentials and platform integrations;
- contracts, settings, themes, and workspaces;
- build, packaging, updater, and release;
- security and documentation.

Every critical area needs a primary owner and a backup before stable release.
Ownership means responsibility for review and maintenance, not exclusive right
to contribute. Generated files inherit ownership from their generator.

An area with no active owner is frozen for non-critical expansion. Security,
data-loss, and compatibility repairs continue through project-maintainer
assignment.

## Decision process

Routine, reversible work uses pull-request review. An ADR is required for
changes to trust boundaries, dependency direction, data ownership, supported
platforms, public contracts/formats, encryption, update trust, release
infrastructure, or a documented performance/security guarantee.

ADR flow:

1. open a proposal with context, constraints, alternatives, migration, security,
   operations, and reversal cost;
2. request review from affected owners;
3. leave a reasonable comment period proportional to reversibility;
4. record accepted, rejected, or superseded status and accountable maintainer;
5. merge the ADR before or with implementation;
6. add a superseding ADR rather than rewriting the history of an accepted one.

Urgent security fixes may use an embargoed decision record and publish the safe
portion after coordinated disclosure.

## Merge policy

- No self-merge for security-critical boundaries, migrations, update/release
  trust, cryptography, credential handoff, or capability expansion.
- Ordinary changes require one qualified approval and passing required checks.
- Critical areas require an area owner plus a second security-aware reviewer.
- Maintainers may use an emergency merge only for active incidents, with
  documented rationale, tests as soon as safely possible, and retrospective
  review within seven days.
- Large pull requests may be rejected solely because they are not safely
  reviewable.

## Role changes and inactivity

Role grants are based on sustained review quality, judgment, and respectful
collaboration—not commit count. Grants and removals are recorded publicly except
where safety or privacy requires limited detail.

A maintainer stepping away should transfer open decisions, release duties,
private security reports, and key-recovery responsibilities. After six months
without activity or response, privileged access may be removed after private
contact. Returning contributors can regain access through normal review.

## Conflict and appeal

Technical disagreement should be resolved against documented constraints,
evidence, and reversal cost. If owners cannot agree, a project maintainer who is
not the proposal author records the decision. Conduct matters follow the Code
of Conduct process and are kept separate from technical authority.

## Bus-factor release gate

Stable release requires at least two prepared people for security response,
release promotion, and signing/recovery. Where platform rules force a single
legal account holder, recovery material and operational knowledge still require
independent escrow and rehearsal.
