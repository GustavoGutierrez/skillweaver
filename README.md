# SkillWeaver

SkillWeaver is now a real root Rust + Ratatui application in this repository.

## What is implemented in this slice

- Root Cargo project (`Cargo.toml`, `src/`, `tests/`)
- Keyboard-first Ratatui shell with screens:
  - Dashboard
  - Profiles
  - Repositories
  - System Settings
  - Help
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
- Modal lifecycle in app state (`Preview` -> `InstallConfirm` -> close via `Esc`) with deterministic tests

## Verified commands

From repository root:

```bash
cargo check
cargo test
```

## Notes

- This is an MVP vertical slice focused on safety-critical install and mutation behavior.
- UI text is in English and navigation is keyboard-first.
