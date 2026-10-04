# Segment 06: LLM Rust generation

Status: not started.

## Research question

Which methods help an LLM write Rust code that compiles on the first attempt? Is the decision matrix of the report correct?

This segment applies to segments 02, 03 and 04.

## Terms

- **Borrow checker:** the part of the Rust compiler that makes sure that memory use is safe.
- **API drift:** a change in a library interface between versions. An LLM can learn both the old and the new form, and then mix them.
- **Context window:** the amount of text that an LLM can use at one time.
- **Decision matrix:** a table that gives a weighted score to each option.

## Failure modes in the report

The report names three reasons why LLMs fail with Rust browser code.

1. **Tree ownership.** DOM nodes refer to parents, siblings and children. LLMs use `Rc<RefCell<Node>>` and then borrow the same node two times for change. This fails at compile time or stops the program at run time. An arena with `NodeId` index numbers avoids the problem.
2. **API drift.** Before winit 0.30, the event loop used a closure, for example `event_loop.run(move |event, _, control_flow| { ... })`. Version 0.30 uses the `ApplicationHandler` trait and `event_loop.run_app(&mut app)`. Wry also changed its builder between minor releases. Some LLMs also use the old `tui-rs` crate in place of `ratatui`.
3. **Context loss.** A toy engine has at least six phases and thousands of lines. The LLM writes it in many steps. It then forgets earlier definitions, for example whether a `Rect` uses integers or floating-point numbers. This causes type mismatches between modules.

## Methods in the report

1. **Pin the dependencies.** Give the full `Cargo.toml` with exact versions in the first prompt. Do not let the LLM choose versions.
2. **Use one file first.** Ask the LLM to write the first version in `src/main.rs` only. Divide it into modules after it compiles.
3. **Use flat data.** Do not allow `Rc<RefCell<T>>`. Use vectors, flat token lists or index-based buffers.

## Decision matrix of the report

| Metric | Weight | A: terminal | B: webview | C: toy engine | D: Servo |
|---|---|---|---|---|---|
| Small code size | 20% | 5 | 5 | 2 | 1 |
| API stability | 25% | 5 | 3 | 4 | 2 |
| Low setup work | 20% | 5 | 3 | 4 | 1 |
| LLM success rate | 25% | 5 | 3 | 1 | 1 |
| Product utility | 10% | 3 | 5 | 1 | 4 |
| Score in report | 100% | 4.75 | 3.65 | 2.40 | 1.50 |
| Score from the weights | 100% | 4.80 | 3.60 | 2.55 | 1.55 |

The scores in the report do not agree with its weights. The order of the options stays the same.

## Claims to test

| Claim | How to test it |
|---|---|
| Pinned versions stop API drift. | Ask an LLM to write the segment 03 browser with and without the pinned `Cargo.toml`. Compare the compile errors. |
| One file stops context loss. | Ask for the segment 04 engine as one file and as many files. Count the type mismatches. |
| Flat data stops borrow-checker errors. | Ask for a DOM tree with and without the flat-data rule. Compare the compile errors. |
| The matrix scores are correct. | Give a source or a test result for each table value. Then calculate the scores again. |

## Experiment

1. Write one prompt template for each method. Store it in this folder.
2. Use a current model for each test. The HTML report names old models, Claude 3.5 Sonnet and GPT-4o.
3. Do each test five times. Record the model, the date and the compile result.
4. Update the decision matrix with the measured values.

## Decision

- Use the methods that reduce compile errors in the tests.
- Replace the scores of the report with measured scores.

Source: `docs/sources/2026-10-05-rust-browser-llm-feasibility-report.md`.

## References

- [winit change log](https://docs.rs/winit/latest/winit/changelog/index.html)
- [winit releases](https://github.com/rust-windowing/winit/releases)
