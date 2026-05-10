# Product Requirements Document (PRD)

## 1. Executive Summary

### Problem Statement
Installing and maintaining AI agent skills and rules across projects is still too manual, inconsistent, and fragile. Developers and tech leads must download ZIP artifacts one by one, extract them into the correct folders, keep lockfiles in sync, and update `AGENTS.md` or `CLAUDE.md` without breaking existing project conventions.

### Proposed Solution
Build **SkillWeaver**, a public MIT-licensed Rust + Ratatui CLI/TUI that manages reusable profiles of skills and rules, installs them in bulk, tracks them with `skills-lock.json`, and updates project agent configuration files safely and non-destructively across macOS, Linux, and Windows. The product must align with the emerging open Agent Skills ecosystem conventions used by tools such as `npx skills`, OpenCode-compatible skill loading, and `SKILL.md` frontmatter-based discovery.

### Success Criteria
- Reduce manual configuration steps for a standard profile installation by **at least 70%** versus the current manual workflow.
- Allow a user to install a profile containing **10 skills and 3 rules in 3 minutes or less** on a supported machine with network access.
- Achieve **>= 95% successful non-destructive installs** in acceptance tests, preserving pre-existing `AGENTS.md` / `CLAUDE.md` content.
- Ensure **100% valid `skills-lock.json` generation/update** for all successful installations.
- Enable profile export/import with **zero required manual edits** for at least one cross-machine migration scenario.

---

## 2. User Experience & Functionality

### User Personas

#### 1. Individual Developer
Needs to bootstrap personal AI workflow capabilities quickly without manually installing each skill or rule in every repository.

#### 2. Tech Lead / Team Enabler
Needs to define curated, shareable skill/rule profiles for teams and roll them out consistently across multiple projects without damaging local project setup.

### User Stories

#### Story 1 — Create and manage profiles
As an individual developer or tech lead, I want to create and manage named profiles of skills and rules so that I can reuse a consistent setup across multiple projects without rebuilding it manually each time.

**Acceptance Criteria**

**AC1 — Create a profile**
- Given the user is in the Profiles view and no profile with the same name exists
- When the user creates a new profile and enters a valid name
- Then the profile is saved to the global JSON configuration store
- And the new profile appears in the profile list without restarting the application

**AC2 — Edit profile contents**
- Given an existing profile is selected
- When the user adds or removes skills, rules, or target agent preferences
- Then the updated profile is persisted
- And reopening the TUI shows the same saved values

**AC3 — Duplicate a profile**
- Given an existing profile is selected
- When the user chooses Duplicate Profile and provides a unique name
- Then a new profile is created with the same contents as the original
- And changes to the duplicate do not modify the original profile

**AC4 — Delete a profile safely**
- Given an existing profile is selected
- When the user chooses Delete Profile and confirms the action
- Then the profile is removed from the global configuration store
- And other saved profiles remain unchanged

**Negative and edge scenarios**
- If the user enters an empty or invalid profile name, the TUI must show a validation message and not save the profile.
- If the user attempts to save a duplicate profile name, the TUI must block the action and offer rename or overwrite-explicitly behavior.
- If the config file is unreadable or unwritable, the TUI must surface an actionable error and keep the current in-memory state intact.

#### Story 2 — Install a profile into a project
As a developer, I want to install a full profile into a target project so that I can enable the required skills and rules with minimal manual setup.

**Acceptance Criteria**

**AC1 — Prepare the project structure**
- Given the user selects Install Profile for a target project
- When the install starts
- Then the CLI creates the required directories for skills and rules if they do not already exist
- And the CLI does not modify unrelated project directories

**AC2 — Install only missing or changed artifacts**
- Given a selected profile contains skills or rules already present in the project
- When the installer compares desired state with local installed state and lockfile/hash data
- Then unchanged artifacts are skipped
- And only missing or changed artifacts are downloaded and installed

