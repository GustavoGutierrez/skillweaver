# Product Requirements Prompt (PRP)

## 1. Project Overview

**Project:** SkillWeaver  
**One-line Description:** A cross-platform Rust + Ratatui CLI/TUI for discovering, previewing, installing, and sharing profiles of AI agent skills and rules across projects with safe lockfile and Markdown updates.

**Pattern Classification:** **Pattern B — Multi-component developer tooling system**  
SkillWeaver is not an AI-native runtime system. It is a developer workflow tool with local state, remote artifact discovery/download, cross-platform filesystem behavior, structured document mutation, and multiple integration targets.

**Estimated Delivery Shape:**
- MVP: 8–12 weeks
- v1.1 hardening: 3–5 additional weeks
- v2.0 ecosystem expansion: post-MVP

**Primary Users:**
- Individual developers bootstrapping personal AI workflow capabilities
- Tech leads / team enablers standardizing skills and rules across projects

**Business / Product Context:**
The current workflow for installing agent skills and rules is fragmented, manual, and inconsistent. Users must download ZIPs individually, extract them into the correct folders, maintain `skills-lock.json`, and update `AGENTS.md` or `CLAUDE.md` manually without damaging project-specific content. SkillWeaver centralizes this workflow into a repeatable, inspectable, and safe installation experience aligned with the open Agent Skills ecosystem.

---

## 2. Problem Statement

Developers and tech leads face repetitive, fragile setup work when they need to install and maintain agent skills and rules across projects.

This causes:
- inconsistent project setup
- time wasted on repeated manual configuration
- lockfile drift and unclear installed state
- accidental corruption or duplication in `AGENTS.md` / `CLAUDE.md`
- low trust in downloaded remote skill artifacts

We know this matters because the product goal is explicitly to:
- reduce manual configuration by at least 70%
- support reusable profiles across multiple projects
- preserve existing project setup without destructive writes
- make skills and rules inspectable before installation

---

## 3. Success Criteria

### Primary Metric
- Reduce manual configuration steps for a standard profile installation by **>= 70%** relative to the current manual process.

### Secondary Metrics
- Install a profile containing **10 skills and 3 rules in <= 3 minutes** on a supported machine with network access.
- Achieve **>= 95% successful non-destructive installs** in acceptance tests.
- Achieve **100% valid `skills-lock.json` output** for successful installs.
- Support **zero-manual-edit profile export/import** across at least one cross-machine migration scenario.
- Re-running the same installation against unchanged artifacts must be **idempotent** and skip redundant downloads.

### Minimum Success Threshold
- >= 50% reduction in manual configuration effort
- >= 90% non-destructive install success rate
- lockfile generation works reliably for all supported happy-path installs

---

## 4. User Stories (Jobs-to-be-Done)

### Job Story 1 — Reusable profile management
When I need a repeatable setup for my projects or team, I want to create and manage named profiles of skills and rules, so I can reuse the same configuration without rebuilding it manually.

### Job Story 2 — Low-friction project installation
When I start or update a project, I want to install a full profile into the target repository, so I can enable the required skills and rules with minimal manual setup.

### Job Story 3 — Installed-state trust
When I rerun installs or audit a project, I want a lockfile that records artifact provenance and hashes, so I can detect drift and avoid unnecessary re-downloads.

### Job Story 4 — Pre-install understanding
When I discover a new skill or rule source, I want to inspect metadata and provenance before installation, so I can decide whether I trust and want that artifact.

### Job Story 5 — Safe project extension
When I apply rules into a project, I want the tool to update agent-facing Markdown files safely, so I can extend behavior without damaging existing documentation.

### Job Story 6 — Team portability
When I want to standardize setup across machines or teammates, I want to export and import profiles, so that the same workflow can be reproduced elsewhere.

---

## 5. Functional Requirements

### P0 — Core MVP Requirements

**FR-001 Profile CRUD**  
The system must let users create, edit, duplicate, delete, save, and reload named profiles from the TUI.

