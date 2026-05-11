# SkillWeaver

[![Rust](https://img.shields.io/badge/Rust-2024_edition-orange?logo=rust)](https://www.rust-lang.org/)
[![Ratatui](https://img.shields.io/badge/Ratatui-0.30-cyan)](https://ratatui.rs/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

SkillWeaver is now a real root Rust + Ratatui application in this repository.

## What is implemented in this slice

- Root Cargo project (`Cargo.toml`, `src/`, `tests/`)
- Keyboard-first Ratatui runtime with real screens:
  - Dashboard
  - Profiles
  - Repositories
  - System Settings
  - Help
- Dashboard shows active/default profile and currently selected skills/rules
- Profiles screen supports baseline keyboard flows:
  - create (`c`)
  - edit name (`e`)
  - select default (`Enter`)
  - select default (`Space`)
  - duplicate (`d`)
  - delete with confirmation (`x` + `Enter`)
  - import profiles (`i`, from `./skillweaver-profiles.json`)
  - export profiles (`o`, to `./skillweaver-profiles.json`)
- Repositories screen supports baseline runtime flows:
  - add local source (`n` modal with name/path)
  - scan registered sources (`r`)
  - view discovery results and add selected discovery to active profile (`a` or `Space`)
- Explicit empty states across Dashboard/Profiles/Repositories/Settings/Help
- Minimal but real modal/input interactions for creation/confirmation
- Profile JSON persistence model and import/export services
- Source scanning conventions restricted to:
  - `skills/`
  - `.agents/skills/`
  - `.claude/skills/`
- `SKILL.md` YAML frontmatter parsing (`name`, `description`, `triggers`)
- Install preview with enforced blockers for traversal, unreadable metadata, target conflicts, and untrusted skill selections
- Safe ZIP extraction with path traversal rejection
- Symlink-first with copy fallback behavior
- `skills-lock.json` handling for **skills only**
- Managed block mutation in `AGENTS.md` / `CLAUDE.md` using exact markers:
  - `<!-- BEGIN SKILLWEAVER RULES -->`
  - `<!-- END SKILLWEAVER RULES -->`
- Deterministic runtime transition tests for key modal and state flows

## Verified commands

From repository root:

```bash
cargo check
cargo test
```

## Notes

- This is an MVP slice focused on safety-critical install/mutation behavior plus usable runtime profile/source workflows.
- UI text is in English and navigation is keyboard-first.
