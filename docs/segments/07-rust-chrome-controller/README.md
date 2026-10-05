# Segment 07: Rust Chrome controller

Status: not started. This is a research plan. No controller, service or API in this document has been implemented.

## Research question

Does a Rust controller around Chrome improve LLM website operation compared with an existing Playwright baseline? Which benefit would justify the extra maintenance?

This segment concerns general website operation. [Segment 08](../08-workday-data-access/README.md) applies it to the proposed Workday use case. Backend selection conditions are in [current recommendations](../../recommendations.md).

## Terms

- **CDP (Chrome DevTools Protocol):** commands and events that inspect and control Chrome.
- **Semantic observation:** a page description containing meaningful controls, names and states.
- **Page revision:** a controller-assigned identifier for the observed page state.
- **Postcondition:** evidence that an action produced its intended result.
- **Idempotent operation:** an operation whose repeat has the same effect as doing it once.

## Design questions from the report

The [source report](../../sources/2026-10-05-workday-chrome-automation-report.md) proposes a backend interface between browser libraries and the LLM tools. This could keep observations, workflow rules and exports independent of a crate. The interface would need observation, action, wait, download and session-state operations. Its cost and coverage have not been tested.

The report proposes these observation levels:

| Level | Information | What to test |
|---|---|---|
| Compact state | Route, title, busy state, alerts and result counts | Whether it supports the next decision |
| Accessibility tree | Roles, names, states and controls | Missing or ambiguous controls |
| Targeted DOM | Table structure and difficult controls | Data exposure and observation size |
| Screenshot | Visual-only content and layout ambiguity | When semantic observations fail |

Full HTML is a diagnostic option, not the default model input. Network metadata can help identify structured data sources; capturing bodies needs a data policy. An observed private endpoint is not automatically a supported extraction interface.

Chrome documents the [Accessibility](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/), [DOMSnapshot](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/) and [Network](https://chromedevtools.github.io/devtools-protocol/tot/Network/) domains. Their capabilities support investigation; coverage of a particular page must be measured.

## Action and session contract to test

An action needs a current target, a permitted workflow operation and an expected result. A page revision can reject a reference observed before a rerender. It does not prove that a control is safe to use.

| Contract | Required experiment |
|---|---|
| References identify an observed target and its revision | Replace a DOM node, then attempt an action through the old reference |
| A stale reference returns a typed conflict | Return `STALE_PAGE_REVISION`; observe again instead of guessing a replacement |
| Preconditions include route, target identity and relevant state | Change the route or disable the target before the action |
| Workflow capabilities constrain actions | Permit a specific search or export operation; reject an unknown submit control |
| Success requires the declared result | Trigger unrelated page changes and make sure that they do not count as success |
| Authentication expiry returns a distinct state | Expire a session and request human authentication handoff |

Generic `click`, `fill` or `select` methods can cause writes. A proposed read-only design needs specific allowed workflow capabilities, least-privilege application access and a rule to reject unknown operations. Test those controls independently of the model's intent. Page content supplies evidence, not new permissions. Keep credentials, cookies and arbitrary JavaScript execution outside the model's tool surface.

A managed session experiment should use an approved dedicated browser profile and human authentication when required. [Chrome's remote-debugging change](https://developer.chrome.com/blog/remote-debugging-port) requires a non-default user-data directory for the affected debugging switches from Chrome 136; Chrome for Testing is an exception described by Chrome. The exact browser arrangement remains an experiment choice.

## Waits, scrolling and downloads

Wait for the intended state, not only a load event, a revision change or network silence. A page can maintain a continuous connection. A revision can change because of an unrelated alert. Test route, expected content, busy state and relevant response conditions as appropriate to each action.

For a virtual list, scrolling an already visible row into view may do nothing. Advance the actual scroll container and record stable row IDs. A virtual list can replace rows without increasing the visible row count. Compare row identities, pagination markers and relevant responses. Stop on cancellation or a time/row bound. A bounded no-progress stop means **incomplete extraction** unless a reliable end marker or reference count proves completion.

An export requires an event correlated with the requested action and a verified completed file. An already visible Export label cannot prove that an export occurred. The [CDP Browser domain](https://chromedevtools.github.io/devtools-protocol/tot/Browser/) documents download events. Test completion, interruption, file validation and duplicate export requests; a download-start event alone is insufficient.

## Recovery and concurrency

| Situation | Behavior to investigate |
|---|---|
| Stale or detached target | Re-observe and resolve the allowed target again |
| Action timeout | Inspect the result before retrying; the action may already have occurred |
| Duplicate operation key | Return a recorded result where possible; do not claim exactly-once browser effects |
| Authentication or permission failure | Return a distinct state; avoid repeated login attempts |
| Rate or temporary server failure | Respect available server guidance and use bounded backoff |
| Partial download | Preserve its failed status and verify whether a retry is safe |
| Repeated same-state loop | Stop with an explicit incomplete or failed result |

Serialize actions within a tab. Independent tabs or data requests can be compared later, with configurable rate and concurrency controls. There is no assumed universal website or Workday rate limit. Cache observations and references by session and revision; do not reuse a target across a changed page.

## Comparison experiment

1. Complete the segment 01 baseline and record its expected task results.
2. Define identical local scenarios for Playwright and a Rust candidate: SPA navigation, a constant-size virtual grid, iframe/shadow content, delayed responses, continuous network activity, DOM replacement and a file download.
3. Include negative scenarios: unchanged Export label, unrelated revision change, a forbidden submit operation, an action that times out after taking effect, HTTP errors and a success response containing a failed operation.
4. Check both transport/HTTP status and the operation response. Include only verified successes in successful latency samples; report failures separately.
5. Measure complete task success, correct final state and data, action-to-result time, recovery, observation size, model round trips and separate browser/controller/adapter resources.
6. Record warm and cold runs, versions, fixture data, sample sizes and latency distributions. Report maintenance and packaging work too.

A 99% action success rate does not imply 99% full task success. For illustration, independent actions with 99% success each give about 61% success across 50 actions. Real failures can be correlated, so measure complete tasks directly.

The report's API JSON, backend trait, scroll algorithm and shell benchmarks are design sketches. They have not been compiled or executed. The proposed endpoints have no implementation in this repository. The experiments above specify what future work would need to prove; they are not a runnable harness.

## Evidence to record

Produce a scenario/result table, failure traces, export checks and resource measurements. Apply the comparison conditions in [current recommendations](../../recommendations.md) to decide whether further Rust work is justified. A shorter control command is useful only when complete workflow behavior and maintenance remain acceptable.

## References

Primary sources checked on 2026-10-05:

- [Playwright actionability](https://playwright.dev/docs/actionability) and [locators](https://playwright.dev/docs/locators)
- [Playwright MCP](https://github.com/microsoft/playwright-mcp)
- [chromiumoxide](https://github.com/mattsse/chromiumoxide), [cdp-rs](https://github.com/oh0123/cdp-rs), and [playwright-rs](https://github.com/padamson/playwright-rust)

These library sources describe candidates. They do not prove a performance or reliability advantage for a Workday workflow.