**FR-002 Profile persistence**  
The system must store profiles in a user-scoped global JSON configuration directory with schema validation and forward-compatible versioning.

**FR-003 Repository source registration**  
The system must let users register supported repository sources and direct ZIP URLs for skills and rules.

**FR-004 Repository scanning**  
The system must detect compatible skill locations in repositories, including at minimum `skills/` and agent-compatible skill directories.

**FR-005 Metadata extraction**  
The system must parse `SKILL.md` frontmatter when present and extract at minimum `name` and `description`, plus triggers or supported metadata when available.

**FR-006 Profile installation**  
The system must install a selected profile into a target project by creating required directories, downloading artifacts, extracting them, and updating managed project files.

**FR-007 Install deduplication**  
The system must compare desired state against local installed state and lockfile/hash data, skipping unchanged artifacts.

**FR-008 Lockfile management**  
The system must create and update `skills-lock.json` according to the required schema and include provenance/hash fields per installed **skills only**.

**FR-009 Safe Markdown updates**  
The system must update `AGENTS.md` if present, otherwise `CLAUDE.md`, while preserving content outside the managed insertion area.

**FR-010 Duplicate protection for rules**  
The system must avoid duplicating previously installed identical rule entries in agent-facing Markdown files.

**FR-011 Preview before install**  
The system must show artifact source, discovery method, metadata, and intended write impact before installation.

**FR-012 Installation mode support**  
The system must support symlink-first installs where supported and copy fallback when symlinks are unavailable or denied.

**FR-013 Error visibility**  
The system must surface actionable errors for malformed config, malformed lockfiles, YAML parse failures, ZIP download failures, extraction failures, permission failures, and unsupported layouts.

**FR-013a Blocking policy enforcement**  
The system must apply a tiered blocking policy for remote artifacts:
- **Allow + confirm:** reachable remote source, valid archive, valid target structure, but unverified provenance/signature/ref
- **Hard block:** unreachable source, invalid archive, path traversal attempt, extraction outside target directory, hash mismatch after download when hash is expected, or unsupported artifact structure that cannot be installed safely
- **Allow silently only for local non-network operations:** inspection of already-saved local profile/config data

**FR-014 Profile export/import**  
The system must export profiles as portable JSON and import them with validation and conflict handling.

### P1 — Important Post-MVP Requirements

**FR-015 Dry-run / install preview mode**  
The system should support a non-mutating preview mode that shows writes, downloads, and conflict points without applying changes.

**FR-016 Better compatibility mapping**  
The system should support additional agent host path mappings beyond the initial `.agents/skills/` and `.claude/skills/` focus.

**FR-017 Richer provenance display**  
The system should display version/ref metadata when available from source providers.

**FR-018 Improved rule conflict detection**  
The system should detect ambiguous Markdown insertion states and guide the user through safe remediation.

### P2 — Future Requirements

**FR-019 Pluggable source providers**  
Support source providers beyond GitHub and direct ZIP URLs.

**FR-020 Update / rollback workflows**  
Support artifact diffing, controlled updates, and rollback flows.

**FR-021 Version pinning**  
Support pinning to tags, commits, or equivalent immutable references where providers expose them.

---

## 6. Non-Functional Requirements

### Performance
- **NFR-001:** Install preview metadata for a highlighted artifact should render in the TUI without noticeable UI blocking under normal local conditions.
- **NFR-002:** A standard profile of 10 skills and 3 rules must install in <= 3 minutes under expected network conditions.
- **NFR-003:** The TUI must remain responsive during downloads and extraction via async/background task handling.

### Reliability
- **NFR-004:** Re-running the same installation against unchanged sources must be idempotent.
- **NFR-005:** Partial failures must be reported clearly without incorrectly marking failed artifacts as installed.
- **NFR-006:** The system must preserve prior successful work during partial install failures unless an explicit rollback feature exists.

