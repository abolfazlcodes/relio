# Supply-Chain Security

Relio depends on operating-system libraries, Rust crates, JavaScript packages, build tools, plugin packages, and release infrastructure. A compromise in any of these can affect users operating production systems.

## Dependency controls

- commit and review Rust and frontend lockfiles;
- use supported, maintained versions and remove unused dependencies;
- audit licenses and known vulnerabilities before adoption;
- review dependency capabilities, build scripts, native code, and transitive network behavior;
- avoid dependencies that introduce an unnecessary runtime or privilege;
- document exceptions and owners for accepted risk.

## Automated scanning

CI should run, at minimum:

- Rust and frontend vulnerability audits;
- secret scanning;
- static analysis and linting;
- license and attribution checks;
- container/toolchain image scanning where used;
- SBOM generation for release artifacts;
- dependency freshness and abandoned-package reporting.

Automated results require triage. A scanner warning is not automatically a vulnerability, and a clean scan is not proof of safety.

## Release integrity

Stable artifacts must be:

- built from a protected, reviewed tag;
- signed with protected release keys;
- published with checksums, provenance, and SBOM;
- verified by the updater before installation;
- retained with the previous known-good version for rollback;
- promoted through staged channels where practical.

Release signing keys must not be stored in the repository or on an ordinary developer workstation. Key access, rotation, recovery, and revocation require documented ownership.

## Reproducible builds

The project should pin toolchains, lock dependencies, record build inputs, avoid timestamps and uncontrolled network fetches, and publish enough metadata for independent rebuild attempts. Reproducibility is an engineering goal; any remaining non-determinism must be measured and documented.

## SBOM and provenance

Each stable installer and plugin package should publish an SBOM in a recognized format and identify source revision, build environment, dependency lock state, and signing identity. The update service must bind metadata to the exact artifact and target platform.

## Plugin and marketplace supply chain

Marketplace packages are not trusted solely because they are listed. Verify signatures and hashes, show publisher and permissions, preserve rollback, and allow local installation from a verified package without requiring an account. A publisher change or capability increase is a material security event.
