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
- A toy engine cannot reliably show real websites. It is useful only for static test pages.
- DOM nodes refer to parents, siblings and children. In Rust, these references cause conflicts with the borrow checker.
- LLMs often use `Rc<RefCell<Node>>` for these references. This can fail at compile time or stop the program at run time.
- An arena with `NodeId` index numbers avoids these conflicts. But LLMs often lose this structure in long programs.
- The report rejects this route because the risk of borrow-checker problems is too high.

Source: `docs/sources/2026-10-05-rust-browser-llm-feasibility-report.md`.

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
4. Record each type mismatch between phases.
5. Stop at the end of the afternoon. Record the last phase that compiles.

## Decision

- This segment is for learning, not for a product. A toy engine cannot operate real websites.
- If the LLM reaches layout in one afternoon, the report claim is too strong. Record the result.

## References

- [robinson](https://github.com/mbrubeck/robinson)
- [rust-browser README](https://github.com/addyosmani/rust-browser/blob/main/README.md)
