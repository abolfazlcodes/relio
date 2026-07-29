# Coding Standards

These standards apply when implementation begins. The project should prefer a small set of automated rules over a large style manual.

## General

- optimize for clarity and local reasoning;
- prefer explicit names and typed boundaries over clever abstractions;
- keep functions and modules focused;
- document why when code is surprising, platform-specific, security-sensitive, or performance-sensitive;
- treat warnings as defects unless a documented exception exists;
- avoid speculative abstractions: add an interface when a boundary or second implementation actually requires it.

## TypeScript and UI

- use strict TypeScript;
- keep components presentational when they do not own behavior;
- use semantic HTML and accessible labels;
- do not pass raw infrastructure errors directly to users;
- keep IPC calls behind typed feature clients;
- never put secrets in URL parameters, logs, analytics, or ordinary client state;
- prefer composition and explicit state ownership to a global singleton store.

## Rust

- use `rustfmt` and Clippy with the project’s locked configuration;
- model recoverable failures with typed errors and add context at boundaries;
- avoid panics on user input, network responses, plugin messages, or filesystem data;
- use traits at infrastructure boundaries and concrete types inside a small module;
- make ownership and shutdown behavior obvious for processes, sockets, and tasks;
- keep unsafe code out of application modules unless an ADR and focused review justify it.

## Documentation and API design

- public types and commands need examples or clear usage documentation;
- every public operation documents permissions, side effects, cancellation, and errors;
- deprecated behavior includes a replacement and migration path;
- examples must not contain real hosts, credentials, tokens, or destructive commands.
