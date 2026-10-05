# Current recommendations

Date: 2026-10-05. Status: research decisions to test. No browser or controller has been implemented in this repository.

The main goal is to help an LLM read a website, use its controls and verify the result. Use an existing browser with Playwright as the first baseline. A Rust controller becomes a candidate when the same representative workflows show a useful advantage and an acceptable maintenance cost. Rust browser construction remains a separate learning goal.

## Compare the eight goals

| Segment and goal | Argument for it | Limit or argument against it | Current decision |
|---|---|---|---|
| [01 Website operation](segments/01-llm-web-operation/README.md) | Tests the original user goal directly with an existing browser. | Some controls need DOM inspection or screenshots; a page can still be inaccessible. | First experiment. Record gaps and full task outcomes. |
| [02 Webview shell](segments/02-webview-shell/README.md) | Teaches Rust windows, engine embedding and custom controls. | Native setup, changing APIs and platform engine differences add work. The sample has not been compiled. | Keep as learning research; justify a shell through a specific need. |
| [03 Terminal browser](segments/03-terminal-browser/README.md) | A small program can teach HTTP, text conversion and terminal interfaces. | No JavaScript or modern page layout; limited fit for interactive websites. | Keep the one-afternoon generation experiment; do not treat it as the website-operation solution. |
| [04 Toy engine](segments/04-toy-engine/README.md) | Teaches parsing, ownership, style and layout. | A small engine cannot operate normal modern websites. | Keep as an educational experiment. |
| [05 Servo shell](segments/05-servo-shell/README.md) | Teaches a Rust engine and its embedding model. | Setup and engine compatibility need separate investigation. | Keep the longer-term engine question; do not select it for Workday from the reports. |
| [06 LLM Rust generation](segments/06-llm-rust-generation/README.md) | Tests methods that may reduce compile errors. | Earlier scores and failure rates lack sufficient evidence. | Measure the methods; retain corrected arithmetic and uncertainty. |
| [07 Rust Chrome controller](segments/07-rust-chrome-controller/README.md) | Could provide compact observations, workflow controls and reusable exports around Chrome. | Direct CDP adds locator, waiting, recovery and packaging work already handled by Playwright. | Compare candidates against the baseline before selecting a backend. |
| [08 Workday data access](segments/08-workday-data-access/README.md) | An approved data interface could avoid repeated row-by-row navigation. | Access, fields, licensing, contracts and pagination depend on the tenant and task. | Discover permitted sources first; compare their results with the browser route. |

The feasibility report's terminal-browser preference concerns code generation in an afternoon. The saved discussion's browser preference concerns operating websites. The new report's Chrome/CDP proposal concerns Workday. These conclusions have different goals and are not interchangeable.

## Selection conditions

1. Write the workflow and its expected result. Use roles, names and page state in the Playwright baseline. Record cases that require screenshots.
2. Use the same logical tasks, fixture data and result checks for a Rust candidate. Consider `chromiumoxide`, `cdp-rs` and `playwright-rs` as candidates, not proven Workday solutions.
3. Compare complete task success, correct data, recovery from failures, maintenance and packaging. Measure latency, memory and model context as supporting evidence.
4. Select a Rust controller only if the comparison supports its benefit and cost. Otherwise continue with the baseline.
5. For Workday, use an approved documented API or report source when it covers the required fields, access and result checks. Use the browser for approved UI workflows or where no suitable data interface exists.

Playwright documents [locators](https://playwright.dev/docs/locators) and [actionability checks](https://playwright.dev/docs/actionability). [Playwright MCP](https://github.com/microsoft/playwright-mcp) supplies a relevant LLM interaction baseline. The repositories for [chromiumoxide](https://github.com/mattsse/chromiumoxide), [cdp-rs](https://github.com/oh0123/cdp-rs) and [playwright-rs](https://github.com/padamson/playwright-rust) support candidate investigation; they do not establish comparative Workday performance. Primary sources were checked on 2026-10-05.

## Proof stages

| Stage | Evidence to produce | Decision it supports |
|---|---|---|
| Baseline | One complete workflow, expected result, failures and observation gaps | What the LLM needs from a browser |
| Local comparison | Identical dynamic-page, download and failure scenarios | Whether a Rust controller is worth further work |
| Workday discovery | Permitted workflows, tenant capabilities, data fields and reference results | Whether an approved data source or UI route fits |
| Approved sandbox comparison | Full task success, export correctness, recovery and resource measurements | Whether a narrow Workday pilot is justified |
| Pilot review | Access controls, data handling, audit and maintenance evidence | Whether a separate implementation project is justified |

These stages have no promised delivery dates. The source report's calendar, latency, memory, token and concurrency values are planning estimates, not results from this repository.

## Open questions

- Which general website and workflow should be the first experiment?
- Is the next learning goal Rust programming, browser embedding, website operation or bulk extraction?
- What benefit would a custom controller need to show to justify its maintenance?
- Which Workday tenant, workflows, fields and reference reports are in scope, if this use case proceeds?
- Which documented data interfaces are enabled and permitted? How will pagination expiry and source changes be reconciled?
- What complete-task reliability and export correctness are required for the intended workflow?

## Evidence and source records

The [saved discussion](sources/2026-10-05-discussion.md) supplies the original website-operation goal. The [feasibility report](sources/2026-10-05-rust-browser-llm-feasibility-report.md) supplies the separate browser construction experiments. The [Workday report](sources/2026-10-05-workday-chrome-automation-report.md) supplies the controller and extraction proposals. The [document review](review/2026-10-05-document-review.md) distinguishes source defects, verified references and remaining gaps. Segment notes hold the detailed experiments; this page holds the current recommendations.
