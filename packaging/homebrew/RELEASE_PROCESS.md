# Release Process — Homebrew Tap

## Overview

The formula lives in the dedicated tap repo (`GustavoGutierrez/homebrew-skillweaver`). This document describes the canonical release flow and the secret required to publish.

## Required Secret

Publishing to the tap requires an SSH deploy key with write access to the tap repository. Store the private key as a repository secret in the main repo:

- **Secret name:** `HOMEBREW_TAP_SSH_KEY`
- **Required access:** write-enabled deploy key on `GustavoGutierrez/homebrew-skillweaver`
- **Where to set:** `GustavoGutierrez/skillweaver` → Settings → Secrets → Actions

## Release Flow

### 1. Tag and Release (Main Repo)

The release begins in the main repo. Run from the root of `skillweaver`:

```bash
./scripts/release.sh
```

The script:
1. Auto-detects bump type (major/minor/patch) from conventional commits since last tag
2. Runs `cargo test`
3. Builds the release binary (`cargo build --release`)
4. Updates `Cargo.toml` and `src/ui.rs` with the new version
5. Commits and tags
6. Pushes to origin

The tag push triggers GitHub Actions, which builds the tarball, computes the SHA256, creates the GitHub Release, renders the Homebrew formula, and publishes the tap update.

The Linux release binary is built as a static `x86_64-unknown-linux-musl` artifact. This avoids host glibc version mismatches and is the most portable option for mainstream x86_64 Linux systems.

### 2. CI Publishes to Tap

A GitHub Actions workflow in the main repo watches for new releases. On each release:

1. CI clones the tap repo
2. Updates `Formula/skillweaver.rb` with the new version and SHA256
3. Commits and pushes using `HOMEBREW_TAP_SSH_KEY`

### Manual Publish (if CI is unavailable)

If CI fails, publish manually:

```bash
# Clone the tap repo
git clone https://github.com/GustavoGutierrez/homebrew-skillweaver.git
cd homebrew-skillweaver

# Update the formula with the new version and SHA256
# Edit Formula/skillweaver.rb with:
#   - version "X.Y.Z"
#   - url "https://github.com/GustavoGutierrez/skillweaver/releases/download/vX.Y.Z/skillweaver-x86_64-unknown-linux-gnu.tar.gz"
#   - sha256 "<computed-sha256>"

git add Formula/skillweaver.rb
git commit -m "skillweaver vX.Y.Z"
git push origin main
```

To compute the SHA256 from the release tarball:

```bash
sha256sum skillweaver-x86_64-unknown-linux-gnu.tar.gz
```

## Version Alignment

The formula version must match the git tag in the main repo exactly (e.g., `v0.0.1` in tag = `0.0.1` in formula).

## Rollback

If a bad release is published to the tap:

```bash
cd homebrew-skillweaver
git log --oneline
# Find the last good commit
git revert <bad-commit-hash>
git push origin main
```

Notify users to run `brew upgrade skillweaver` again to get the corrected formula.
