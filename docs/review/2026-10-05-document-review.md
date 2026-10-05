# Document review

Date: 2026-10-05.

This review records the earlier document review and the Workday report review added on the same date. It records source problems and how the derived notes separate the research goals. It does not report completed implementation experiments.

## Terms

- **LLM (large language model):** an AI system that reads and writes text, for example Claude.
- **Feasibility:** a measure of whether a task is possible with the time, tools and skill that we have.
- **Segment:** one research question in this repository. Each segment has its own folder in `docs/segments/`.
- **Crate:** a Rust code library.

## Documents that we examined

| Document | Type | Origin |
|---|---|---|
| `README.md` | Project summary | Written in this repository |
| `docs/sources/2026-10-05-discussion.md` | Saved conversation | Written in this repository |
| `docs/sources/2026-10-05-rust-browser-llm-feasibility-report.md` | Research report, about 3,500 words | Added by the user, origin not recorded |
| `docs/sources/2026-10-05-rust-browser-llm-feasibility-report.html` | Interactive web page of the same report | Added by the user, origin not recorded |

## Finding 1: The documents ask two different questions

The discussion and the README ask this question: how can an LLM operate a website? An LLM operates a website when it reads a page, clicks controls, types text and makes sure of the result.

The report asks a different question: can an LLM write a Rust web browser in one afternoon of three to four hours?

The two questions share the topic of Rust browsers, but they have different goals. Thus we put them in different segments. Segment 01 holds the first question. Segments 02 to 06 hold the second question.

## Finding 2: The composite scores in the report do not agree with its weights

The report gives a weight to each metric. It then gives a composite score to each option. We calculated the scores again from the weights and the table values. No score agrees.

| Option | Score in report | Score from the weights |
|---|---|---|
| A: terminal browser | 4.75 | 4.80 |
| B: embedded webview shell | 3.65 | 3.60 |
| C: toy engine | 2.40 | 2.55 |
| D: Servo extension | 1.50 | 1.55 |

The order of the options does not change. But the error shows that nobody examined the numbers. Segment 06 must calculate the scores again.

## Finding 3: The report gives numbers without a source

The report gives these numbers but no measurement or source for them:

- LLM failure rates, for example "less than 10%" and "70% to 85%".
- Cold compile times, for example "45 to 90 seconds". A cold compile is the first build, with no saved build files.
- Web compatibility, for example "99%".
- Lines of code for each option.

Each segment lists its numbers as claims to test. Do not use a number from the report as a fact until a test makes sure of it.

## Finding 4: The two report files number the tracks differently

The Markdown report calls the webview shell "Track 1" and the terminal browser "Track 2". The HTML page uses the opposite order. In this repository, we use segment numbers and not track numbers.

## Finding 5: The code blueprints have not been compiled

A blueprint is a sample program in the report. We copied the two blueprints from the Markdown report into the segment folders. We did not change the code.

- The webview blueprint moves the `webview` value into a new thread. In Rust, a value can go to another thread only when its type has the `Send` property. We think that the wry `WebView` type does not have this property. If so, the blueprint does not compile. Segment 02 must make sure of this.
- The HTML page has a shorter terminal-browser blueprint. It does not load a first page at start. We used the longer Markdown version.

## Finding 6: Some content is old or weak

- The HTML page makes prompts for "Claude 3.5 Sonnet or GPT-4o". These models are old on the date of this review.
- The report says that winit 0.30 changed its event loop. But the webview blueprint uses tao and not winit. Tao is a separate crate. Segment 02 must make sure which tao API the blueprint needs.
- Many sources in the report are forum posts and blog posts, for example Reddit and DEV Community. Segment work must find a primary source for each important claim. A primary source is the project documentation, the source code or a release note.
- The citation numbers in the report touch the words before them, for example "APIs1". This makes the text hard to read.

## Finding 7: The HTML page needs internet access

The HTML page loads Tailwind CSS and Chart.js from content delivery networks. If the computer has no internet access, the page shows no charts and no styles.

## How we divided the content

| Segment | Research question | Content from |
|---|---|---|
| [01 LLM web operation](../segments/01-llm-web-operation/README.md) | Can an LLM operate a website through an existing browser? | Discussion, README |
| [02 Webview shell](../segments/02-webview-shell/README.md) | Can we build a Rust browser shell on an existing web engine? | Discussion, report |
| [03 Terminal browser](../segments/03-terminal-browser/README.md) | Can an LLM write a terminal browser in Rust in one afternoon? | Report |
| [04 Toy engine](../segments/04-toy-engine/README.md) | Can an LLM write a small rendering engine from the start? | Report |
| [05 Servo shell](../segments/05-servo-shell/README.md) | Can we build a browser shell on the Servo engine? | Report |
| [06 LLM Rust generation](../segments/06-llm-rust-generation/README.md) | Which methods help an LLM write Rust code that compiles? | Report |

We kept the source files without change in `docs/sources/`. They are records. The segment documents carry their facts in ASD-STE100 Simplified Technical English.

## Workday report review added on 2026-10-05

The earlier findings above concern the original discussion and feasibility files. This addition examines `deep-research-report.md`, now preserved without edits at [docs/sources/2026-10-05-workday-chrome-automation-report.md](../sources/2026-10-05-workday-chrome-automation-report.md). Its SHA-256 before and after the move is `8cf7d3ef02e0366ad43dc6cf31f2ece84cbdbe65e38d559ea509cb8f5b6a8dfb`.

