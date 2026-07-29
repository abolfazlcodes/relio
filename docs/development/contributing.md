# Contributing Guide

## Before opening an issue

Search existing issues and documentation. For a feature idea, explain the user
problem, the current workaround, why it belongs in the focused v1 product, and
its security and maintenance cost. For a bug, include a minimal reproduction
and remove secrets from logs.

## Before opening a pull request

- identify the roadmap phase or issue;
- update relevant documentation when behavior or boundaries change;
- keep the change focused;
- add tests at the lowest useful layer;
- verify loading, empty, error, cancellation, and accessibility states for UI work;
- run the checks appropriate to the touched modules;
- review the diff for credentials, private host data, generated files, and unrelated formatting.

## Design and architecture changes

Use an ADR when a change affects technology, process boundaries, data
ownership, encryption/key formats, security assumptions, public APIs, update
trust, performance budgets, or platform support.
Small implementation choices can remain in the pull request description.

## Commit guidance

Use concise, imperative subjects and keep unrelated changes separate. A useful
commit explains one coherent change, such as
`Document host-key verification decision flow`.

## Review expectations

Reviewers should evaluate behavior, boundaries, failure handling, security, accessibility, tests, and documentation. “Works on my machine” is not sufficient for a cross-platform desktop feature; identify platform assumptions explicitly.

Security-critical dependency additions or upgrades include maintenance,
advisory, license, build-script/native-code, platform, and rollback evidence.

## Community behavior

Please follow the [Code of Conduct](../../CODE_OF_CONDUCT.md). Security vulnerabilities belong in the private reporting channel described by [SECURITY.md](../../SECURITY.md), not in a public issue.