**AC3 — Install to supported agent targets**
- Given the selected profile targets OpenCode, Claude, or both
- When the installation completes successfully
- Then the artifacts are placed or linked into the correct target directories for the selected agent hosts

**AC4 — Respect installation mode**
- Given the environment supports symlinks and the profile is configured for symlink mode
- When the install runs
- Then the CLI creates symlinks instead of duplicate copies
- And if symlinks are unavailable or denied by the host OS
- Then the CLI falls back to copy mode or prompts the user according to configuration

**Negative and edge scenarios**
- If a ZIP download fails, the CLI must report which artifact failed and must not mark it as installed.
- If extraction fails for one artifact, the CLI must preserve successfully installed prior artifacts and report partial completion clearly.
- If the target project path is not writable, the CLI must stop before mutating files and show a permission error.

#### Story 3 — Track installed artifacts with a lockfile
As a developer, I want a lockfile that records installed artifacts and their provenance so that I can detect drift, avoid unnecessary re-downloads, and trust what is installed.

**Acceptance Criteria**

**AC1 — Generate a valid lockfile**
- Given at least one skill is installed successfully
- When the installation workflow persists project state
- Then the CLI creates or updates `skills-lock.json`
- And the file matches the defined schema exactly

**AC2 — Record provenance and hash data**
- Given a skill has been installed
- When the lockfile entry is written
- Then the entry includes `source`, `sourceType`, `skillPath`, and `computedHash`

**AC3 — Use lockfile state before download**
- Given the project already contains a `skills-lock.json`
- When the user installs a profile again
- Then the CLI compares desired artifacts against lockfile state before downloading
- And unchanged artifacts are skipped

**AC4 — Reinstall only affected artifacts**
- Given a lockfile entry is missing or the computed hash no longer matches
- When the user runs install or update
- Then only the affected artifacts are reinstalled
- And unrelated artifacts remain untouched

**Negative and edge scenarios**
- If the existing lockfile is malformed JSON, the CLI must surface an error and offer a safe recovery path instead of silently replacing it.
- If the installed artifact exists on disk but is missing from the lockfile, the CLI must treat it as unmanaged or drifted state and explain the decision.

#### Story 4 — Discover skill metadata from repositories
As a user, I want SkillWeaver to inspect repositories and extract skill metadata automatically so that I can understand source, purpose, and triggers before installation.

**Acceptance Criteria**

**AC1 — Scan compatible repositories**
- Given the user adds a supported repository source
- When SkillWeaver scans the repository
- Then it detects compatible skill locations such as `skills/` or agent-compatible skill directories
- And it lists discovered installable artifacts in the TUI

**AC2 — Support manual ZIP registration**
- Given the repository layout is not discoverable automatically
- When the user provides a direct ZIP URL and artifact details
- Then the artifact is stored as an installable source in the profile or repository catalog

**AC3 — Parse `SKILL.md` frontmatter**
- Given a discovered skill contains a valid `SKILL.md`
- When metadata is parsed
- Then the CLI extracts at minimum `name` and `description`
- And it extracts triggers or other supported metadata when present

**AC4 — Show metadata before installation**
- Given a skill is highlighted in the TUI
- When its metadata is available
- Then the detail pane shows source, artifact name, description, and trigger information in English

**Negative and edge scenarios**
- If `SKILL.md` is missing, the CLI must still allow installation when the artifact is otherwise valid, but it must show limited metadata.
- If YAML frontmatter is invalid, the CLI must report a metadata parsing warning and avoid presenting incorrect values.
- If repository scanning returns no compatible skills, the CLI must state that explicitly and suggest manual ZIP registration.

#### Story 5 — Update project rules safely
As a tech lead, I want rules added into project agent files safely so that I can extend agent behavior without damaging existing project documentation or conventions.

**Acceptance Criteria**

