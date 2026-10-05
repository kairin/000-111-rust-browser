# Segment 03: Terminal browser

Status: not started.

## Research question

Can an LLM write a working terminal browser in Rust in one afternoon of three to four hours?

A terminal browser shows web pages as text in a terminal window. Lynx, Links and w3m are older examples.

## Terms

- **TUI (text user interface):** a program interface that uses text characters in a terminal.
- **ratatui:** a Rust crate that draws a TUI layout. It replaced the older `tui-rs` crate.
- **crossterm:** a Rust crate that controls the terminal and reads key presses.
- **reqwest:** a Rust crate that downloads web pages over HTTP.
- **rustls:** a TLS library written in Rust. TLS encrypts the connection for HTTPS.
- **html2text:** a Rust crate that changes HTML into wrapped plain text.

## What the sources say

- The browser downloads a page, changes the HTML into text and draws the text in the terminal.
- html2text keeps paragraphs, list items, links and simple tables as text. It wraps the text to a given column width.
- With the `rustls-tls` feature, reqwest does not need OpenSSL. Thus the build needs no C library.
- The browser does not run JavaScript. It does not apply CSS grid or flexbox layout.
- The report recommends this route as the most feasible afternoon Rust-generation route. Its stated score is 4.75 of 5; segment 06 corrects the arithmetic to 4.80. This is not a ranking of website automation capability.
- The report says that an LLM can write the browser in one pass without borrow-checker errors. The borrow checker is the part of the Rust compiler that makes sure that memory use is safe.

Source: `docs/sources/2026-10-05-rust-browser-llm-feasibility-report.md`. Its feasibility values are source-derived hypotheses, not measured results.

## Arguments for and against this route

- **For learning and an afternoon build:** downloading HTML, converting it to text and handling terminal input create a small Rust project with clear boundaries. Static documentation and simple link traversal are useful test cases.
- **Against enterprise website automation:** this blueprint has no JavaScript execution or browser layout. It cannot exercise the client-side controls and SPA transitions required by a modern Workday-style workflow. A high generation score cannot establish otherwise.
- **Boundary:** the [Workday report](../../sources/2026-10-05-workday-chrome-automation-report.md) proposes controlling an existing browser for these interactions. Keep terminal-browser learning separate from the controller research in [segment 07](../07-rust-chrome-controller/README.md) and the choices in [recommendations](../../recommendations.md).

## Claims to test

The report gives these numbers without a source.

| Claim | Report value | How to test it |
|---|---|---|
| Lines of code | 250 to 450 | Count the lines of a working browser. |
| Cold compile time | 45 to 90 seconds | Build once with no saved build files. Record the time and the computer. |
| LLM failure rate | less than 10% | Ask an LLM to write the browser five times. Count the attempts that do not compile. |
| Composite score | 4.75 of 5 | Calculate again in segment 06. The weights give 4.80. |

## Blueprint

The folder [blueprint/](blueprint/) holds the sample program from the Markdown report, without change. It is an uncompiled source example, not a validated implementation. It pins crossterm 0.28.1, ratatui 0.29.0, reqwest 0.12.9, tokio 1.40.0 and html2text 0.13.6.

The HTML report has a shorter copy of this program. That copy does not load a first page at start. We did not use it.

## Experiment

1. Build the blueprint without changes. Record each compile error and the build time.
2. Open three pages: a news page, a documentation page and a page that needs JavaScript.
3. Record how each page looks as text.
4. Give the pinned `Cargo.toml` to an LLM. Ask it to write the browser again from the start.
5. Do step 4 five times. Record the compile result each time.
6. Compare the JavaScript-dependent page with the existing-browser baseline in segment 01. Record which controls and result states the terminal version omits; missing interaction is a scope limitation, not a compile failure.
7. Stop after the afternoon. Record working navigation, error handling and terminal restoration separately from first-attempt compilation.

## Decision

- If the blueprint builds and the LLM succeeds in most attempts, record support for the narrow afternoon-generation hypothesis. Five attempts do not establish a general failure rate.
- Keep this route for static text browsing and Rust learning. Stop using it as an automation candidate when the required workflow depends on JavaScript or rendered layout.
- Record whether the text-only interface helps any part of segment 01, but do not infer that its feasibility score makes it the best browser automation route.

## References

- [html2text README](https://github.com/jugglerchris/rust-html2text/blob/main/README.md)
- [html2text documentation](https://docs.rs/html2text/)
- [feedr, a terminal feed reader in Rust](https://github.com/bahdotsh/feedr)
