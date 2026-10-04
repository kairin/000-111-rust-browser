# Segment 02: Webview shell

Status: not started.

## Research question

Can we build a Rust browser shell on an existing web engine? Can an LLM write this shell in one afternoon of three to four hours?

A shell is the window, menus and controls around a web engine. The web engine reads HTML, applies CSS, runs JavaScript and draws the page.

## Terms

- **Webview:** a browser area that a program puts inside its own window. The operating system supplies the web engine.
- **Wry:** a Rust crate that makes a webview. A crate is a Rust code library.
- **Tao:** a Rust crate that makes windows and runs the event loop. The Tauri project makes wry and tao.
- **Event loop:** the part of a program that waits for input, such as a click, and then acts on it.
- **WebKitGTK:** the web engine that wry uses on Linux.
- **IPC (inter-process communication):** a channel that carries messages between two parts of a program.

## What the sources say

- The webview gives the drawing, the CSS layout and the JavaScript work to the web engine of the operating system.
- Wry uses WebKit on macOS, WebView2 (Edge Chromium) on Windows and WebKitGTK on Linux.
- The Rust program makes a window, starts the webview and controls an IPC channel or a menu.
- On Linux, the build needs `libwebkit2gtk-4.1-dev` and the GTK3 libraries. If they are not installed, the build fails when it links the C libraries.
- On Linux, WebKitGTK is a different engine from Chromium. Thus Playwright tests in Chromium do not test the same engine.
- The discussion says that the main advantage of a Rust shell is its automation interface, not the Rust language.
- The report says that the wry builder API changes between minor releases. LLMs often mix old and new forms.

Sources: `docs/sources/2026-10-05-discussion.md` and `docs/sources/2026-10-05-rust-browser-llm-feasibility-report.md`.

## Claims to test

The report gives these numbers without a source.

| Claim | Report value | How to test it |
|---|---|---|
| Lines of code | 150 to 350 | Count the lines of a working shell. |
| Cold compile time | 1 to 3 minutes | Build once with no saved build files. Record the time and the computer. |
| Web compatibility | 99% | Open a set of test pages. Record each page that fails. |
| LLM failure rate | 25% to 35% | Ask an LLM to write the shell five times. Count the attempts that do not compile. |
| Composite score | 3.65 of 5 | Calculate again in segment 06. The weights give 3.60. |

## Blueprint

The folder `blueprint/` holds the sample program from the report, without change. It pins tao 0.30.0, wry 0.46.0 and url 2.5.0. A pinned version is an exact version number in `Cargo.toml`.

Known problems:

- The program moves the `webview` value into a new thread. We think that the wry `WebView` type cannot go to another thread. If so, the program does not compile.
- GTK must run on the main thread. A second thread that controls the webview can fail on Linux.
- The report describes the winit 0.30 event loop change. But the blueprint uses tao, not winit. Make sure which API tao 0.30 uses.

## Experiment

1. Install the WebKitGTK and GTK3 development packages. Record their names and versions.
2. Build the blueprint without changes. Record each compile error.
3. Repair the blueprint to the smallest extent. Record each change.
4. Open one website in the shell.
5. Add one automation command, for example "read the page title". Make sure that a program can call it.
6. Compare the result with segment 01.

## Decision

- Continue only if segment 01 shows a need that an existing browser cannot meet.
- If the shell builds and the automation command works, write a design for a full automation interface.
- If the build needs more than one afternoon, record why.

## References

- [Wry repository](https://github.com/tauri-apps/wry)
- [Wry documentation](https://docs.rs/wry)
- [Tauri architecture](https://v2.tauri.app/concept/architecture/)
- [winit change log](https://docs.rs/winit/latest/winit/changelog/index.html)
