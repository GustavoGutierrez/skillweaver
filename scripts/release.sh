#!/usr/bin/env bash
set -euo pipefail

# SkillWeaver Release Script
# Usage: ./scripts/release.sh [--patch|--minor|--major|<version>]
# Default: --patch

RED='\033[0;31m'
GREEN='\033[0;32m'
CYAN='\033[0;36m'
NC='\033[0m'

log()  { echo -e "${CYAN}[release]${NC} $1"; }
ok()   { echo -e "${GREEN}[✓]${NC} $1"; }
err()  { echo -e "${RED}[✗]${NC} $1"; exit 1; }

# ── Parse arguments ──────────────────────────────────────────────
BUMP="${1:---patch}"

if [[ "$BUMP" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    NEW_VERSION="$BUMP"
else
    case "$BUMP" in
        --major)    COMPONENT="major" ;;
        --minor)    COMPONENT="minor" ;;
        --patch)    COMPONENT="patch" ;;
        *)          err "Invalid argument: $BUMP. Use --patch, --minor, --major, or x.y.z" ;;
    esac
fi

# ── Determine version ────────────────────────────────────────────
CURRENT=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
log "Current version: $CURRENT"

if [ -z "${NEW_VERSION:-}" ]; then
    IFS='.' read -r MAJ MIN PAT <<< "$CURRENT"
    case "$COMPONENT" in
        major) NEW_VERSION="$((MAJ+1)).0.0" ;;
        minor) NEW_VERSION="$MAJ.$((MIN+1)).0" ;;
        patch) NEW_VERSION="$MAJ.$MIN.$((PAT+1))" ;;
    esac
fi
log "New version: $NEW_VERSION"

# ── Check prerequisites ──────────────────────────────────────────
command -v cargo  >/dev/null 2>&1 || err "cargo not found"
command -v gh     >/dev/null 2>&1 || err "gh CLI not found. Install: https://cli.github.com"
gh auth status    >/dev/null 2>&1 || err "gh not authenticated. Run: gh auth login"

# ── Ensure clean working tree ────────────────────────────────────
if ! git diff --quiet || ! git diff --cached --quiet; then
    err "Working tree is dirty. Commit or stash changes first."
fi

git pull origin main --rebase 2>/dev/null || true

# ── Run tests ────────────────────────────────────────────────────
log "Running tests..."
cargo test 2>&1 | tail -5
ok "Tests passed"

# ── Build release ────────────────────────────────────────────────
log "Building release..."
cargo build --release
ok "Release build complete"

# ── Package tarball ──────────────────────────────────────────────
TARBALL="skillweaver-x86_64-unknown-linux-gnu.tar.gz"
log "Packaging $TARBALL..."
rm -f "$TARBALL"
cp target/release/skillweaver skillweaver
tar czf "$TARBALL" skillweaver
rm -f skillweaver
SHA256=$(sha256sum "$TARBALL" | awk '{print $1}')
ok "SHA256: $SHA256"

# ── Bump version in files ────────────────────────────────────────
log "Bumping version to $NEW_VERSION..."
sed -i "s/version = \"$CURRENT\"/version = \"$NEW_VERSION\"/" Cargo.toml
sed -i "s/v$CURRENT/v$NEW_VERSION/g" src/ui.rs
ok "Version bumped in Cargo.toml and src/ui.rs"

# ── Update Homebrew formula ──────────────────────────────────────
FORMULA="Formula/skillweaver.rb"
log "Updating formula..."
sed -i "s/v$CURRENT/v$NEW_VERSION/g" "$FORMULA"
sed -i "s/sha256 \".*\"/sha256 \"$SHA256\"/" "$FORMULA"
ok "Formula updated"

# ── Commit, tag, push ────────────────────────────────────────────
log "Committing..."
git add Cargo.toml src/ui.rs "$FORMULA"
git commit -m "chore: bump version to v$NEW_VERSION"

log "Tagging v$NEW_VERSION..."
git tag -a "v$NEW_VERSION" -m "v$NEW_VERSION"

log "Pushing..."
git push origin main
git push origin "v$NEW_VERSION"
ok "Pushed main + tag v$NEW_VERSION"

# ── Create GitHub Release ────────────────────────────────────────
log "Creating GitHub Release..."
RELEASE_NOTES=$(cat <<EOF
## v$NEW_VERSION

### Binary
- \`$TARBALL\` — Linux x86_64

### SHA256
\`\`\`
$SHA256
\`\`\`

### Install
\`\`\`bash
brew upgrade skillweaver
\`\`\`
EOF
)

gh release create "v$NEW_VERSION" \
    "$TARBALL" \
    --title "v$NEW_VERSION" \
    --notes "$RELEASE_NOTES"

ok "Release v$NEW_VERSION published!"

# ── Cleanup ──────────────────────────────────────────────────────
rm -f "$TARBALL"

echo ""
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}  Release v$NEW_VERSION complete!${NC}"
echo -e "${GREEN}  Homebrew users: brew upgrade skillweaver${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
