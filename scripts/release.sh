#!/usr/bin/env bash
set -euo pipefail

# SkillWeaver Auto-Release Script
# Auto-detects semantic version from commits since last tag.
# Usage: ./scripts/release.sh

RED='\033[0;31m'; GREEN='\033[0;32m'; CYAN='\033[0;36m'; NC='\033[0m'
log()  { echo -e "${CYAN}[release]${NC} $1"; }
ok()   { echo -e "${GREEN}[✓]${NC} $1"; }
err()  { echo -e "${RED}[✗]${NC} $1"; exit 1; }

# ── Prerequisites ─────────────────────────────────────────────────
command -v cargo >/dev/null 2>&1 || err "cargo not found"
command -v gh    >/dev/null 2>&1 || err "gh CLI not found: https://cli.github.com"
gh auth status   >/dev/null 2>&1 || err "gh not authenticated. Run: gh auth login"
git diff --quiet && git diff --cached --quiet || err "Working tree is dirty. Commit or stash first."
git pull origin main --rebase 2>/dev/null || true

# ── Auto-detect bump type from commits ────────────────────────────
LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.0")
log "Last tag: $LAST_TAG"
COMMITS=$(git log "${LAST_TAG}..HEAD" --oneline 2>/dev/null || git log --oneline)

HAS_BREAKING=$(echo "$COMMITS" | grep -ciE 'BREAKING CHANGE|!:|^[a-z]+(\(.+\))?!:') || true
HAS_FEAT=$(echo "$COMMITS" | grep -ciE '^[a-f0-9]+ feat[(:]') || true
HAS_FIX=$(echo "$COMMITS" | grep -ciE '^[a-f0-9]+ fix[(:]') || true

if [ "$HAS_BREAKING" -gt 0 ]; then
    BUMP="major"
    REASON="$HAS_BREAKING breaking change(s)"
elif [ "$HAS_FEAT" -gt 0 ]; then
    BUMP="minor"
    REASON="$HAS_FEAT feature(s)"
else
    BUMP="patch"
    REASON="$HAS_FIX fix(es) / chore"
fi
log "Auto-detected bump: $BUMP ($REASON)"

# ── Compute new version ───────────────────────────────────────────
CURRENT=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
IFS='.' read -r MAJ MIN PAT <<< "$CURRENT"
case "$BUMP" in
    major) NEW="$((MAJ+1)).0.0" ;;
    minor) NEW="$MAJ.$((MIN+1)).0" ;;
    patch) NEW="$MAJ.$MIN.$((PAT+1))" ;;
esac
log "Bump: $CURRENT → $NEW"

# ── Confirm ───────────────────────────────────────────────────────
echo -e "${CYAN}Release v$NEW ($BUMP) — proceed? [Y/n]${NC}"
read -r CONFIRM
[[ "$CONFIRM" =~ ^[Nn] ]] && err "Aborted."

# ── Build & test ─────────────────────────────────────────────────
log "Running tests..."
cargo test 2>&1 | tail -3
log "Building release..."
cargo build --release
ok "Build complete"

# ── Package ──────────────────────────────────────────────────────
TARBALL="skillweaver-x86_64-unknown-linux-gnu.tar.gz"
rm -f "$TARBALL" skillweaver
cp target/release/skillweaver skillweaver
tar czf "$TARBALL" skillweaver
rm -f skillweaver
SHA256=$(sha256sum "$TARBALL" | awk '{print $1}')
ok "Packaged: $TARBALL ($SHA256)"

# ── Bump version in source ───────────────────────────────────────
sed -i "s/version = \"$CURRENT\"/version = \"$NEW\"/" Cargo.toml
sed -i "s/v$CURRENT/v$NEW/g" src/ui.rs

# ── Update Homebrew formula ──────────────────────────────────────
FORMULA="Formula/skillweaver.rb"
sed -i "s/v$CURRENT/v$NEW/g" "$FORMULA"
sed -i "s/sha256 \".*\"/sha256 \"$SHA256\"/" "$FORMULA"
ok "Formula updated"

# ── Commit, tag, push ────────────────────────────────────────────
git add Cargo.toml src/ui.rs "$FORMULA"
git commit -m "chore: bump version to v$NEW"
git tag -a "v$NEW" -m "v$NEW"
git push origin main
git push origin "v$NEW"
ok "Pushed main + tag v$NEW"

# ── GitHub Release ───────────────────────────────────────────────
gh release create "v$NEW" "$TARBALL" \
    --title "v$NEW" \
    --notes "$(cat <<NOTES
## v$NEW

**Type:** $BUMP ($REASON)

### Changes since $LAST_TAG
\`\`\`
$COMMITS
\`\`\`

### Install
\`\`\`bash
brew upgrade skillweaver
\`\`\`

### Checksum
\`\`\`
$SHA256  $TARBALL
\`\`\`
NOTES
)"

# ── Cleanup ──────────────────────────────────────────────────────
rm -f "$TARBALL"

echo ""
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}  v$NEW released!${NC}"
echo -e "${GREEN}  brew upgrade skillweaver${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