### Safety
- **NFR-007:** Existing content outside managed sections in `AGENTS.md` / `CLAUDE.md` must never be overwritten silently.
- **NFR-008:** Malformed config or lockfile data must never be replaced silently.
- **NFR-009:** The system must warn users that remote skill/rule artifacts are a supply-chain surface.
- **NFR-009a:** ZIP extraction must reject path traversal and any archive entry that resolves outside the intended target directory.
- **NFR-009b:** Rules must not be tracked in `skills-lock.json`; rule installation state must be derived from the managed block content and source/profile configuration rather than the skill lockfile.

### Cross-platform Compatibility
- **NFR-010:** The MVP must support latest stable macOS, Linux, and Windows releases.
- **NFR-011:** Platform-specific filesystem behavior such as path separators, symlink permissions, and writable directory checks must be abstracted behind platform-aware services.

### Usability
- **NFR-012:** All UI text must be in English.
- **NFR-013:** The TUI must use clear status messaging for success, warnings, errors, and partial completion.
- **NFR-014:** The product should be understandable by a first-time user without requiring prior knowledge of the repository layout.

### Accessibility
- **NFR-015:** TUI flows must be fully keyboard navigable.
- **NFR-016:** Color usage must not be the only signal for warnings, errors, or selected state.

---

## 7. Technical Constraints

### Language and Runtime
- Must be implemented in **Rust**.
- Must use **Ratatui** for the TUI layer.
- Must use async/background execution patterns compatible with Rust TUI responsiveness.

### Storage and Persistence
- MVP persistence must use **JSON files only**.
- MVP must not require a database.
- Config must be stored in a user-scoped global config directory.

### Installation and Distribution
- Public release under **MIT** license.
- Must support public distribution for macOS/Linux via **Homebrew**.
- Must provide a Windows-compatible public binary distribution path.
- For the current release strategy, Windows binaries may be compiled and distributed as compressed `.zip` release assets.

### Compatibility Constraints
- Must align with open Agent Skills conventions around `SKILL.md`, YAML frontmatter, and common repository layouts.
- Must prioritize `.agents/skills/` and `.claude/skills/` while remaining extensible for additional host mappings.

### Scope Constraints
- No authentication or hosted sync in MVP.
- No remote publishing of skills/rules in MVP.
- No runtime AI orchestration or inference features.

---

## 8. Data Requirements

### Data Sources
- Remote skill repositories with ZIP artifacts
- Direct ZIP URLs
- `SKILL.md` files with YAML frontmatter
- Local project filesystem
- Global JSON profile/config files

### Core Data Models

#### Profile
- `id`
- `name`
- `description` (optional)
- `skills[]`
- `rules[]`
- `targetAgents[]`
- `installMode` (`symlink` | `copy` | `auto`)
- `sourceRefs[]`
- `version`

#### Skill Source
- `name`
- `source`
- `sourceType`
- `zipUrl` (when explicit)
- `skillPath`
- `metadata` (`name`, `description`, `triggers`, optional extra fields)

#### Rule Source
- `name`
- `source`
- `sourceType`
- `zipUrl`
- `rulePath`
- `metadata` (`name`, `description`, `trigger` when available)

#### Lockfile
- `version`
- `skills` map
  - `source`
  - `sourceType`
  - `skillPath`
  - `computedHash`

**Important constraint:** rules are **not** stored in `skills-lock.json` for the MVP.

### Rule State Strategy
- Rule state must be represented in the managed Markdown block generated by SkillWeaver, not in the skill lockfile.
- Each generated rule row should contain enough visible information to identify the rule source and trigger.
- The system may maintain transient in-memory rule install decisions during execution, but persisted rule tracking in MVP must not expand `skills-lock.json`.

### Data Validation Rules
- Profile names must be unique within local storage.
- Imported profiles must match the expected schema.
- `SKILL.md` frontmatter must validate required fields before being treated as trusted metadata.
- Malformed JSON/YAML must be reported explicitly.

### Data Privacy and Retention
- No personal cloud data in MVP.
- All persistent state remains local to the user machine.
- Remote requests are limited to artifact discovery and artifact download.

---

## 9. UI/UX Requirements

