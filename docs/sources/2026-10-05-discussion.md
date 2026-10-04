# Rust browser and LLM interaction: saved discussion

Date: 2026-10-05 (Asia/Singapore).

Source: the conversation about the Rust image application in `000-222-rust-feh`. This record keeps the browser question, the response and the related earlier user request. It is a discussion record. It is not an approved implementation specification. The quoted text is the original text, without change.

## Related earlier request

User:

> is there any way to make the menu and functions of the app something that llm can easily walk through, like as if using playwright in chromium. but llm can directly interact.

Context: the recent Rust-app launch verification inspected startup tool capabilities through a debugger. Reliable navigation through the app's accessibility tree had not been demonstrated. That distinction carries into the browser discussion below.

## Browser question

User:

> help me think through. since llms like you can walk through the rust based app here, can we build a browser in rust such that you can walk through the website more easily? or how else can you walk through a web browser loading a web page?

Assistant:

Yes—we can build a browser shell in Rust that an LLM can control. **The main advantage would come from its automation interface, rather than Rust itself.** My recommendation is to use an existing browser engine and expose clear inspection and action tools.

One clarification: our recent Rust-app verification inspected startup values through a debugger. We haven’t yet demonstrated reliable navigation through its accessibility tree.

For websites, an LLM can follow this loop:

1. Inspect the page’s controls, text, names and current states.
2. Choose an action, such as clicking **“Search”** or filling **“Email.”**
3. Execute that action through a browser tool.
4. Inspect the resulting page and verify the outcome.

Playwright supports locating controls by their accessible role and name. Its MCP integration exposes structured accessibility snapshots, which give an LLM a readable representation of the page. Screenshots remain useful for checking appearance and canvas content. [Playwright locators](https://playwright.dev/docs/locators), [Playwright MCP](https://github.com/microsoft/playwright-mcp)

There are three practical approaches:

| Approach | When it makes sense |
|---|---|
| **Existing browser integration** | Let me work with websites through the available browser connection. OpenAI’s browser integration supports built-in browsing and connected browser profiles. |
| **Chromium with Playwright CLI or MCP** | Build reusable website workflows with page inspection, clicks, typing and outcome verification. This is my recommended starting point. |
| **Rust browser shell with an automation interface** | Build your own browser application when you need custom menus, workflows or integration with other Rust software. |

[OpenAI browser integration](https://learn.chatgpt.com/docs/chrome-extension)

A Rust shell could use an existing webview library such as Wry. On Linux, Wry uses WebKitGTK; choosing it would therefore involve a different engine from Chromium. [Wry documentation](https://github.com/tauri-apps/wry)

For your goal, I would first prove one complete website workflow using an existing browser: inspect the page, find controls, perform actions and verify the result. That establishes what the LLM actually needs. If a custom Rust browser remains useful, we can then build its interface around those proven capabilities—without taking on the considerable work of creating a browser engine.

## Request to save the discussion

User:

> save our communications regarding rust browser into rust-browser use the relevant naming schema under tier 3 for learning 3 - Tier 111 - Learning so that we can include into https://github.com/kairin/000-0-manifest

The manifest's `000-111-` prefix makes the corresponding project name `000-111-rust-browser`. See the [README](../../README.md) for the concise recommendation and future manifest inclusion steps.
