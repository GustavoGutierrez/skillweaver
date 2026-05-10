# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog and this project follows Semantic Versioning.

## [0.1.0] - 2026-05-10

### Added
- Initial real Rust + Ratatui SkillWeaver application at the repository root.
- Keyboard-first TUI shell with Dashboard, Profiles, Repositories, System Settings, and Help screens.
- Profile persistence plus import/export support.
- Narrow skill discovery conventions and `SKILL.md` frontmatter parsing.
- Install preview, blocker enforcement, safe ZIP extraction, and symlink-first copy fallback behavior.
- Skills-only `skills-lock.json` handling.
- Managed block mutation for `AGENTS.md` / `CLAUDE.md`.
- Unit and safety test coverage for install, mutation, and modal lifecycle behavior.