**AC1 — Select the correct target file**
- Given a target project contains `AGENTS.md`
- When rule installation runs
- Then SkillWeaver updates `AGENTS.md`
- And if `AGENTS.md` does not exist but `CLAUDE.md` does
- Then SkillWeaver updates `CLAUDE.md` instead

**AC2 — Preserve existing content**
- Given the target file already contains user-authored documentation
- When new rules are applied
- Then existing content outside the managed insertion area remains unchanged

**AC3 — Append structured rule entries**
- Given one or more rules include metadata such as name, description, or trigger
- When the file is updated
- Then new rules are added in a structured table or managed section
- And trigger information is included when available

**AC4 — Prevent duplicate entries**
- Given the same rule has already been installed previously
- When the install is run again
- Then the CLI does not duplicate the existing rule entry

**Negative and edge scenarios**
- If neither `AGENTS.md` nor `CLAUDE.md` exists, the CLI must create the appropriate supported file according to product rules.
- If the file structure cannot be parsed safely, the CLI must stop and request manual review rather than corrupt the document.

#### Story 6 — Share profiles between machines and users
As a team enabler, I want to export and import profiles so that teams can share a standard setup across machines and projects without rebuilding it manually.

**Acceptance Criteria**

**AC1 — Export a portable profile**
- Given a saved profile exists
- When the user chooses Export Profile
- Then the CLI writes a portable JSON file containing the profile definition

**AC2 — Import a valid profile**
- Given the user selects a valid exported profile file
- When the user imports it
- Then the CLI validates the schema before saving
- And the imported profile becomes available in the profile list

**AC3 — Handle naming conflicts safely**
- Given a profile with the same name already exists locally
- When the user imports another profile with that name
- Then the CLI does not overwrite it silently
- And the user is prompted to rename, replace explicitly, or cancel

**Negative and edge scenarios**
- If the import file is invalid JSON or does not match the expected schema, the CLI must reject it with a clear validation error.
- If the imported profile references unknown agent targets or malformed artifact sources, the CLI must flag them before final save.

### Primary UX Flows
- Browse saved profiles.
- Inspect skills and rule metadata.
- Toggle artifacts to include in a profile.
- Install profile into the current project.
- Export/import profiles.
- Manage repositories and system settings.

### Compatibility and Installation Expectations
- The product must support **project-local** installation as the default mode.
- The product should support **global/user-scoped** profile and artifact management as a future-compatible extension.
- When installing to target agent hosts, the product must support both **symlink-first workflows** and **copy fallback workflows** for environments where symlinks are unavailable or undesirable.
- The UX must clearly communicate whether an install will use **copy** or **symlink** semantics before applying changes.

#### Story 7 — Preview and trust before install
As a developer, I want to preview what a skill or rule contains before installation so that I can reduce the risk of installing unsafe or misleading artifacts.

**Acceptance Criteria**

**AC1 — Preview artifact provenance**
- Given a skill or rule is selected in the TUI
- When preview data is available
- Then the CLI shows source repository or direct URL, artifact name, and discovery method

**AC2 — Preview parsed metadata**
- Given the selected artifact contains parseable metadata
- When the user opens the detail pane or preview view
- Then the CLI shows the parsed metadata before installation

**AC3 — Warn about trust-sensitive installs**
- Given the user is about to install artifacts from a remote source
- When the confirmation step is shown
- Then the CLI warns that skills and rules may contain hidden instructions or unsafe content
- And the user can continue or cancel

**AC4 — Show write impact before apply**
- Given installation will create, modify, copy, or link files
- When the user requests install
- Then the CLI shows a pre-install summary of intended writes before applying changes

**Negative and edge scenarios**
- If metadata is incomplete, the preview must still show raw source and artifact identity rather than blank or misleading content.
- If the artifact source cannot be verified or fetched, the CLI must block installation or require an explicit override according to product policy.