The new report proposes Chrome automation and Workday extraction. It adds useful design questions, but does not replace the original general website-operation goal or the Rust construction experiments. The phrase "reject for this project" in the report applies to its Workday architecture, not to all educational use of Servo or toy engines.

The derived files are [current recommendations](../recommendations.md), [segment 07: Rust Chrome controller](../segments/07-rust-chrome-controller/README.md) and [segment 08: Workday data access](../segments/08-workday-data-access/README.md). The README indexes them. Recommendations have one home; segment notes hold experiments and evidence requirements. The earlier six segment goals remain.

## Findings 8 to 14: New report defects

| Finding | Source problem | Resolution in derived notes |
|---|---|---|
| 8: Unresolved references | The report contains internal ChatGPT citation and file-reference tokens. Its source map gives subjects, not usable source URLs. | Preserve tokens only in the original. Derived notes link primary sources and label proposals and gaps. |
| 9: Export can report false success | The example accepts a visible Export label or any page revision change. The label may already exist; unrelated changes do not prove export. | Require an action-correlated download event, completed file and expected content checks. |
| 10: Virtual scrolling may not advance | Scrolling an already visible last row can do nothing. A virtual grid can keep a fixed row count. | Advance the actual container; track stable record IDs and pagination progress. A bounded no-progress stop is incomplete unless completion is proved. |
| 11: Read-only policy is not enforced by generic tools | A click, fill or select can submit a write. The report does not define concrete allowed workflow capabilities. | Require specific permitted operations, least-privilege access and rejection of unknown actions. Page revisions protect target freshness, not action permission. |
| 12: WQL cache lifetime is omitted | Pagination discussion omits query-result expiry, which can affect long extractions. | Include the documented maximum 30-minute user-session cache lifetime. Test checkpoints, expiry recovery, restart and reconciliation; do not assume offsets remain consistent across a new result. |
| 13: Benchmark counts failures as timing samples | `curl -sS` does not reject HTTP errors. The sample discards bodies, so a failed operation can appear successful. | Verify transport, HTTP status and operation result. Report failed samples separately from verified successful latency. No runnable harness is claimed. |
| 14: Action reliability does not establish task reliability | A 99% action target is insufficient evidence for a multi-step workflow or correct extraction. | Measure full task success and data agreement directly. The independent-step example illustrates compounding, not measured Workday behavior. |

The report's JSON API, Rust trait, scroll algorithm and benchmark scripts are illustrative design sketches. They are uncompiled and unexecuted in this repository. No proposed service endpoint exists here. Its schedule and latency, memory, token and concurrency numbers remain untested planning estimates.

## Primary-source checks and limits

Primary sources were checked on 2026-10-05. [Playwright locators](https://playwright.dev/docs/locators), [actionability](https://playwright.dev/docs/actionability) and [Playwright MCP](https://github.com/microsoft/playwright-mcp) support the existing-browser baseline. The CDP [Accessibility](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/), [DOMSnapshot](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/), [Network](https://chromedevtools.github.io/devtools-protocol/tot/Network/) and [Browser](https://chromedevtools.github.io/devtools-protocol/tot/Browser/) documentation supports investigation of observations, responses and downloads.

[Chrome's remote-debugging announcement](https://developer.chrome.com/blog/remote-debugging-port) supports the dedicated user-data-directory requirement for affected switches from Chrome 136 and describes the Chrome for Testing exception. It does not establish permission to attach to a particular enterprise session.

[Workday WQL guidance](https://doc.workday.com/admin-guide/en-us/reporting-and-analytics/custom-reports-and-analytics/workday-query-language-wql-/sfx1612553126122.html) supports `limit`/`offset`, the up-to-10,000-row limit and maximum 30-minute cached user-session result. The [WQL/RaaS comparison](https://developer.workday.com/documentation/GUID-8f1d3acf-87ba-4de7-9dc5-84e3991be6a2-enHYPHENus/ReferenceWQLandRaaSComparisons) says RaaS has no pagination. These sources do not prove tenant availability or successful recovery after expiry.

The [public site terms](https://www.workday.com/en-us/legal/site-terms.html) concern defined Sites. The [customer contract framework](https://www.workday.com/en-us/legal/universal-contract-terms-and-conditions/index.html) identifies the importance of documents referenced in the Order Form or UMSA. The derived notes require determining the applicable access conditions; they do not apply public-site terms indiscriminately to all tenants.

[chromiumoxide](https://github.com/mattsse/chromiumoxide), [cdp-rs](https://github.com/oh0123/cdp-rs) and [playwright-rs](https://github.com/padamson/playwright-rust) remain backend candidates. The last repository is named `playwright-rust`, but its crate is `playwright-rs`. No checked source establishes a comparative Workday benchmark or justifies selecting direct CDP now.

## Remaining evidence gaps

- No website baseline, Rust comparison or Workday sandbox task has been run in this repository.
- The required task, field coverage, tenant interfaces, permissions, applicable contracts and reference results are unknown.
- Workflow capability enforcement, timeouts after an action takes effect, download correlation and pagination expiry recovery need experiments.
- Full-task reliability, sample sizes, maintenance cost, resource use and export correctness need measurement.
- Data destinations, retention, model visibility and rate/concurrency limits remain task-specific conditions.

The derived notes use clear, simple language. They are research plans, not an approved implementation specification or a claim of tested behavior.
