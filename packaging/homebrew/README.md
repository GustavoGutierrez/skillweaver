# Homebrew Tap — SkillWeaver

## Overview

This tap contains the Homebrew formula for SkillWeaver's Linux binary releases.

- **Canonical repo:** `GustavoGutierrez/homebrew-skillweaver`
- **Formula:** `Formula/skillweaver.rb`
- **Platform:** Linux only (x86_64)
- **Binary source:** GitHub Releases (`skillweaver-x86_64-unknown-linux-gnu.tar.gz`)

## Using the Tap

```bash
# First time install
brew tap GustavoGutierrez/homebrew-skillweaver
brew install skillweaver

# Upgrade
brew upgrade skillweaver

# Uninstall
brew uninstall skillweaver
brew untap GustavoGutierrez/homebrew-skillweaver
```

## What This Tap Contains

```
homebrew-skillweaver/
├── README.md           ← you are here
├── RELEASE_PROCESS.md  ← how releases are published
└── Formula/
    └── skillweaver.rb  ← the Homebrew formula
```

The formula downloads a pre-built tarball from the main repo's GitHub Releases and installs only the `skillweaver` binary — no source builds.

## Files and Responsibilities

| File | Owner | Purpose |
|------|-------|---------|
| `Formula/skillweaver.rb` | `release` agent (CI) | Formula definition; auto-updated on release |
| `README.md` | Documentation (this repo) | User-facing tap docs |
| `RELEASE_PROCESS.md` | Documentation (this repo) | Release workflow and secret management |

The main repo (`GustavoGutierrez/skillweaver`) owns the release scripts and template documentation under `packaging/homebrew/`. The tap repo owns the published formula.