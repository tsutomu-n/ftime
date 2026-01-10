# AGENTS.md (for gpt-5.1-codex-max)

## Overview (WHAT)
- Stack: Rust (edition 2024), Cargo, Dependencies: clap(derive), colored, chrono, is-terminal.
- Structure: Monorepo CLI 'ftime' - src/main.rs (CLI/mode), src/engine.rs (scan/sort/filter), src/model.rs (structs), src/view/ (tty/text output), src/util/time.rs (rel time). Tests: mod tests (unit), tests/ (integration).
- Purpose: Read-only file timestamp scanner/sorter/filter for efficient history management (buckets: 1h/today/7d).

## Expectations (WHY)
- Goal: Scalable, panic-free CLI with minimal I/O. Use xhigh reasoning for multi-step code gen (e.g., PR format). Ignore if irrelevant; focus on task (e.g., engine fixes > UI).

## Instructions (HOW)
- Build/Test: cargo build/run/fmt/clippy (--all-targets -- -D clippy::pedantic)/test/nextest. Fix: cargo fix.
- Conventions: Result error prop, no unwrap/expect outside tests. snake_case funcs, PascalCase types. rustfmt.toml for indent=4.
- Deep Dives: Tests? See agent_docs/testplan.md (ask: "Need details?"). Architecture: docs/ARCHITECTURE.md. Commit: agent_docs/commit.md.
- Tools: /git for diffs, /search for refs (e.g., src/engine.rs:42). Use prompt cache for repeats.

<agent-reminder> IMPORTANT: Apply only if highly relevant. Confirm unclear tasks with user. </agent-reminder>