### Primary Screens
- **Dashboard** — current profile, selected artifacts, install actions, status footer
- **Profiles** — create/edit/duplicate/delete/export/import profiles
- **Repositories** — add repository sources, browse discovered skills/rules, register direct ZIP URLs
- **System Settings** — install mode defaults, target agent preferences, path and environment settings

### UI Information Architecture

#### Top-level Navigation
- `1` Dashboard
- `2` Profiles
- `3` Repositories
- `4` System Settings
- `h` Help
- `q` Quit

#### Shared Layout Model
- **Header:** app name, version, top-level tabs
- **Main split view:** left navigation/context panel + right detail/work panel
- **Footer/status bar:** current action state, errors, success messages, key hints

This layout must remain consistent across screens so users do not have to relearn navigation by view.

### Core UX Principles
- Show the installable unit, not just the repository.
- Make provenance visible before trust decisions.
- Preview file impact before writes.
- Preserve existing project content unless explicitly instructed otherwise.
- Prefer progressive disclosure: simple happy path first, detail pane second.
- Never ask the user to trust a remote artifact blindly when a safe preview or warning can be shown first.

### Visual and Interaction Standards
- Primary highlight color: **cyan**
- Success state color: **green**
- Warning state color: **yellow**
- Error state color: **red**
- Borders and muted metadata: **dim / gray-style treatment**
- Color must reinforce meaning, but labels/icons/text must still communicate state without relying only on color.
- All interactions must be keyboard-first.
- Selection state must be visible with both cursor focus and explicit markers such as `[x]`, `>`, or bordered emphasis.

### Core User Flows
1. Create or select a profile.
2. Add repository sources or direct ZIP sources.
3. Inspect discovered skills/rules and metadata.
4. Toggle inclusion into the profile.
5. Choose target agents and install mode.
6. Preview intended writes and trust warnings.
7. Install and review status.
8. Export/import profile if needed.

### Screen Definitions

#### 9.1 Dashboard Screen
**Purpose:** give the user the fastest path to inspect a profile and install it into the current project.

**Layout responsibilities**
- Left panel: active profile list and quick actions
- Right panel: skills/rules contained in the selected profile
- Lower-right detail area or contextual panel: metadata for the highlighted item
- Footer: install/result status and key hints

**Required data visible on screen**
- active profile name
- skills selected in the profile
- source repository or ZIP origin for highlighted item
- description and triggers from metadata when available
- install result or pending action state

