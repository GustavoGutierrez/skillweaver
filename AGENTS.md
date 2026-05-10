# AGENTS.md

## Scope: what this repo actually is
- This snapshot is **not** the SkillWeaver app source tree. There is **no** root `Cargo.toml`, `src/`, CI workflow, `opencode.json`, or existing agent instruction file.
- Treat the repo as a **docs + installed-skills bundle** unless new source files are added later.

## Trust these sources first
- Prefer `skills-lock.json` and real files under `.agents/skills/` over `README.md` claims.
- `README.md` describes a Rust app architecture and CI setup that are **not present** in this snapshot. Verify before repeating those claims.

## What is here
- Root high-signal files: `README.md`, `PRD.md`, `PRP.md`, `skills-lock.json`.
- Installed skills live in `.agents/skills/<skill-name>/`.
- The only runnable code in-repo is the bundled Ratatui templates under `.agents/skills/ratatui-tui/assets/templates/*`.

## Commands you can verify
- **Repo root:** no verified build / test / lint / typecheck command exists.
- **Ratatui template crates only** (run inside a template directory such as `.agents/skills/ratatui-tui/assets/templates/component-app`):
  - `cargo run`
  - `cargo build --release`
  - `cargo fmt`
  - `cargo clippy --all-features`

## Repo-local skills available
- `context-engineer` — reduce and structure LLM context.
- `documentation-architect` — design doc systems / information architecture.
- `prd-writer` — write PRDs.
- `prp-writer` — write PRPs for AI-assisted delivery.
- `ratatui-tui` — build Rust TUIs with Ratatui; includes templates and references.
- `spec-writer` — write or normalize `*.spec.md` specs.
- `story-refiner` — turn stories into sprint-ready backlog items.
- `traceability-manager` — maintain requirement-to-implementation traceability.
- `validation-strategist` — design layered validation / QA strategies.

## Rule / Markdown mutation constraints
- Product intent from `PRD.md` + `PRP.md`: update `AGENTS.md` if present; otherwise `CLAUDE.md`.
- Preserve all user-authored content outside the managed section.
- If you need a managed block, reuse the exact markers from `PRP.md`:
  - `<!-- BEGIN SKILLWEAVER RULES -->`
  - `<!-- END SKILLWEAVER RULES -->`
- Do **not** track rules in `skills-lock.json`; that lockfile is for installed **skills** only.

## Safety constraints worth preserving
- Treat remote skill/rule artifacts as a **supply-chain surface**.
- ZIP extraction must reject **path traversal** and any write outside the target directory.
- Installation mode is **symlink-first with copy fallback** when symlinks are unavailable.
- User-facing UI copy and metadata are expected to be **in English**.
- TUI workflows are expected to be **keyboard-first**.

## Known drift / contradictions
- `README.md` claims a root Rust app, CI enforcement, and specific crate versions; the current repo snapshot does not verify that.
- README stack versions (`ratatui 0.26`, `crossterm 0.27`) differ from bundled Ratatui template crates (`ratatui 0.30`, `crossterm 0.29`). Trust the template manifests when working on those templates.
- `PRP.md` still leaves one product ambiguity open: what file to create when neither `AGENTS.md` nor `CLAUDE.md` exists. Do not invent that rule without an explicit decision.
