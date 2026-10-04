# Segment 01: LLM web operation

Status: not started.

## Research question

Can an LLM operate a website through an existing browser, without a new browser program?

An LLM (large language model) is an AI system that reads and writes text. To operate a website, the LLM must read the page, use its controls and make sure of the result.

## Why this segment is first

The discussion recommends this segment as the starting point. The user wants an LLM that can move through a website. This segment tests that goal directly. If an existing browser is sufficient, we do not need a Rust browser for this goal.

## Terms

- **Playwright:** a tool that controls a browser from a program. It can click, type and read a page.
- **MCP (Model Context Protocol):** a standard way to connect an LLM to tools.
- **Accessibility tree:** a structure that the browser makes from a page. It gives each control a role, a name and a state. Screen readers use it.
- **Role:** the type of a control, for example "button" or "text box".
- **Accessible name:** the label of a control, for example "Search".
- **Locator:** a Playwright rule that finds a control, for example by its role and name.

## What the sources say

- An LLM can use a loop of four steps. It inspects the page, chooses an action, does the action and makes sure of the result.
- Playwright can find controls by their accessible role and name.
- Playwright MCP gives the LLM structured snapshots of the accessibility tree. These snapshots are easy for an LLM to read.
- Screenshots are still useful. They show the appearance of a page and the content of a canvas element. A canvas element is a drawing area that has no accessibility tree.
- The OpenAI browser integration supports built-in browsing and connected browser profiles.
- The earlier Rust-app work read startup values through a debugger. It did not show reliable movement through the accessibility tree of that app.

Sources: `docs/sources/2026-10-05-discussion.md` and the project `README.md`.

## Claims to test

| Claim | How to test it |
|---|---|
| An accessibility snapshot is sufficient for the LLM to find the controls. | Do the experiment below. Record each control that the snapshot does not show. |
| Role and name locators are reliable. | Do the same workflow three times. Record each locator that fails. |
| Screenshots are necessary only for appearance and canvas content. | Record each step where the LLM needed a screenshot, and why. |

## Experiment

1. Choose one website and one small workflow. Write down the expected result.
2. Open the page through an existing browser integration, for example Playwright MCP.
3. Find each control by its accessible role and name.
4. Do the actions of the workflow.
5. Make sure that the page state agrees with the expected result. Take a screenshot if it helps.
6. Record each gap in the interaction.

## Decision

- If the workflow succeeds and has no important gaps, use an existing browser for LLM web operation. Do not build a Rust browser for this goal.
- If the gaps come from the browser itself, examine segment 02.
- If the gaps come from the website, record them. A new browser does not repair a website.

## Open questions

- Is the goal to learn Rust webviews, to automate websites or to build a browser product? Each goal can need a different implementation.
- Does the existing browser integration cover the chosen workflow?

## References

- [Playwright locators](https://playwright.dev/docs/locators)
- [Playwright MCP](https://github.com/microsoft/playwright-mcp)
- [OpenAI browser integration](https://learn.chatgpt.com/docs/chrome-extension)
