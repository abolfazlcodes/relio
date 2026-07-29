# ADR-007: Encrypt local profile data at rest

- Status: Accepted
- Date: 2026-07-29
- Owners: Relio maintainers

## Context

Relio stores host inventories, workspace structure, snippets, connection
history, and possibly terminal recordings. Keeping credentials
out of SQLite is necessary but does not make the remaining data low sensitivity.
File permissions and full-disk encryption are useful platform controls but are
not uniformly present and do not protect a copied profile.

Selective column encryption would complicate indexes, leak substantial
metadata, and create a recurring classification burden.

## Decision

Use a SQLCipher-compatible SQLite build for the complete application database.
Generate a random 256-bit profile root key and protect it with the OS secret
store. Derive a database key and independent wrapping keys using a reviewed KDF
with domain separation.

Store large recordings and logs as envelope-encrypted immutable segments
outside SQLite, indexed by the encrypted database.

There is no plaintext persistent fallback. If the secret store is unavailable,
the profile remains closed and the user may choose a separate non-persistent
local-terminal mode.

## Rationale

Full-database encryption protects data and indexes consistently while retaining
SQLite transactions and query behavior. A random key avoids low-entropy user
passwords. Separate blob encryption keeps database size and write amplification
bounded.

## Alternatives considered

- **Plain SQLite plus ACL/full-disk encryption:** rejected as the product
  baseline because protection depends too heavily on external configuration.
- **Application-level encryption for selected columns:** rejected because
  classification and query leakage are easy to get wrong.
- **SQLite SEE:** not selected because its licensing/distribution model must be
  evaluated against the open-source governance decision.
- **A custom encrypted database:** rejected as unnecessary and unsafe.

## Consequences

- Encryption library packaging, attribution, update response, and performance
  become release requirements.
- Key loss means data loss unless an independently usable encrypted backup
  exists.
- Database opening depends on a working platform secret store.
- Migration and rollback procedures must preserve encryption.

## Migration or follow-up

Before Phase 3:

- complete cross-platform build and performance spikes;
- approve dependency licensing and notices;
- specify the encrypted blob framing and concrete audited library;
- test database, WAL, journal, temporary, backup, and crash behavior;
- commission focused review before storing real user data.

## Evidence and references

- [SQLCipher security design](https://www.zetetic.net/sqlcipher/design/)
- [SQLCipher Community Edition license requirements](https://www.zetetic.net/sqlcipher/license/)

Relio must verify the exact selected build and notices rather than relying on a
generic product claim.
