# 000-111-rust-browser — AI Agent Guidelines

Single source of truth for AI agents in this repository. If `CLAUDE.md` or `GEMINI.md` exists, it points to this file.

## Security and secrets

- Never expose, print, log, commit, or include in diffs, prompts, fixtures, screenshots, or generated files any API key, token, password, credential, private key, session cookie, or other secret. Redact with `<REDACTED>`.
- Never ask the user to paste a secret into chat. Do not read or display secret-file contents. If a credential is missing, stop and explain how to provide it securely.
- Secrets live in the owner's `pass` password store. A command gets a secret only through `with-secret <service>/<name> -- <command>`. Never put a secret in `.env`, `.envrc`, `.envrc.local`, source code, config or documentation.
- Do not send personal or sensitive data to an external service unless the user explicitly authorizes it.

## Protected files

Do not rewrite these without an explicit per-file instruction from the user: `docs/segments/02-webview-shell/blueprint/Cargo.toml` and `docs/segments/03-terminal-browser/blueprint/Cargo.toml`. Read them freely. If a task needs one changed, stop and ask. `git show origin/main:<file>` is the authoritative original.

## Project overview

This repository holds learning notes about how an LLM can read and operate a website, and where Rust can help with a browser shell or controller. The content is divided into eight research segments in `docs/segments/`. All segments are research plans, not completed experiments. `docs/sources/` holds the original documents without change, and `docs/review/` records the problems in them. There is no browser implementation.

## Technologies

- Documentation only: Markdown documents and one HTML report.
- Segments 02 and 03 hold Rust code blueprints (`Cargo.toml` and `src/main.rs`) from the report. Nothing builds them.

## Development workflow

No build or test commands. The repository holds documents only.

A task is done only when you show evidence: command output, a log line or a screenshot.

## Branches and pull requests

- Never push to `main` directly. Make a branch, push it, and open a pull request.
- Keep `CHANGELOG.md` up to date if the repository has one.

## Documentation

- `README.md`: for users.
- `AGENTS.md` (this file): for agents. Update it when a tool, a command or a rule changes.

## Git identity

Commit as `Mister K <678459+kairin@users.noreply.github.com>`. This is the
public GitHub name and the GitHub noreply email. Do not commit with another
name or with a personal email address. Check with `git config user.name` and
`git config user.email` before you commit.
