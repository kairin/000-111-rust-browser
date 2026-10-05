# Rust Browser Learning

Learning notes about how an LLM can read and operate a website, and where Rust can help with a browser shell or controller. Workday is one proposed use case. It does not define the scope of all the research.

An LLM (large language model) is an AI system that reads and writes text, for example Claude. A browser shell is the window, menus and controls around a web engine.

| Field | Value |
|---|---|
| Project name | `000-111-rust-browser` |
| Manifest section | 3 - Tier 111 - Learning |
| Status | Discussion and learning notes. No browser implementation. |
| Discussion date | 2026-10-05, Asia/Singapore |
| Suggested GitHub description | Explores Rust browser shells and semantic browser automation so LLMs can inspect pages, operate controls and verify results. |

## Research segments

We divided the content into eight segments. Each segment asks one research question. Feasibility is a measure of whether a task is possible with the time, tools and skill that we have.

| Segment | Research question |
|---|---|
| [01 LLM web operation](docs/segments/01-llm-web-operation/README.md) | Can an LLM operate a website through an existing browser? |
| [02 Webview shell](docs/segments/02-webview-shell/README.md) | Can we build a Rust browser shell on an existing web engine? |
| [03 Terminal browser](docs/segments/03-terminal-browser/README.md) | Can an LLM write a terminal browser in Rust in one afternoon? |
| [04 Toy engine](docs/segments/04-toy-engine/README.md) | Can an LLM write a small rendering engine from the start? |
| [05 Servo shell](docs/segments/05-servo-shell/README.md) | Can we build a browser shell on the Servo engine? |
| [06 LLM Rust generation](docs/segments/06-llm-rust-generation/README.md) | Which methods help an LLM write Rust code that compiles? |
| [07 Rust Chrome controller](docs/segments/07-rust-chrome-controller/README.md) | Does a Rust controller improve website operation compared with the Playwright baseline? |
| [08 Workday data access](docs/segments/08-workday-data-access/README.md) | Which approved browser or data interface fits a Workday task? |

Start with segment 01. It tests the main goal directly. Segments 02 to 06 retain the separate Rust browser learning experiments. Segments 07 and 08 examine the new Chrome and Workday report. All eight segments are research plans, not completed experiments.

## Repository layout

| Folder | Content |
|---|---|
| `docs/segments/` | One folder for each segment. Segments 02 and 03 also hold a code blueprint from the report. |
| `docs/recommendations.md` | The [current recommendations](docs/recommendations.md), arguments for each goal, decision conditions and open questions. |
| `docs/review/` | The [document review](docs/review/2026-10-05-document-review.md). It records the problems in the source documents. |
| `docs/sources/` | The original documents, without change. |

The source documents are:

- [The saved discussion](docs/sources/2026-10-05-discussion.md). It holds the questions of the user and the response of the assistant. The discussion started in `000-222-rust-feh`, but this learning project has a separate scope.
- [The feasibility report (Markdown)](docs/sources/2026-10-05-rust-browser-llm-feasibility-report.md). It examines whether an LLM can write a Rust web browser in one afternoon.
- [The feasibility report (HTML)](docs/sources/2026-10-05-rust-browser-llm-feasibility-report.html). It is an interactive page of the same report. Open it in a web browser with internet access.
- [The Workday Chrome automation report](docs/sources/2026-10-05-workday-chrome-automation-report.md). This is the original `deep-research-report.md`, moved without edits. Its internal citation tokens remain part of the source record; the review records their limits.

## Working recommendation

Prove one complete website workflow with an existing browser and Playwright first. Compare a Rust controller only when the same representative workflows can show an advantage and an acceptable maintenance cost. See [current recommendations](docs/recommendations.md) for the full comparison, including the separate learning goals and the conditional Workday extraction route.

## Manifest inclusion

The [classification rules](https://github.com/kairin/000-0-manifest/blob/main/docs/local-sync.md#classification-and-new-repositories) of the manifest put active original repositories that start with `000-111-` in the Learning section. A script makes the repository tables of the manifest. Do not add a table row by hand.

This project is a public Learning repository at [kairin/000-111-rust-browser](https://github.com/kairin/000-111-rust-browser). It uses the GitHub description above. The manifest script classifies a public active original repository with the `000-111-` prefix automatically. But you must refresh and examine the inventory of the manifest to include a new repository. A new private or internal repository needs an entry in the `inventory-config.json` file of the manifest.

## References

- [Playwright locators](https://playwright.dev/docs/locators): find controls by role and accessible name.
- [Playwright MCP](https://github.com/microsoft/playwright-mcp): structured accessibility snapshots and browser actions. It also discusses command-line alternatives.
- [Wry](https://github.com/tauri-apps/wry): a Rust webview library that uses existing platform engines. On Linux, it uses WebKitGTK.
- [OpenAI browser integration](https://learn.chatgpt.com/docs/chrome-extension): built-in browsing and connected browser profiles.
