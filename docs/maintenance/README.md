# Project Maintenance

This section defines how Relio stays understandable, supportable, and secure
after the initial implementation. Product documents describe what Relio is;
architecture documents describe how it is built; maintenance documents
describe how decisions, dependencies, compatibility, and ownership evolve.

## Policies

- [Ten-year maintainability review](maintainability-review.md)
- [Governance and ownership](governance.md)
- [Dependency lifecycle](dependency-policy.md)
- [Documentation lifecycle](documentation-policy.md)
- [Compatibility and support](compatibility-policy.md)

## Maintainer health indicators

Review these quarterly:

- median time to first issue and pull-request response;
- pull requests waiting for a qualified reviewer;
- unowned critical modules and expiring ownership assignments;
- flaky or quarantined tests past expiry;
- dependencies outside support windows or with untriaged advisories;
- ADRs, diagrams, and compatibility tables past review date;
- unsupported migration paths still carried by stable builds;
- platform failures and support promises without current test evidence;
- release/signing/security roles with fewer than two prepared people;
- recurring incidents that indicate a missing boundary or runbook.

Metrics guide capacity decisions; they are not contributor performance scores.
