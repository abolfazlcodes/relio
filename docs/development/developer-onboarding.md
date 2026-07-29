# Developer Onboarding

## Before implementation exists

Read, in order:

1. [Vision](../product/vision.md)
2. [Feature map](../product/feature-map.md)
3. [Architecture overview](../architecture/overview.md)
4. [Technology decisions](../architecture/technology-decisions.md)
5. [Roadmap](../roadmap.md)
6. [Contributing guide](contributing.md)

At this stage, the repository is intentionally documentation-only. Do not create application code until the relevant phase is selected and its acceptance criteria are understood.

## Expected implementation prerequisites

When Phase 1 begins, contributors will need:

- Git;
- Rust stable toolchain and Cargo;
- Node.js LTS and the project package manager;
- Tauri prerequisites for the host operating system;
- a supported browser/webview runtime supplied by the platform;
- a local shell suitable for testing;
- platform-specific build and signing tools only when packaging.

Exact versions belong in the build documentation once the first app scaffold exists. Avoid documenting a version number before the project locks it in a reproducible toolchain file.

## First contribution path

Good early contributions are documentation corrections, competitor evidence, design exploration, test fixtures, performance experiments, and small architecture decision records. A contributor should be able to explain which boundary the change touches and how it will be verified.

## Learning path for frontend engineers

1. Learn the Tauri request/event lifecycle and the difference between frontend and host authority.
2. Learn Rust ownership, error handling, traits, and async/concurrency only as needed by a module.
3. Build a small local command or repository use case before touching the terminal runtime.
4. Study PTYs and process groups before implementing session behavior.
5. Add integration tests with fake processes before connecting real hosts.
6. Read the security document before handling credentials, paths, network sockets, or plugins.

The goal is gradual systems exposure. A frontend contributor does not need to become a kernel or cryptography expert to make valuable changes, but should understand the boundary they are crossing.
