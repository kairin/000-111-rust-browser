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

Source: `docs/sources/2026-10-05-rust-browser-llm-feasibility-report.md`. Dependency, build-time and standards-support statements describe that report’s assessment; this segment has not checked a current release or measured a build.

## Arguments for and against this route

- **For longer-term engine research:** Servo offers an engine-embedding path for studying a Rust-oriented rendering system and owning a custom shell. This is a different goal from controlling Chrome.
- **Against near-term Workday selection:** integration cost, evolving embedding interfaces and untested workflow compatibility add risk without a demonstrated automation advantage. The [Workday report](../../sources/2026-10-05-workday-chrome-automation-report.md) argues for an existing Chrome engine.
- **Boundary:** no experiment here shows that Servo can complete the intended Workday workflows or that it cannot ever do so. Keep the long-term research separate from current choices in [recommendations](../../recommendations.md).

## Claims to test

The report gives these numbers without a source.

| Claim | Report value | How to test it |
|---|---|---|
| Lines of code | more than 2,000,000 | Count the lines in the Servo repository. |
| Cold compile time | 30 to more than 90 minutes | Build Servo once. Record the time and the computer. |
| Web standards support | about 80% | Find a Servo statement or a test result. |
| LLM failure rate | more than 95% | No evidence yet. A long build is a scope constraint, not a measurement of LLM failure rate. |
| Composite score | 1.50 of 5 | Calculate again in segment 06. The weights give 1.55. |

## Experiment

This segment does not need the one-afternoon limit. It asks a long-term question.

1. Read the current Servo embedding documentation. Record the current embedding API.
2. Find the current state of Verso and `tauri-runtime-verso`.
3. Build `servoshell`, the sample browser of the Servo project. Record the build time.
4. Record whether Servo has an automation interface that segment 01 can use.
5. If the build and interface are usable, test a controlled page action, SPA state change and download with declared result checks. Compare identical tasks with segment 01 before making an automation claim.
6. If Workday remains a target, use the authorised scenarios in [segment 08](../08-workday-data-access/README.md). A static page rendering result is insufficient. These are proposed experiments, not completed compatibility tests.

## Decision

- Defer Servo for the afternoon-generation goal: the report gives no validated small embedding blueprint. This is a scope decision, not a blanket impossibility claim.
- For longer-term research, set a build and integration budget first. Stop if a supported embedding path or required automation interface cannot be demonstrated within it.
- If the embedding API is usable, compare Servo with segment 02 as a long-term engine choice. Workday adoption additionally needs demonstrated compatibility and an advantage over the existing-browser baseline; [segment 07](../07-rust-chrome-controller/README.md) covers that comparison.

## References

- [Servo repository](https://github.com/servo/servo)
- [Verso](https://github.com/versotile-org/verso/)
- [tauri-runtime-verso](https://github.com/versotile-org/tauri-runtime-verso)
- [Servo improvements for Tauri, NLnet](https://nlnet.nl/project/Verso/)
- [Tauri and Servo discussion](https://github.com/orgs/tauri-apps/discussions/15235)
