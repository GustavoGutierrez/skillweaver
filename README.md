# 🛠️ SkillWeaver

**The terminal-based package manager for AI agent skills and rules.**

SkillWeaver lets you discover, install, and share collections of AI agent skills across projects — without hunting them down one by one. Define a profile once, apply it everywhere.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---

## What problem does it solve?

Installing AI agent skills today means manually downloading `.zip` files, extracting them into `.agents/skills/`, updating `skills-lock.json`, and appending rules to `AGENTS.md`. Multiply that by multiple projects and multiple team members — it becomes tedious and error-prone.

SkillWeaver automates the entire workflow. One TUI, one profile, one command. Done.

---

## Quick start

```bash
# Install via Homebrew
brew tap GustavoGutierrez/skillweaver
brew install skillweaver

# Launch the TUI
skillweaver
```

**What happens next:**
1. Select a profile (e.g., "Frontend Dev") with Tab nav
2. Toggle skills with Space — metadata auto-loads from `SKILL.md`
3. Press `i` to install — skills land in `.agents/skills/`, lockfile updates, `AGENTS.md` gets new rules appended safely

---

## Features

| Feature | What it does |
|---------|--------------|
| **Profiles** | Define named collections of skills + rules, save them globally, reuse across projects |
| **Batch install** | Download, extract, and register multiple skills at once |
| **Lockfile integrity** | `skills-lock.json` tracks source, path, and SHA-256 hash per skill |
| **Safe rule injection** | Appends rules to `AGENTS.md` / `CLAUDE.md` without overwriting existing content |
| **Import / Export** | Share profiles as JSON files — ideal for teams onboarding new members |
| **Multi-source** | Supports GitHub repos, direct `.zip` URLs, and local paths |
| **Cross-platform** | macOS, Linux, and WSL via `dirs`-based config paths |

---

## How it works

```
┌─ Profile ──────┐     ┌─ SkillWeaver ──┐     ┌─ Project ────────────┐
│ skills:         │────▶│ downloads .zip  │────▶│ .agents/skills/      │
│   - prd-writer  │     │ extracts to     │     │ skills-lock.json     │
│   - prompt-eng  │     │ target project  │     │ AGENTS.md (updated)  │
│ rules:          │     │ validates hash  │     └──────────────────────┘
│   - convention  │     └────────────────┘
└─────────────────┘
```

1. **Profiles** live in `~/.config/skillweaver/profiles.json` — global, reusable, shareable
2. **Skills** are `.zip` archives fetched from GitHub or direct URLs, extracted into `.agents/skills/<skill-name>/`
3. **Lockfile** (`skills-lock.json`) records every installed skill with its source, path, and SHA-256 hash — detects drift
4. **Rules** from `.zip` archives get parsed (triggers extracted from frontmatter) and smart-appended to `AGENTS.md`

---

## Architecture

SkillWeaver follows the **Elm Architecture (Model-View-Update)** — the standard pattern for Ratatui applications:

```
src/
├── main.rs          # Entry point, Tokio runtime
├── app.rs           # Centralized application state (Model)
├── ui.rs            # Ratatui rendering (View)
├── events.rs        # Crossterm input handling
├── core/
│   ├── config.rs    # ~/.config/skillweaver/ management
│   ├── lockfile.rs  # skills-lock.json ser/de
│   └── profile.rs   # Import/export logic
├── manager/
│   ├── downloader.rs # Async HTTP (reqwest)
│   ├── extractor.rs  # ZIP decompression (zip-rs)
│   └── symlink.rs    # Cross-platform symbolic links
└── parsers/
    ├── frontmatter.rs # SKILL.md YAML extraction
    └── markdown.rs    # Safe AGENTS.md rule appending
```

**Key design decisions:**

| Decision | Rationale |
|----------|-----------|
| Elm Architecture | Decouples rendering from I/O — UI never freezes during downloads |
| Tokio async channels | Network operations run on background tasks, results sent via `mpsc` |
| Strategy pattern for sources | GitHub, GitLab, local paths — extensible without refactoring core logic |
| Builder pattern for Markdown | Constructs safe table rows when appending rules to `AGENTS.md` |

---

## TUI Layout

```
┌─ SkillWeaver v1.0.0 ───────────────────────────────────────────────────────────┐
│  [1] Dashboard   [2] Profiles   [3] Repositories   [4] System Settings         │
├──────────────────────┬─────────────────────────────────────────────────────────┤
│ Active Profile:      │ Available Skills in Profile (Frontend Dev)              │
│ > Frontend Dev       │                                                         │
│   Backend Security   │ [x] prompt-engineer  (anthropics/skills)                │
│   SvelteKit Expert   │ [ ] prd-writer       (GustavoGutierrez/eng-skills)      │
│   Rust Systems       │ [x] compliance-guard (GustavoGutierrez/eng-skills)      │
│                      │                                                         │
│ Actions:             │ Skill Details: prd-writer                               │
│ [i] Install Profile  │ ─────────────────────────────────────────────────────── │
│ [e] Export Profile   │ Source: GitHub (.zip archive)                           │
│ [m] Import Profile   │ Desc: Generates Product Requirements Documents (PRD).   │
│ [a] Add New Skill    │ Path: .agents/skills/prd-writer                         │
│ [d] Delete Profile   │ Triggers: "Write a PRD", "Draft requirements"           │
├──────────────────────┴─────────────────────────────────────────────────────────┤
│ Status: [Success] Extracted compliance-guard.zip into .agents/skills/          │
│ [q] Quit  [↑↓] Navigate  [Space] Toggle  [Enter] Apply Actions  [h] Help       │
└────────────────────────────────────────────────────────────────────────────────┘
```

---

## Tech Stack

| Layer | Crate |
|-------|-------|
| TUI rendering | `ratatui` 0.26, `crossterm` 0.27 |
| Async runtime | `tokio` (full features) |
| HTTP client | `reqwest` (JSON + streaming) |
| Serialization | `serde`, `serde_json`, `serde_yaml` |
| ZIP handling | `zip` 0.6 |
| Hashing | `sha2` 0.10 |
| System paths | `dirs` 5.0 |
| Error handling | `anyhow`, `thiserror` |

---

## Contributing

1. Fork the repository
2. Create a feature branch (`feat/my-feature`)
3. Write tests covering your changes
4. Open a PR against `main`

The project follows conventional commits and enforces `rustfmt` + `clippy` in CI.

---

## Author

**Ing. Gustavo Gutiérrez** — Bogotá, Colombia

[![LinkedIn](https://img.shields.io/badge/LinkedIn-0077B5?style=flat&logo=linkedin&logoColor=white)](https://www.linkedin.com/in/gustavo-gutierrez-mercado)
[![Ko-fi](https://img.shields.io/badge/Ko--fi-Support%20me-FF5E5B?style=flat&logo=ko-fi&logoColor=white)](https://ko-fi.com/Z8Z81YYSUI)

---

## License

MIT — see [LICENSE](./LICENSE) for details.