### Non-Goals
- No cloud sync, hosted backend, or database in the MVP.
- No marketplace with user authentication in the MVP.
- No automatic publishing of skills or rules back to remote repositories in the MVP.
- No dependency resolution between skills beyond lockfile/hash validation in the MVP.
- No runtime AI inference, model orchestration, or prompt execution inside SkillWeaver.

---

## 3. AI System Requirements (If Applicable)

### Applicability
SkillWeaver is **not** an AI inference product. It is a workflow/package management tool for AI-agent-compatible skills and rules.

### Tool Requirements
- ZIP download and extraction for skills and rules.
- YAML frontmatter parsing from `SKILL.md`.
- Markdown-safe project file updates for `AGENTS.md` / `CLAUDE.md`.
- Cross-platform filesystem operations for config storage, extraction, and optional symlinks.
- Skill/repository discovery compatible with common open Agent Skills repository layouts.

### Evaluation Strategy
- Validate metadata extraction against a fixture set of compliant `SKILL.md` files.
- Validate that installs are idempotent across repeated runs.
- Validate that rules are appended without corrupting existing Markdown.
- Validate that the same profile can be exported from one machine and imported on another supported OS.

---

## 4. Technical Specifications

### Architecture Overview
SkillWeaver will use a **Rust + Ratatui** architecture based on **Model-View-Update (Elm Architecture)**.

#### Core runtime model
- `Model`: application state, active tab, profile selection, artifact selection, install status, errors.
- `View`: Ratatui rendering for dashboard, profile manager, repository browser, and settings.
- `Update`: event handling and action dispatch for keyboard input, async download completion, parse results, and write operations.

#### Core modules
- **Config Manager**: manages global JSON configuration and stored profiles.
- **Repository Scanner**: inspects repository structures and/or direct URLs for skills/rules.
- **Metadata Parser**: extracts frontmatter from `SKILL.md`.
- **Installer**: downloads ZIPs, verifies hashes, extracts artifacts, and updates lockfiles.
- **Rules Writer**: updates `AGENTS.md` / `CLAUDE.md` safely.
- **Symlink Manager**: handles optional linking workflows for supported environments.

#### Data flow
1. User selects a profile.
2. CLI resolves configured repositories or direct ZIP URLs.
3. Metadata is parsed and shown in the TUI.
4. Installer compares desired state with `skills-lock.json`.
5. Missing or changed skills/rules are downloaded and extracted.
6. Managed project files are updated safely.
7. Lockfile and local profile state are persisted.

### Integration Points
- **GitHub-hosted repositories** containing ZIP artifacts and `SKILL.md` files.
- **Direct ZIP URLs** for manual artifact registration.
- **Local filesystem** for target project installation and global configuration.
- **Homebrew distribution** for macOS/Linux-friendly public installation.
- **Windows-compatible binary distribution** for public releases outside Homebrew.

### Discovery and Compatibility Rules
- Skill discovery must assume `SKILL.md` is the canonical entrypoint for a skill directory.
- `SKILL.md` parsing must validate required YAML frontmatter fields at minimum: `name` and `description`.
- Skill names should follow ecosystem-compatible slug rules: lowercase, alphanumeric, and hyphen-separated.
- The installer must recognize that common host targets may use different skill directories, even when SkillWeaver initially prioritizes `.agents/skills/` and `.claude/skills/`.
- The design must preserve compatibility with repository layouts that expose skills under `skills/`, `.agents/skills/`, or similar agent-skill conventions.

### Installation Modes
- **Symlink mode (preferred when supported):** maintains a canonical artifact location and links agent-visible directories to it.
- **Copy mode (fallback):** creates independent copies for environments where symlinks are not supported or not permitted.
- The installer must detect unsupported symlink scenarios and fall back safely instead of failing late.

### Platform Requirements
- Must support the latest stable versions of **macOS**, **Linux**, and **Windows**.
- Must operate **offline after installation**, except when downloading new skills/rules from remote sources.
- Must use **JSON files only** for persisted state in the MVP.
- Must not require a database in the MVP.