**ASCII reference**
```text
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

#### 9.2 Profiles Screen
**Purpose:** manage reusable profile definitions.

**Capabilities on this screen**
- create profile
- rename profile
- duplicate profile
- delete profile with confirmation
- assign skills/rules to profile
- choose target agents and install mode
- export/import profile

**Expected layout**
- Left panel: profile list
- Right panel: selected profile editor
- Modal dialogs for create, rename, duplicate, delete confirmation, import conflict handling

#### 9.3 Repositories Screen
**Purpose:** discover installable artifacts and register sources.

**Capabilities on this screen**
- add repository source
- add direct ZIP URL
- scan repository
- browse discovered skills/rules
- inspect parsed metadata
- add selected items to a profile

**Expected layout**
- Left panel: configured sources
- Center or right panel: discovered artifacts
- Detail panel: metadata, provenance, trust warnings, compatibility notes

#### 9.4 System Settings Screen
**Purpose:** manage app-wide behavior defaults.

**Capabilities on this screen**
- default install mode (`auto`, `symlink`, `copy`)
- default agent targets
- config path visibility
- optional safety preferences for provenance warnings and install confirmations

**Expected layout**
- grouped settings list
- inline value editor or modal editor
- save/cancel feedback in footer

### Required Modal / Dialog Patterns

#### Confirmation Dialog
Used for:
- delete profile
- install after trust warning
- overwrite/replace import conflict

Must show:
- clear action title
- impacted object
- confirm and cancel keys

#### Preview Dialog
Used for:
- pre-install write impact
- provenance/trust review
- dry-run result summary

Must show:
- artifact list
- source/provenance
- intended writes (create/update/link/copy)
- warnings before continue

#### Blocking Warning Dialog
Used for:
- remote artifact with incomplete provenance
- unsigned/unpinned source where install is still allowed by policy

Must show:
- why the artifact is not fully verified
- what is still known (source URL/repository, metadata, hash if available)
- explicit continue/cancel choice

#### Hard Block Dialog
Used for:
- unsafe ZIP structure
- extraction path traversal
- target path escape attempt
- unreadable target that makes safe install impossible

Must show:
- why installation is blocked
- which artifact caused the block
- remediation guidance

#### Error Dialog
Used for:
- malformed JSON/YAML
- permission failures
- repository scan failure
- ZIP extraction failure

Must show:
- what failed
- likely cause
- next recommended action

### UX States That Must Exist
- Empty state: no profiles
- Empty state: no compatible skills found
- Loading state: repository scan in progress
- Partial completion state: some artifacts succeeded, some failed
- Warning state: metadata incomplete or invalid
- Error state: malformed JSON/YAML, download failure, extraction failure, permission failure
- Success state: install completed with lockfile update summary

### Example Empty States

#### No Profiles Empty State
- Message: `No profiles yet.`
- Guidance: `Create a profile to start grouping skills and rules.`
- Primary action hint: `[n] New Profile`

#### No Compatible Skills Found Empty State
- Message: `No compatible skills were found in this source.`
- Guidance: `Try another repository or add a direct ZIP URL.`

### Footer and Keybinding Contract
- Footer must always show the most relevant keybindings for the current context.
- Global keys should remain stable across screens where possible.
- Minimum shared bindings:
  - `q` quit
  - `h` help
  - `↑↓` navigate lists
  - `←→` switch columns or tabs when applicable
  - `Enter` confirm/select
  - `Esc` cancel/close dialog
  - `Space` toggle selection in multi-select lists

### UI Constraints
- All UI copy must be in English.
- Must work in terminal environments on macOS, Linux, and Windows.
- Must use keyboard navigation only; mouse support is optional.

### Implementation Notes for Ratatui
- The TUI should use a **header / main / footer** layout split consistently.
- Multi-pane screens should use a **30/70 or similar left-right split** unless a task-specific layout is better.
- Stateful lists/tables should preserve current selection when refreshing async data.
- Async repository scans and downloads must not block redraws.
- Status messages should be short, contextual, and overwritten by newer transient states only when appropriate.

### AGENTS.md / CLAUDE.md Mutation Strategy

#### Chosen Strategy
Use a **managed block with explicit markers** and replace only the content inside that block on subsequent runs.

#### Required Markers
```md
<!-- BEGIN SKILLWEAVER RULES -->
<!-- END SKILLWEAVER RULES -->
```

#### Write Rules
- If `AGENTS.md` exists, write there.
- Else if `CLAUDE.md` exists, write there.
- Else create the supported default file according to product policy.
- On first install, append a dedicated managed section near the end of the file.
- On subsequent installs, replace only the content between the SkillWeaver markers.
- Do **not** attempt to merge into arbitrary existing user-authored tables.
- Do **not** re-render the whole Markdown document from an AST if that would reformat unrelated user content.

#### Managed Section Shape
The managed section should contain:
- a heading such as `## SkillWeaver Managed Rules`
- a short explanatory note indicating the block is tool-managed
- a deterministic Markdown table with rule name, source, trigger, and status/context as needed

#### Why This Strategy Was Chosen
- It preserves user-authored content outside the managed section.
- It avoids brittle table-merging heuristics.
- It prevents duplicate rule rows by making the generated block authoritative.
- It is safer than full-document AST round-tripping, which may reformat unrelated content.
- It is easier to test with golden files across platforms.

---

## 10. Risks & Assumptions

### Risks

**R1 — Cross-platform filesystem inconsistencies**  
Windows symlink behavior and permissions differ from macOS/Linux.

**Mitigation:** abstract FS operations and test on all supported targets.

**R2 — Unsafe or malformed remote artifacts**  
Remote ZIPs may be missing metadata, malformed, or contain misleading instructions.

