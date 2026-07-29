# Developer Onboarding

## Before implementation exists

Read, in order:

1. [Vision](../product/vision.md)
2. [Feature map](../product/feature-map.md)
3. [Technical blueprint](../architecture/technical-blueprint.md)
4. [Architecture overview](../architecture/overview.md)
5. [Security architecture](../security/README.md)
6. [Technology decisions](../architecture/technology-decisions.md)
7. [Roadmap](../roadmap.md)
8. [Contributing guide](contributing.md)

At this stage, the repository is intentionally documentation-only. Do not create application code until the relevant phase is selected and its acceptance criteria are understood.

## Expected implementation prerequisites

When Phase 1 begins, contributors will need:

- Git;
- Rust stable toolchain and Cargo;
- Node.js LTS and the project package manager;
- Tauri prerequisites for the host operating system;
- pnpm at the version pinned by the root `packageManager` field;
- a supported browser/webview runtime supplied by the platform;
- a local shell suitable for testing;
- platform-specific build and signing tools only when packaging.

Exact versions belong in reproducible toolchain and package-manager files once
the first app scaffold exists. A contributor does not choose or upgrade the
encrypted SQLite build, Tauri runtime or desktop modules, terminal parser,
OpenSSH interaction layer, or cryptographic dependency casually; those are
security-critical dependencies with review requirements.

## First contribution path

Good early contributions are documentation corrections, competitor evidence, design exploration, test fixtures, performance experiments, and small architecture decision records. A contributor should be able to explain which boundary the change touches and how it will be verified.

## Learning path for frontend engineers

1. Learn the Tauri request/event lifecycle and the difference between frontend and host authority.
2. Learn Rust ownership, error handling, traits, and async/concurrency only as needed by a module.
3. Build a small local command or repository use case before touching the terminal runtime.
4. Study PTYs and process groups before implementing session behavior.
5. Add integration tests with fake processes before connecting real hosts.
6. Read the focused security document before handling IPC exposure,
   credentials, encryption, paths, network sockets, SSH helpers, updates, or
   remote file operations.

The goal is gradual systems exposure. A frontend contributor does not need to become a kernel or cryptography expert to make valuable changes, but should understand the boundary they are crossing.
