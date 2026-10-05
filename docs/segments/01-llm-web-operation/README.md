# Segment 01: LLM web operation

Status: not started.

## Research question

Can an LLM operate a website through an existing browser, without a new browser program?

An LLM (large language model) is an AI system that reads and writes text. To operate a website, the LLM must read the page, use its controls and make sure of the result.

## Why this segment is first

The discussion recommends this segment as the starting point. The user wants an LLM that can move through a website. This segment tests that goal directly. If an existing browser is sufficient, we do not need a Rust browser for this goal.

The Workday report proposes a Rust controller around Chrome, with Playwright as the interaction baseline. That proposal extends this segment; it does not show that the baseline or a Rust candidate has completed a Workday task. Current project choices belong in [recommendations](../../recommendations.md).

## Terms

- **Playwright:** a tool that controls a browser from a program. It can click, type and read a page.
- **MCP (Model Context Protocol):** a standard way to connect an LLM to tools.
- **Accessibility tree:** a structure that the browser makes from a page. It gives each control a role, a name and a state. Screen readers use it.
- **Role:** the type of a control, for example "button" or "text box".
- **Accessible name:** the label of a control, for example "Search".
- **Locator:** a Playwright rule that finds a control, for example by its role and name.
- **SPA (single-page application):** a website that changes its visible state or route without loading a whole new document.

## What the sources say

- An LLM can use a loop of four steps. It inspects the page, chooses an action, does the action and makes sure of the result.
- Playwright can find controls by their accessible role and name.
- Playwright MCP gives the LLM structured snapshots of the accessibility tree. These snapshots are easy for an LLM to read.
- Screenshots are still useful. They show the appearance of a page and the content of a canvas element. A canvas element is a drawing area whose visual contents may not appear in the accessibility tree.
- The OpenAI browser integration supports built-in browsing and connected browser profiles.
- The earlier Rust-app work read startup values through a debugger. It did not show reliable movement through the accessibility tree of that app.

Sources: `docs/sources/2026-10-05-discussion.md` and the project `README.md`. The [Workday report](../../sources/2026-10-05-workday-chrome-automation-report.md) adds architecture proposals and benchmark hypotheses, not measured results. Its opaque citation markers are preserved source text, not independently checked references.

## Arguments for and against this route

- **For website automation:** an existing browser tests real page actions immediately. Playwright role/name locators and actionability checks provide a baseline for finding controls and waiting before acting. Accessibility and targeted DOM state give the LLM semantics; screenshots help when those representations are insufficient.
- **Against treating the baseline as a finished system:** authentication, changing page state, downloads and ambiguous controls still need workflow-specific handling. A successful connection to the browser or debugger proves access, not a completed business task.
- **Hypothesis for Rust:** a narrow controller could improve packaging, observability or deterministic extraction. It must show that advantage on the same tasks without losing the baseline’s reliability.

## Claims to test

| Claim | How to test it |
|---|---|
| An accessibility snapshot is sufficient for the LLM to find the controls. | Do the experiment below. Record each control that the snapshot does not show. |
| Role and name locators are reliable. | Do the same workflow three times. Record each locator that fails. |
| Semantic state usually supports the next action; screenshots cover remaining gaps. | Record each step where accessibility or DOM state was insufficient and a screenshot helped. |
| The system completes tasks, rather than only dispatching actions. | Declare each task’s result in advance and check it after the last action. Count completed tasks and recovery interventions. |

## Experiment

1. Choose one website and one small workflow. Write down the expected result.
2. Open the page through an existing browser integration, for example Playwright MCP.
3. Find each control by its accessible role and name.
4. Do the actions of the workflow.
5. Make sure that the page state agrees with the expected result, for example the requested report is displayed with the intended filter. A click returning successfully or a debugger value is insufficient. Take a screenshot if it helps.
6. Record each gap in the interaction, action-to-result time, retries, human recovery, model round-trips and screenshot use. Report complete verified tasks as the main success measure.
7. Add local fixtures with a delayed control, a SPA route change and DOM replacement. Test re-observation after a stale target and explicit result waits; a page can remain network-active after its visible result is ready.
8. If a Rust candidate is needed, run identical logical scenarios with the same result checks. Separate warm and cold runs, browser time and controller overhead. Follow [segment 07](../07-rust-chrome-controller/README.md) for the controller comparison and [segment 08](../08-workday-data-access/README.md) for authorised Workday scenarios.

These additions are experiments proposed by the Workday report. No results are recorded yet.

## Decision

- Stop this feasibility experiment when the chosen workflows complete with verified results and the important gaps are understood. Use the existing browser baseline when it meets the task.
- Advance a Rust controller only when identical scenarios show a concrete advantage without worse task completion. A lower primitive click latency alone is insufficient. Keep the selection in [recommendations](../../recommendations.md).
- If the need is an owned desktop window or platform integration, examine [segment 02](../02-webview-shell/README.md). That is a separate shell goal.
- If the gaps come from website semantics, authentication or loading behaviour, record them and test a targeted remedy. A new rendering engine does not by itself repair them.

## Open questions

- Is the goal to learn Rust webviews, to automate websites or to build a browser product? Each goal can need a different implementation.
- Does the existing browser integration cover the chosen workflow?

## References

- [Playwright locators](https://playwright.dev/docs/locators)
- [Playwright actionability checks](https://playwright.dev/docs/actionability)
- [Playwright MCP](https://github.com/microsoft/playwright-mcp)
- [OpenAI browser integration](https://learn.chatgpt.com/docs/chrome-extension)
