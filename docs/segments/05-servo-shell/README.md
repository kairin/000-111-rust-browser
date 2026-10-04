# Segment 05: Servo shell

Status: not started.

## Research question

Can we build a browser shell on the Servo web engine? Is this possible in one afternoon of three to four hours?

## Terms

- **Servo:** a web engine written in Rust. It does layout in parallel on many processor cores.
- **mach:** the build tool of the Servo project.
- **SpiderMonkey:** the JavaScript engine that Servo uses. It is written in C++.
- **Verso:** a project that puts Servo into a browser that other programs can use.
- **Embedding:** the use of a web engine inside a different program.

## What the sources say

- Mozilla started Servo. The Linux Foundation and Igalia now manage it.
- Servo has hundreds of crates, threaded pipelines and C++ bindings to SpiderMonkey.
- The build needs Clang and LLVM, CMake, Python 3 with the Mako template engine, and HarfBuzz. The report table also lists GStreamer and Vulkan.
- The report says that the first build takes 30 to more than 90 minutes.
- Verso and other embedding projects are experimental. Their interfaces change frequently.
- The report rejects this route for one afternoon. Its score is 1.50 of 5, the lowest score.
- The report also gives Servo a high utility score, 4 of 5, because it supports about 80% of modern web standards.

Source: `docs/sources/2026-10-05-rust-browser-llm-feasibility-report.md`.

## Claims to test

The report gives these numbers without a source.

| Claim | Report value | How to test it |
|---|---|---|
| Lines of code | more than 2,000,000 | Count the lines in the Servo repository. |
| Cold compile time | 30 to more than 90 minutes | Build Servo once. Record the time and the computer. |
| Web standards support | about 80% | Find a Servo statement or a test result. |
| LLM failure rate | more than 95% | Do not test this. The build time alone is outside one afternoon. |
| Composite score | 1.50 of 5 | Calculate again in segment 06. The weights give 1.55. |

## Experiment

This segment does not need the one-afternoon limit. It asks a long-term question.

1. Read the current Servo embedding documentation. Record the current embedding API.
2. Find the current state of Verso and `tauri-runtime-verso`.
3. Build `servoshell`, the sample browser of the Servo project. Record the build time.
4. Record whether Servo has an automation interface that segment 01 can use.

## Decision

- Do not use Servo for an afternoon project.
- If the embedding API is stable, compare Servo with segment 02 as a long-term engine choice.

## References

- [Servo repository](https://github.com/servo/servo)
- [Verso](https://github.com/versotile-org/verso/)
- [tauri-runtime-verso](https://github.com/versotile-org/tauri-runtime-verso)
- [Servo improvements for Tauri, NLnet](https://nlnet.nl/project/Verso/)
- [Tauri and Servo discussion](https://github.com/orgs/tauri-apps/discussions/15235)
