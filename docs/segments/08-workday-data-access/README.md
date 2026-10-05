# Segment 08: Workday data access

Status: not started. This is a proposed use case, not a Workday implementation or production plan.

## Research question

Which approved browser or data interface fits a specific Workday task? Can it produce complete, correct results with fewer browser and LLM steps?

The [saved discussion](../../sources/2026-10-05-discussion.md) asks about general website operation. The [new report](../../sources/2026-10-05-workday-chrome-automation-report.md) adds Workday as a case study. [Segment 07](../07-rust-chrome-controller/README.md) holds controller mechanics and local comparisons. [Current recommendations](../../recommendations.md) holds selection priorities.

## Terms

- **WQL (Workday Query Language):** a Workday query interface for data.
- **RaaS (Reports-as-a-Service):** access to configured Workday reports through a service.
- **Pagination:** fetching a collection in successive parts.
- **Provenance:** a record of where data came from and how it was collected.
- **Classification:** a label that states the handling rules for data.

## Scope and discovery

Define one read-oriented task, its fields and its expected result before selecting a source. Inventory which documented APIs, WQL queries, RaaS reports and UI exports are enabled and permitted for the task. API-first extraction is conditional on access, field coverage and equivalent results. It is not a rule that every Workday task has an API replacement.

| Question | Evidence needed |
|---|---|
| Which tenant and workflow are in scope? | Owner, approved test environment and expected result |
| Which operations are permitted? | Specific workflow capabilities and least-privilege access |
| Which fields may be collected and shown to the model? | Data classification and approved handling rules |
| Which documented source covers those fields? | Tenant capability check, query/report definition and access test |
| Which contract terms apply? | The organisation's applicable documents and responsible owner's determination |
| How is the session obtained and renewed? | Approved browser arrangement or integration credentials; human SSO/MFA handoff where required |
| Which rate and retention rules apply? | Tenant guidance and destination policy |

[Workday's public site terms](https://www.workday.com/en-us/legal/site-terms.html), updated 2026-08-13, address defined Sites and contain restrictions on scraping and unconsented interaction. They do not alone establish the terms for every customer tenant. Workday's [customer contract framework](https://www.workday.com/en-us/legal/universal-contract-terms-and-conditions/index.html) says only documents referenced in the relevant Order Form or UMSA apply. Record the applicable access conditions rather than treating a public-site clause as a universal tenant conclusion.

Credentials and cookies remain outside the LLM. A generic button click is not a read-only guarantee. Define allowed operations such as a particular search, report execution or export; reject unknown operations. No task here authorizes a create, update or approval transaction. Investigate these access controls in an approved test environment if the use case proceeds.

## Extraction routes and completeness

| Route | Research question |
|---|---|
| Documented API or WQL | Does the permitted query supply the needed fields and complete collection? |
| RaaS | Does an approved report supply an appropriately bounded result? |
| Browser report/export | Can the permitted UI workflow produce a verified file? |
| Browser table extraction | Can row identity, pagination and a reference result prove completeness? |

Workday documents WQL pagination with `limit` and `offset`, with a limit of up to 10,000 rows. It also documents a cached query result in the user session with a maximum lifetime of 30 minutes. These are source capabilities, not evidence that a particular tenant task can use them. See the [WQL administration guidance](https://doc.workday.com/admin-guide/en-us/reporting-and-analytics/custom-reports-and-analytics/workday-query-language-wql-/sfx1612553126122.html). The [WQL/RaaS comparison](https://developer.workday.com/documentation/GUID-8f1d3acf-87ba-4de7-9dc5-84e3991be6a2-enHYPHENus/ReferenceWQLandRaaSComparisons) states that RaaS does not support pagination.

A WQL experiment must track query identity, start time, offsets and collected record keys. Checkpoint the collection. After expiry, do not assume that continuing an offset in a new result gives the same snapshot. Investigate the supported restart behavior and reconcile record IDs, counts and changed data against a reference. Mark the output incomplete until this process proves completeness.

For the UI fallback, use the scrolling and download checks in segment 07. Do not count a fixed-size grid's visible rows as the entire collection. Observed private network endpoints can help identify a documented equivalent; do not silently replay them as an integration.

## Export contract to test

The report proposes structured exports. A future export should carry enough information to validate and trace its data without exposing credentials.

| Group | Proposed fields |
|---|---|
| Identity | Schema version, run ID, record ID and entity type |
| Source | Source kind, approved query/report/workflow name, tenant alias and query/route hash |
| Collection | Extraction time, pagination/checkpoint information and complete/incomplete status |
| Data | Typed fields, explicit null handling and deterministic column names |
| Provenance | Request or observation identifier, snapshot/file hash and validation result |
| Handling | Classification, permitted destination and retention rule |

For CSV, keep operational metadata distinguishable from the domain columns. Verify keys, row counts, field types and reference values. A file hash proves file identity, not data correctness. A download is successful only after completion and the expected format/content checks.

Never place cookies, tokens, SSO assertions or full sensitive URLs in export metadata. Restrict raw data, traces and screenshots to their permitted destination and retention period. Keep only the fields needed for the task. Investigate local filtering and summaries before sending rows to an LLM; token budgets are measurements to record, not adopted report estimates.

## Sandbox experiment

1. Record the approved task, environment, access conditions, fields and reference result. No live tenant access is assumed by this repository.
2. Discover the available documented sources. Compare an allowed data route with the baseline UI workflow where both cover the same task.
3. Test human authentication handoff, search, report execution, filtering, export and session expiry as applicable.
4. Test pagination expiry, duplicate keys, changed source data, permission failures, no results, interrupted downloads and partial collections.
5. Use the same logical task and correctness checks for each controller candidate. Keep local fixture results separate from sandbox results.
6. Measure full task success, exact reference agreement, recovery and incomplete results. Also record elapsed time, rows collected, model round trips and resource use. Separate HTTP/operation failures from verified successful timing samples.
7. Begin with serial tab actions and configurable conservative concurrency. Apply tenant guidance before increasing it; the report's sample rates are not Workday limits.

The result should be a capability matrix, source comparison, export validation record and list of unresolved failures. Use these artifacts to evaluate the next proof stage in current recommendations. The source report's seven-to-twelve-week schedule is an untested planning estimate and is not a commitment of this learning project.

## Evidence gaps

- Tenant API availability, licensing, exact field coverage and access conditions are unknown.
- No comparison proves that direct CDP outperforms Playwright on this task.
- Supported recovery after WQL cache expiry needs a task-specific test.
- A full-task reliability target and sufficient sample size have not been defined.
- Retention, model visibility, rate limits and destination controls need the relevant owner's rules if this use case proceeds.

The source report's code and API samples remain illustrative, uncompiled and unexecuted. This segment does not define an implemented daemon or runnable benchmark.

Primary sources linked in this document were checked on 2026-10-05. The [document review](../../review/2026-10-05-document-review.md) records source defects and how the derived research plans handle them.
