# Rust Browser Learning

Learning notes on making websites easy for an LLM to inspect and operate, and where Rust could help build a browser shell or controller.

| Field | Value |
|---|---|
| Project name | `000-111-rust-browser` |
| Manifest section | 3 - Tier 111 - Learning |
| Status | Discussion and learning notes; no browser implementation |
| Discussion date | 2026-10-05, Asia/Singapore |
| Suggested GitHub description | Explores Rust browser shells and semantic browser automation so LLMs can inspect pages, operate controls and verify results. |

## Discussion record

Read [the saved conversation](docs/2026-10-05-discussion.md) for the user's questions and the assistant's response. The discussion originated in `000-222-rust-feh`, but this learning project has a separate scope.

## Working recommendation

Start with an existing browser and Playwright automation. An LLM needs readable page semantics and reliable actions: inspect, click, type, wait and verify. Rust can later provide a custom browser shell or controller if the experiment demonstrates a useful reason for one.

Use accessibility or DOM information to identify controls by role, name and state. Use screenshots to check appearance and content that is not represented adequately in those structures. A Rust implementation alone does not establish that an LLM can navigate an application reliably.

## First learning experiment

1. Choose one website and one small workflow, with an explicit expected result.
2. Inspect the page through an existing browser automation integration.
3. Find controls by their accessible role and name; perform the required actions.
4. Verify the resulting page state and capture a screenshot where useful.
5. Record any interaction gaps before deciding whether a Rust shell is needed.

This is a proposed experiment. It has not been carried out for this project.

## Questions to resolve

- Is the objective to learn Rust webviews, to automate websites, or to build a browser product? These goals may call for different implementations.
- Does the existing browser integration cover the chosen workflow?
- If a custom shell is useful, which browser engine and automation interface meet the workflow's needs?

## Manifest inclusion

The manifest's [classification rules](https://github.com/kairin/000-0-manifest/blob/main/docs/local-sync.md#classification-and-new-repositories) put active original repositories beginning `000-111-` in the Learning section. Its repository tables are generated; do not add a handwritten table row.

This project is published as [kairin/000-111-rust-browser](https://github.com/kairin/000-111-rust-browser), a public Learning repository, with the About description above. The manifest script classifies a public active original with the `000-111-` prefix automatically. Its generated inventory must still be refreshed and reviewed to include a new repository. A new private or internal repository would require an explicit entry in the manifest's `inventory-config.json`.

## References

- [Playwright locators](https://playwright.dev/docs/locators): locating controls by role and accessible name.
- [Playwright MCP](https://github.com/microsoft/playwright-mcp): structured accessibility snapshots and browser actions; also discusses CLI alternatives.
- [Wry](https://github.com/tauri-apps/wry): a Rust webview library using existing platform engines, including WebKitGTK on Linux.
- [OpenAI browser integration](https://learn.chatgpt.com/docs/chrome-extension): built-in browsing and connected browser profiles.
