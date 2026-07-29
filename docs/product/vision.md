# Vision and Product Principles

## Vision

Build the next-generation Developer Operations Workspace: a fast, beautiful, local-first desktop application that helps engineers understand, connect to, and operate the systems behind their software.

The terminal remains the most compatible and powerful interaction surface. The workspace adds context around it without hiding what the user is doing.

## Problem

Infrastructure work is spread across terminal tabs, SSH profiles, text files, cloud consoles, snippets, port-forward commands, log viewers, and browser tabs. Existing terminal products are often strong in one dimension:

- powerful but difficult to discover;
- attractive but restrictive;
- configurable but fragmented;
- extensible but inconsistent;
- collaborative but dependent on a service.

This project treats a host, project, environment, and workflow as related pieces of one local workspace.

## Primary users

1. Application developers who occasionally operate remote environments.
2. DevOps, SRE, and platform engineers who manage many hosts and services.
3. Developers who need a polished terminal but want a path to deeper infrastructure tooling.
4. Contributors who want clear, stable extension points instead of modifying a monolith.

## Product promises

- Open a useful terminal quickly.
- Find the right host or workspace without remembering where it was configured.
- Keep advanced workflows available without making the default experience intimidating.
- Make dangerous or irreversible actions visible and deliberate.
- Work offline and keep user data on the user’s machine unless they opt in otherwise.
- Let the community add integrations without expanding the trusted core unnecessarily.

## Non-goals

- Reimplement every cloud provider console.
- Replace a full IDE or a general-purpose text editor.
- Hide shell behavior behind an opaque command abstraction.
- Require an account for core functionality.
- Make AI the primary interaction model.
- Support every remote protocol in the first release.
- Promise identical native behavior on all platforms when the operating systems differ.

## Product principles

### Local-first

The application must remain useful with networking disabled. Local settings, hosts, workspaces, history, and session metadata are stored locally. Sync is a separate, opt-in provider.

### Progressive disclosure

The default surface should be calm. Power features remain reachable through the command palette, context actions, and advanced views rather than being displayed everywhere.

### Terminal truth

The raw terminal stream is authoritative. Visual command blocks, detectors, suggestions, and AI summaries are derived views and must never silently alter terminal input or output.

### Safe infrastructure operations

The UI should show target, identity, command, and scope before an operation that could cause material impact. Automation and AI suggestions require an explicit execution boundary.

### Extensible by contract

Core modules expose narrow typed capabilities. Plugins contribute through documented manifests and APIs, not by reaching into internal UI or database implementation details.

### Observable quality

Performance, reliability, accessibility, and security are acceptance criteria. They are not polish to be added after feature completion.

## Initial success signals

These are product hypotheses, not launch commitments:

- a new user reaches a working local shell in under one minute;
- a returning user restores a previous workspace without manual reconstruction;
- a user can connect to a saved SSH host without re-entering routine connection details;
- common actions are discoverable through search and the command palette;
- idle CPU and memory remain low enough that the app can stay open all day;
- no core workflow requires cloud access or an account.
