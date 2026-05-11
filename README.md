# SkillWeaver

[![Rust](https://img.shields.io/badge/Rust-2024_edition-orange?logo=rust)](https://www.rust-lang.org/)
[![Ratatui](https://img.shields.io/badge/Ratatui-0.30-cyan)](https://ratatui.rs/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

**Terminal-based package manager for AI agent skills and rules.**

SkillWeaver lets you discover, preview, install, and share collections of AI agent skills across projects — without hunting them down one by one. Define a profile once, apply it everywhere.

---

## Quick Start

```bash
# Build from source
cargo build

# CLI mode — install a profile directly (auto-detects current directory)
./target/debug/skillweaver install "Recommended Starter"

# CLI mode — list available profiles
./target/debug/skillweaver list

# CLI mode — import profiles from JSON
./target/debug/skillweaver import profiles/skillweaver-profile-recommended.json

# TUI mode — interactive terminal interface (no arguments)
./target/debug/skillweaver
```

### CLI Commands

| Command | Description |
|---------|-------------|
| `skillweaver install <name>` | Download and extract all skills from a profile into the current directory |
| `skillweaver install <name> --target <path>` | Install into a specific directory |
| `skillweaver list` | Show all profiles with skill and rule counts |
| `skillweaver import <file>` | Import profiles from a JSON file |
| `skillweaver` (no args) | Launch the interactive TUI |

The CLI shows real-time progress for each skill being downloaded and extracted:

```
[1/12] context-engineer
  ⬇ downloading context-engineer...
  📦 extracting context-engineer...
  ✅ context-engineer installed
[2/12] prompt-engineer
  ⬇ downloading prompt-engineer...
  ...
[3/12] ⏭ prd-writer (already installed)

📝 writing AGENTS.md...
🔒 updating skills-lock.json...
Installed profile 'Recommended Starter' via 12 skill(s) installed
```

---

## How to Use

### Navigation

| Key | Action |
|-----|--------|
| `1`–`5` or `h` | Switch screen |
| `Tab` | Next screen |
| `q` | Quit |
| `j` / `↓` | Move selection down |
| `k` / `↑` | Move selection up |
| `Enter` or `Space` | Confirm or select default |

The **active screen** is highlighted with `●` in the top bar — you always know where you are.

---

### Dashboard `[1]`

View your profiles, see which skills and rules each one contains, and set a default.

| Key | Action |
|-----|--------|
| `j` / `k` | Navigate profile list |
| `Enter` or `Space` | Set selected profile as default |
| `i` | Install selected profile (Enter to confirm, auto-detects current dir) |

The right panel shows the currently selected profile's skills and rules.

---

### Profiles `[2]`

Create, edit, and manage reusable skill bundles.

| Key | Action |
|-----|--------|
| `c` | Create new profile (modal — type name, Enter to confirm) |
| `e` | Edit profile name |
| `s` | Add skill to profile (modal — type skill name, Enter) |
| `r` | Add rule to profile (modal — type rule, Enter) |
| `d` | Duplicate profile |
| `x` | Delete profile (confirmation required) |
| `i` | Import profiles from JSON file (modal — type file path) |
| `o` | Export profiles to JSON file (modal — type file path) |

**Importing pre-made profiles:**
```bash
# In the TUI, go to Profiles [2], press i, then type:
profiles/skillweaver-profile-recommended.json
```

Pre-made profiles are in `profiles/`:
- `skillweaver-profile-recommended.json` — 12 core skills (starter)
- `skillweaver-profiles.json` — all 24 engineering skills
- `skillweaver-profile-frontend.json` — frontend development
- `skillweaver-profile-backend.json` — backend development
- `skillweaver-profile-devops.json` — DevOps & platform
- `skillweaver-profile-product.json` — product management
- `skillweaver-profile-architecture.json` — solution architecture

You can import multiple profiles — they merge, never overwrite.

---

### Repositories `[3]`

Register local skill directories, scan for skills, and add them to your active profile.

| Key | Action |
|-----|--------|
| `n` | Add local source (modal — type name, Tab to path, Enter) |
| `r` | Scan registered sources for discoverable skills |
| `a` or `Space` | Add selected discovery to the active profile |
| `j` / `k` | Move between discovered skills |

The source path must be a **local directory** containing a `skills/`, `.agents/skills/`, or `.claude/skills/` subdirectory with `SKILL.md` files.

**Example:** register the repo's own bundled skills:
```
Name: Bundled Skills
Path: /path/to/skillweaver/.agents/skills
```

---

### System Settings `[4]`

View runtime state — how many profiles and sources are stored, which profile is the default.

Settings are persisted automatically to `~/.config/skillweaver/profiles.json`.

---

### Help `[5]`

Full keyboard reference with all keybindings.

---

## Install and Safety Features

| Feature | Status |
|---------|--------|
| Install preview with blocker enforcement (traversal detection) | ✅ |
| Safe ZIP extraction (path traversal rejection) | ✅ |
| Real skill download from GitHub ZIP sources | ✅ |
| Symlink-first with copy fallback | ✅ |
| `skills-lock.json` generation (skills only) | ✅ |
| Managed block mutation in `AGENTS.md` / `CLAUDE.md` | ✅ |
| `SKILL.md` YAML frontmatter parsing | ✅ |
| Multi-profile install (existing skills are skipped) | ✅ |

**Managed block markers** used in target Markdown files:
```markdown
<!-- BEGIN SKILLWEAVER RULES -->
<!-- END SKILLWEAVER RULES -->
```

---

## Profile JSON Format

Profiles are stored and imported as JSON. Example:

```json
{
  "schema_version": 1,
  "default_profile_id": "my-profile",
  "profiles": [
    {
      "id": "my-profile",
      "name": "My Profile",
      "description": "Optional description",
      "skills": ["context-engineer", "prd-writer", "spec-writer"],
      "rules": ["use-conventional-commits"],
      "install_mode": "auto"
    }
  ],
  "sources": [
    {
      "name": "Engineering Skills",
      "root": "https://github.com/GustavoGutierrez/engineering-skills/raw/refs/heads/main/skills",
      "source_type": "zip"
    }
  ]
}
```

- `skills` — list of skill names (download URL = `source.root + "/" + skill-name + ".zip"`)
- `rules` — list of rule identifiers
- `sources` — where skills come from (`source_type`: `local`, `zip`, or `git`)
- `install_mode` — `auto`, `symlink`, or `copy`

---

## Verified Commands

```bash
cargo check     # Compilation check
cargo test      # 22 tests (6 app state + 16 safety/install)
cargo build     # Build binary at target/debug/skillweaver
```

---

## Tech Stack

| Layer | Crate |
|-------|-------|
| TUI rendering | `ratatui` 0.30, `crossterm` 0.29 |
| Serialization | `serde`, `serde_json`, `serde_yaml` |
| ZIP handling | `zip` 4 |
| Error handling | `color-eyre`, `thiserror` |
| System paths | `dirs` 6 |
| Testing | `tempfile` |

---

## License

MIT — see [LICENSE](./LICENSE).

## Author

**Ing. Gustavo Gutiérrez** — [GitHub](https://github.com/GustavoGutierrez)
