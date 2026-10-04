# Document review

Date: 2026-10-05.

This review examines all documents in the repository on this date. It records the problems that we found. It also tells how we divided the content into research segments.

## Terms

- **LLM (large language model):** an AI system that reads and writes text, for example Claude.
- **Feasibility:** a measure of whether a task is possible with the time, tools and skill that we have.
- **Segment:** one research question in this repository. Each segment has its own folder in `docs/segments/`.
- **Crate:** a Rust code library.

## Documents that we examined

| Document | Type | Origin |
|---|---|---|
| `README.md` | Project summary | Written in this repository |
| `docs/sources/2026-10-05-discussion.md` | Saved conversation | Written in this repository |
| `docs/sources/2026-10-05-rust-browser-llm-feasibility-report.md` | Research report, about 3,500 words | Added by the user, origin not recorded |
| `docs/sources/2026-10-05-rust-browser-llm-feasibility-report.html` | Interactive web page of the same report | Added by the user, origin not recorded |

## Finding 1: The documents ask two different questions

The discussion and the README ask this question: how can an LLM operate a website? An LLM operates a website when it reads a page, clicks controls, types text and makes sure of the result.

The report asks a different question: can an LLM write a Rust web browser in one afternoon of three to four hours?

The two questions share the topic of Rust browsers, but they have different goals. Thus we put them in different segments. Segment 01 holds the first question. Segments 02 to 06 hold the second question.

## Finding 2: The composite scores in the report do not agree with its weights

The report gives a weight to each metric. It then gives a composite score to each option. We calculated the scores again from the weights and the table values. No score agrees.

| Option | Score in report | Score from the weights |
|---|---|---|
| A: terminal browser | 4.75 | 4.80 |
| B: embedded webview shell | 3.65 | 3.60 |
| C: toy engine | 2.40 | 2.55 |
| D: Servo extension | 1.50 | 1.55 |

The order of the options does not change. But the error shows that nobody examined the numbers. Segment 06 must calculate the scores again.

## Finding 3: The report gives numbers without a source

The report gives these numbers but no measurement or source for them:

- LLM failure rates, for example "less than 10%" and "70% to 85%".
- Cold compile times, for example "45 to 90 seconds". A cold compile is the first build, with no saved build files.
- Web compatibility, for example "99%".
- Lines of code for each option.

Each segment lists its numbers as claims to test. Do not use a number from the report as a fact until a test makes sure of it.

## Finding 4: The two report files number the tracks differently

The Markdown report calls the webview shell "Track 1" and the terminal browser "Track 2". The HTML page uses the opposite order. In this repository, we use segment numbers and not track numbers.

## Finding 5: The code blueprints have not been compiled

A blueprint is a sample program in the report. We copied the two blueprints from the Markdown report into the segment folders. We did not change the code.

- The webview blueprint moves the `webview` value into a new thread. In Rust, a value can go to another thread only when its type has the `Send` property. We think that the wry `WebView` type does not have this property. If so, the blueprint does not compile. Segment 02 must make sure of this.
- The HTML page has a shorter terminal-browser blueprint. It does not load a first page at start. We used the longer Markdown version.

## Finding 6: Some content is old or weak

- The HTML page makes prompts for "Claude 3.5 Sonnet or GPT-4o". These models are old on the date of this review.
- The report says that winit 0.30 changed its event loop. But the webview blueprint uses tao and not winit. Tao is a separate crate. Segment 02 must make sure which tao API the blueprint needs.
- Many sources in the report are forum posts and blog posts, for example Reddit and DEV Community. Segment work must find a primary source for each important claim. A primary source is the project documentation, the source code or a release note.
- The citation numbers in the report touch the words before them, for example "APIs1". This makes the text hard to read.

## Finding 7: The HTML page needs internet access

The HTML page loads Tailwind CSS and Chart.js from content delivery networks. If the computer has no internet access, the page shows no charts and no styles.

## How we divided the content

| Segment | Research question | Content from |
|---|---|---|
| [01 LLM web operation](../segments/01-llm-web-operation/README.md) | Can an LLM operate a website through an existing browser? | Discussion, README |
| [02 Webview shell](../segments/02-webview-shell/README.md) | Can we build a Rust browser shell on an existing web engine? | Discussion, report |
| [03 Terminal browser](../segments/03-terminal-browser/README.md) | Can an LLM write a terminal browser in Rust in one afternoon? | Report |
| [04 Toy engine](../segments/04-toy-engine/README.md) | Can an LLM write a small rendering engine from the start? | Report |
| [05 Servo shell](../segments/05-servo-shell/README.md) | Can we build a browser shell on the Servo engine? | Report |
| [06 LLM Rust generation](../segments/06-llm-rust-generation/README.md) | Which methods help an LLM write Rust code that compiles? | Report |

We kept the source files without change in `docs/sources/`. They are records. The segment documents carry their facts in ASD-STE100 Simplified Technical English.