### Data Model Requirements

#### Lockfile
`skills-lock.json` must follow this schema:

```json
{
  "version": 1,
  "skills": {
    "skill-creator": {
      "source": "anthropics/skills",
      "sourceType": "github",
      "skillPath": "skills/skill-creator/SKILL.md",
      "computedHash": "5ea13a6d9f0d4bb694405d79acd00cadec0d21bb138c4dd10fcf3c500cb835c2"
    }
  }
}
```

#### Global profile storage
- Profiles must be persisted in a user-scoped global config directory.
- Storage format must be JSON.
- Profiles must support versioning for forward compatibility.

### Security & Privacy
- No user account, login, or personal cloud data in MVP.
- No hidden destructive writes to project files.
- File updates must be scoped to managed sections or safe append operations.
- The installer must never overwrite unrelated project content silently.
- Downloaded artifacts should be hash-checked before lockfile persistence.
- The product must treat installed skills as a **supply-chain surface**, since skills may contain prompt instructions, scripts, or hidden operational behavior.
- The product must preserve or record provenance metadata where possible so users can trace artifact origin.
- The UX should make source repository, version/ref when known, and computed hash visible before or after install.

---

## 5. Risks & Roadmap

### Phased Rollout

#### MVP
- Create and manage profiles locally.
- Scan compatible repositories and register direct ZIP URLs.
- Install skills and rules into projects.
- Maintain `skills-lock.json`.
- Update `AGENTS.md` / `CLAUDE.md` safely.
- Export/import profiles.
- Release public binaries under MIT license.
- Support metadata preview before installation.
- Support symlink-first install with copy fallback.

#### v1.1
- Better repository discovery UX and validation.
- Improved conflict detection for rule insertion.
- Richer install previews and dry-run mode.
- Expanded fixture coverage for Windows filesystem behaviors.
- Optional provenance enrichment in installed metadata.
- Better compatibility mapping for additional agent hosts.

#### v2.0
- Pluggable source providers beyond GitHub/direct ZIP.
- Profile sharing catalog or team bundles.
- Advanced artifact diffing, upgrade workflows, and rollback support.
- Version pinning by tag or commit-like reference where source providers support it.
- Update workflows informed by provenance metadata and content-addressed change detection.

### Technical Risks

#### 1. Cross-platform filesystem inconsistencies
Windows symlink behavior, path handling, and permissions can differ significantly from macOS/Linux.

**Mitigation**
- Abstract filesystem operations behind platform-aware services.
- Test extraction, path handling, and optional symlinks on all supported OS targets.

#### 2. Non-destructive Markdown updates are harder than they look
Blind string replacement can corrupt `AGENTS.md` or duplicate rule entries.

**Mitigation**
- Use a structured parser/writer strategy.
- Restrict managed output to explicit sections or deterministic table rows.
- Add golden-file tests for update scenarios.

#### 3. Remote repository structures may be inconsistent
Not every repository will follow the same ZIP naming or `SKILL.md` conventions.

**Mitigation**
- Support both convention-based discovery and manual URL registration.
- Validate repository compatibility before install.

#### 4. Lockfile drift and hash mismatch edge cases
Users may manually edit installed files after extraction.

**Mitigation**
- Recompute or compare known hashes during install/update paths.
- Offer clear reinstall prompts when drift is detected.

#### 5. Trust and provenance ambiguity
Users may install a skill from a repository or ZIP URL without enough context about source, version, or authenticity.

**Mitigation**
- Surface provenance in the UI wherever available.
- Preserve source metadata in local records.
- Add preview and confirmation flows before installation.

#### 6. Public distribution complexity
Homebrew helps, but public Windows distribution requires a complementary release path.

**Mitigation**
- Define release automation for GitHub Releases and document Windows install paths early.
