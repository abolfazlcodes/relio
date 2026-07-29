# Performance and Capacity

## Principle

Relio is a workstation application. Scalability means remaining responsive with
large local inventories and many live streams while keeping resource use
bounded. It does not justify distributed services or speculative background
daemons.

## Reference profiles

Before implementation, CI must record two physical reference profiles:

- **Baseline:** supported OS, 4 logical CPU cores, 8 GiB RAM, integrated
  graphics, SSD.
- **Standard:** supported OS, 8 logical CPU cores, 16 GiB RAM, modern integrated
  or discrete graphics, SSD.

Each result records OS build, architecture, webview version, release build,
display scale, power mode, shell, dataset, and whether encryption is enabled.
Virtualized CI is useful for trends but is not the sole source of latency or
memory acceptance.

## Initial budgets

These are release acceptance budgets, not marketing claims. Baseline values are
95th percentile unless stated otherwise.

| Scenario | Initial budget |
| --- | --- |
| Cold launch to interactive empty workspace | <= 3.0 seconds |
| Request local shell to first usable prompt | <= 2.0 seconds |
| Restored workspace with 10 panes to interactive UI | <= 4.0 seconds; sessions may reconnect afterward |
| Keystroke to local terminal paint | <= 50 ms when not under intentional backpressure |
| Command-palette open | <= 100 ms |
| Search first page over reference metadata | <= 200 ms |
| Empty-workspace resident memory | <= 200 MiB |
| Ten idle local terminals resident memory | <= 400 MiB |
| Ten idle terminals average CPU after settling | <= 2% of one baseline machine |
| Remote file list first page from a responsive test host | <= 500 ms excluding connection/authentication |
| Normal UI action main-thread task | < 50 ms; no repeated long tasks |

Budgets may change only with measurements, user impact, and a documented
decision. Debug builds are not judged against release budgets.

## Capacity datasets

Contract, integration, and performance tests use deterministic generated data:

- 1,000 workspaces;
- 10,000 host profiles and 50,000 tags/associations;
- 100,000 snippets/history metadata records;
- 1,000,000 searchable metadata rows;
- a workspace with 100 tabs and 64 panes in its persisted layout;
- 20 simultaneous live sessions, including 5 sustained-output sessions;
- 10 GiB of segmented recordings with retention cleanup.

These are design targets, not recommendations to display or activate all items
at once. Default runtime limits protect the system before these maxima.

## Runtime limits

- Session, transfer, recording, and search queues are byte- and item-bounded.
- UI lists and log views page or virtualize data.
- Search returns a bounded first page and supports cancellation.
- Session supervisors have configurable global and per-host limits; the default
  warns before exceeding 20 live sessions.
- The built-in remote text editor refuses content above the 10 MiB hard limit
  and must not duplicate the full buffer through general-purpose application
  state.
- Recording storage has explicit retention and free-space thresholds.
- New disk-writing operations stop safely before consuming the configured
  reserve.

Limits are discoverable and errors identify the resource and remediation.

## Terminal throughput

Benchmark:

- burst output;
- sustained text output;
- full-screen applications;
- Unicode-heavy output;
- output while resizing, switching panes, and recording.

The test asserts byte order, no silent loss, bounded pending queues, input
responsiveness, and memory stabilization after the stream ends. A throughput
number is recorded per platform after the first implementation; the blueprint
does not invent one before xterm/webview measurements exist.

Backpressure is an end-to-end contract. It may slow the child or remote channel.
It may not allocate without bound. If a provider cannot pause and data must be
discarded, the session shows an explicit output-gap marker.

## Database and search

- Index foreign keys, normalized lookup fields, revisions, and retention
  queries.
- Use SQLite full-text search only for content the user has elected to retain.
- Avoid `SELECT *` across aggregate graphs and avoid loading recording bodies
  into list queries.
- Page with stable cursors for large changing result sets; offset pagination is
  acceptable only for small bounded views.
- Serialize writes and batch high-frequency metadata updates.
- Debounce layout and scroll-position persistence.
- Run `EXPLAIN QUERY PLAN` checks for critical reference queries.

## Profiling and regression policy

Performance tests run on release builds. CI stores trend data by platform and
flags:

- more than 10% latency or memory regression in a stable scenario;
- any budget failure;
- unbounded growth or leaked process/task;
- startup network access;
- unexpected remote connection or command work during startup.

A regression is fixed, explicitly accepted with an owner and expiry, or the
change is reverted. Adding hardware to the reference profile is not a fix.
