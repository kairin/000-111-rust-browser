# Segment 04: Toy engine

Status: not started.

## Research question

Can an LLM write a small web rendering engine in Rust from the start, in one afternoon of three to four hours?

A rendering engine reads HTML and CSS and then draws the page. A toy engine supports only a small part of HTML and CSS. It is for learning.

## Terms

- **DOM (Document Object Model):** a tree of nodes that the engine makes from HTML.
- **CSSOM (CSS Object Model):** a structure that the engine makes from CSS.
- **Style tree:** a tree that joins each DOM node to its CSS rules.
- **Layout:** the step that calculates the position and size of each box on the page.
- **Rasterizer:** the step that changes boxes into pixels.
- **Arena:** a vector that holds all nodes. Each node refers to other nodes by an index number, not by a pointer.

## What the sources say

- Matt Brubeck wrote robinson, a toy engine in Rust. Addy Osmani wrote rust-browser, a newer version of the same idea.
- The engine has these phases: parse HTML into a DOM, parse CSS into a CSSOM, match selectors to make a style tree, calculate layout, then draw pixels.
- The DOM module needs about 150 lines of code.
- Layout is the hardest step. It must calculate block flow, inline text size, line wrap and margin collapse.
- This toy engine supports a deliberately small subset of HTML and CSS. Static test pages are its useful scope; compatibility with arbitrary real websites is not established.
- DOM nodes refer to parents, siblings and children. In Rust, these references can cause conflicts with the borrow checker.
- LLMs often use `Rc<RefCell<Node>>` for these references. This can fail at compile time or stop the program at run time.
- An arena with `NodeId` index numbers can simplify ownership. It still needs valid indices and correct traversal; the report says LLMs often lose this structure in long programs.
- The report rejects this route because the risk of borrow-checker problems is too high.

Source: `docs/sources/2026-10-05-rust-browser-llm-feasibility-report.md`. Its sizes and failure rates are source claims to test, not measurements from this project.

## Arguments for and against this route

- **For engine learning:** owning the parser, style matching and layout makes their data contracts and Rust ownership choices visible. An arena is a useful design hypothesis for this particular graph.
- **Against near-term Workday use:** browser compatibility is a separate, much larger task involving JavaScript, layout, authentication and browser APIs. A compiled parser or rendered static page shows no direct gain for the workflows in the [Workday case study](../08-workday-data-access/README.md).
- **Boundary:** the [Workday report](../../sources/2026-10-05-workday-chrome-automation-report.md) proposes using Chrome’s existing engine. That proposal leaves this learning experiment useful; project adoption choices belong in [recommendations](../../recommendations.md).

## Claims to test

The report gives these numbers without a source.

| Claim | Report value | How to test it |
|---|---|---|
| Lines of code | 1,500 to 3,500 | Count the lines of robinson and rust-browser. |
| Cold compile time | 2 to 5 minutes | Build robinson once with no saved build files. |
| LLM failure rate | 70% to 85% | Ask an LLM to write one phase at a time. Record where it fails. |
| Composite score | 2.40 of 5 | Calculate again in segment 06. The weights give 2.55. |

## Experiment

1. Read robinson and rust-browser. Count the lines of each phase.
2. Ask an LLM to write only the DOM and HTML parser with an arena. Build it.
3. Ask the LLM to add the CSS parser and the style tree. Build it again.
4. Record each type mismatch between phases. Fix the DOM, style and layout interfaces explicitly before expanding the supported syntax.
5. Use small static fixtures with known node trees, matched styles and expected box positions. A build alone does not check these results.
6. Stop at the end of the afternoon. Record the last phase that compiles and the supported HTML/CSS subset. Do not add enterprise website compatibility to this time-bounded experiment.

## Decision

- Continue for parser, style and layout learning within the declared subset. Stop this experiment at the afternoon boundary and preserve partial results.
- If the LLM reaches working layout in one afternoon, record evidence against the report’s rejection for that scope. This does not establish general browser compatibility.
- Do not advance it for near-term Workday automation without a separate compatibility programme and a demonstrated advantage over segment 01. See [segment 07](../07-rust-chrome-controller/README.md) for the controller path.

## References

- [robinson](https://github.com/mbrubeck/robinson)
- [rust-browser README](https://github.com/addyosmani/rust-browser/blob/main/README.md)