**Mitigation:** preview, provenance visibility, warnings, validation, and explicit confirmation before install.

**R2a — Unsafe archive extraction paths**  
ZIP entries may attempt path traversal or write outside the intended target directory.

**Mitigation:** normalize and validate every archive path before extraction and hard-block unsafe archives.

**R3 — Markdown mutation corruption**  
Naive string replacement can break `AGENTS.md` / `CLAUDE.md`.

**Mitigation:** use structured parsing/writing and managed insertion boundaries.

**R4 — Repository layout inconsistency**  
Not all repositories will expose the same layout for skills/rules.

**Mitigation:** support convention-based discovery plus manual ZIP registration.

**R5 — Lockfile drift**  
Users may edit installed files manually after extraction.

**Mitigation:** compare hashes/state on reinstall and surface drift clearly.

**R6 — Distribution complexity**  
Homebrew alone does not solve Windows distribution.

**Mitigation:** define GitHub Releases/public binary workflow early.

### Assumptions
- Users are willing to manage profiles locally as JSON in MVP.
- Most useful skill repositories will expose either discoverable layouts or direct ZIP URLs.
- Installing skills/rules into project-local directories is the dominant initial use case.
- A tiered blocking policy is sufficient for MVP trust decisions: warn+confirm for incomplete provenance, hard-block for unsafe archive/install conditions.
- Agent host compatibility can begin narrow and expand over time.

---

## 11. Out of Scope

### Out of Scope for MVP
- Cloud sync or hosted profile sharing
- Authentication and user accounts
- Database-backed persistence
- Skill/rule publishing back to remote repositories
- Automated dependency resolution between skills
- Runtime prompt execution or model inference
- Marketplace search with user identity or reputation systems
- Full multi-agent host matrix support from day one
- Artifact rollback engine
- Advanced version pinning and update orchestration
- Rule persistence inside `skills-lock.json`

### Explicitly Not Building Now
- A web UI
- A mobile companion app
- A proprietary skills format incompatible with `SKILL.md`
- Silent auto-update behavior for installed artifacts

---

## 12. Open Questions

| ID | Question | Owner | Needed By | Impact if Unanswered |
|---|---|---|---|---|
| Q1 | Should the default file creation target when neither file exists be `AGENTS.md` only, or should it depend on selected target agent? | Product / Engineering | Before Markdown writer implementation | File creation behavior remains ambiguous |
| Q2 | Should future releases support trusted-source allowlists to reduce repeated confirmations for known repositories? | Product / Security stance | Before v1.1 trust UX planning | Warning fatigue mitigation remains unspecified |
| Q3 | Should global installation of artifacts be MVP or deferred to v1.1? | Product | Before final MVP scope lock | Affects persistence and target path design |
| Q4 | What minimum agent host targets beyond OpenCode and Claude should be officially supported in v1.1? | Product / Ecosystem strategy | Before compatibility abstraction design | Path mapping abstraction may be underdesigned |
| Q5 | Should rule state later receive its own dedicated `rules-lock.json`, or remain derived from managed Markdown and profile config? | Product / Engineering | Before post-MVP state design | Future rule lifecycle may be constrained |

---

## Implementation Readiness Notes

This PRP is intended for AI-assisted or human-assisted implementation. Any implementation prompt derived from it should include:
- the PRD and PRP together as source context
- the required `skills-lock.json` schema
- the non-destructive mutation constraints for `AGENTS.md` / `CLAUDE.md`
- the cross-platform filesystem constraints
- the requirement to keep UI text in English
- validation criteria for malformed JSON, YAML, ZIP, permissions, and drift cases

## Build Checklist

- [ ] Profile CRUD is defined and testable
- [ ] Install flow is previewable before writes
- [ ] Lockfile behavior is explicit and schema-bound
- [ ] Markdown mutation safety rules are explicit
- [ ] Cross-platform constraints are visible
- [ ] Non-goals are explicit enough to prevent scope creep
- [ ] Open questions are assignable and actionable
