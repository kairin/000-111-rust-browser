# Segment 06: LLM Rust generation

Status: not started.

## Research question

Which methods help an LLM write Rust code that compiles on the first attempt? Is the decision matrix of the report correct?

This segment applies to segments 02, 03 and 04, and can support the Rust controller proposed in [segment 07](../07-rust-chrome-controller/README.md). Compiling that controller is an enabling step; workflow reliability remains a separate test.

## Terms

- **Borrow checker:** the part of the Rust compiler that makes sure that memory use is safe.
- **API drift:** a change in a library interface between versions. An LLM can learn both the old and the new form, and then mix them.
- **Context window:** the amount of text that an LLM can use at one time.
- **Decision matrix:** a table that gives a weighted score to each option.

## Failure modes in the report

The feasibility report names three reasons why LLMs can fail with Rust browser code. These are source-derived failure hypotheses, not measured failure rates for the current model or project.

1. **Tree ownership.** DOM nodes refer to parents, siblings and children. Generated code can have compile-time borrow conflicts. With `Rc<RefCell<Node>>`, conflicting dynamic borrows can instead panic at run time. An arena with `NodeId` index numbers changes the ownership problem, but still needs valid indices and correct traversal.
2. **API drift.** Before winit 0.30, the event loop used a closure, for example `event_loop.run(move |event, _, control_flow| { ... })`. Version 0.30 uses the `ApplicationHandler` trait and `event_loop.run_app(&mut app)`. Wry also changed its builder between minor releases. Some LLMs also use the old `tui-rs` crate in place of `ratatui`.
3. **Context loss.** A toy engine has at least six phases and thousands of lines. The LLM writes it in many steps. It then forgets earlier definitions, for example whether a `Rect` uses integers or floating-point numbers. This causes type mismatches between modules.

## Methods in the report

1. **Pin the dependencies.** Give the full `Cargo.toml` with exact versions in the first prompt. Check generated calls against those versions. Pinning makes the test reproducible; it does not stop an LLM from emitting an outdated API.
2. **Use one file first.** Ask the LLM to write a small first version in `src/main.rs`. Compile it before expanding it or dividing it into modules. A larger controller still needs explicit interfaces; one long file can become difficult to maintain.
3. **Use flat data where appropriate.** For the toy DOM experiment, compare vectors and index-based nodes with `Rc<RefCell<T>>`. Flat data simplifies some ownership relationships but adds index validation and does not eliminate every borrowing or logic error.

For the controller proposal in the [Workday report](../../sources/2026-10-05-workday-chrome-automation-report.md), first compile small sketches for observation, action, waiting and download handling against pinned dependencies. Keep library-specific types behind a narrow interface. The report’s Rust snippets are uncompiled architecture sketches with omitted surrounding types, not a working controller.

## Arguments for and against these methods

- **For generation research:** fixed dependency versions and small compile checkpoints make errors reproducible and help locate API or ownership mistakes before the programme grows.
- **Against treating them as sufficient:** flat data and one-file structure have correctness and maintenance costs. First-attempt compilation does not check browser waiting, extraction completeness or business results.
- **Boundary:** select methods for the tested scope and keep controller adoption dependent on complete verified workflows.

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

The scores in the report do not agree with its weights. The order of the options stays the same. The corrected values are source-derived arithmetic, not measured evidence: the underlying ratings and weights remain a heuristic for afternoon Rust generation. They do not rank Workday workflow reliability or Chrome-controller libraries. Keep those decisions in [recommendations](../../recommendations.md).

## Claims to test

| Claim | How to test it |
|---|---|
| Pinned versions reduce API mismatch. | Ask an LLM to write the segment 03 browser with and without the pinned `Cargo.toml`. Compare the compile errors. |
| One small file reduces context loss. | Ask for the segment 04 engine as one file and as many files. Count the type mismatches. |
| Flat data reduces ownership errors for a DOM. | Ask for a DOM tree with and without the flat-data rule. Compare compile errors and runtime traversal results. |
| The matrix scores are correct. | Give a source or a test result for each table value. Then calculate the scores again. |

## Experiment

1. Write one prompt template for each method. Store it in this folder.
2. Use a current model for each test. The HTML report names old models, Claude 3.5 Sonnet and GPT-4o.
3. Do each test five times. Record the model, the date and the compile result.
4. Record the raw outcomes before changing a rating. Keep the source matrix above intact and place any measured comparison beside it, with its sample size and scope.
5. For segment 07, compile one pinned backend sketch at a time. Then run the same delayed-page, stale-target and download scenarios as segment 01. Record compile success separately from complete verified tasks.
6. Stop the generation experiment at its stated time or attempt budget. A small sample supports a local comparison, not a universal model failure rate.

## Decision

- Use methods that improve the tested generation task, with their maintenance and data-validation tradeoffs recorded.
- Preserve the corrected source matrix as a historical heuristic. Adopt measured ratings only when their evidence and intended goal are explicit.
- A compiled Rust controller has not yet proved reliable enterprise navigation. Advance it only through the identical workflow comparisons in segment 07; keep the Workday-specific result checks in [segment 08](../08-workday-data-access/README.md).

Source: `docs/sources/2026-10-05-rust-browser-llm-feasibility-report.md`.

## References

- [winit change log](https://docs.rs/winit/latest/winit/changelog/index.html)
- [winit releases](https://github.com/rust-windowing/winit/releases)
