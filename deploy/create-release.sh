#!/bin/bash
#
# RustyWorm GitHub Release Creator
# Creates a GitHub release with pre-built binaries
#
# Usage: ./create-release.sh [version]
# Example: ./create-release.sh 2.1.0
#

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_step() { echo -e "${BLUE}==>${NC} $1"; }
print_success() { echo -e "${GREEN}[OK]${NC} $1"; }
print_warning() { echo -e "${YELLOW}[WARN]${NC} $1"; }
print_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BINARY_NAME="rustyworm"
RELEASE_DIR="$PROJECT_ROOT/release-artifacts"

# Get version from argument or Cargo.toml
if [ -n "$1" ]; then
    VERSION="$1"
else
    VERSION=$(grep '^version' "$PROJECT_ROOT/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')
fi

TAG="v$VERSION"

echo ""
echo "  ╔═══════════════════════════════════════════════════════════╗"
echo "  ║          RustyWorm GitHub Release Creator                  ║"
echo "  ║                  Version: $VERSION                             ║"
echo "  ╚═══════════════════════════════════════════════════════════╝"
echo ""

# Check prerequisites
print_step "Checking prerequisites..."

if ! command -v gh &> /dev/null; then
    print_error "GitHub CLI (gh) is not installed"
    echo "  Install with: curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg"
    exit 1
fi

if ! gh auth status &> /dev/null; then
    print_error "GitHub CLI is not authenticated"
    echo "  Run: gh auth login"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    print_error "Cargo is not installed"
    exit 1
fi

print_success "All prerequisites met"

# Check git status
print_step "Checking git status..."

cd "$PROJECT_ROOT"

if ! git diff --quiet; then
    print_warning "You have uncommitted changes"
    read -p "Continue anyway? [y/N] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check if tag exists
if git rev-parse "$TAG" >/dev/null 2>&1; then
    print_warning "Tag $TAG already exists"
    read -p "Delete and recreate? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        git tag -d "$TAG" 2>/dev/null || true
        git push origin ":refs/tags/$TAG" 2>/dev/null || true
    else
        exit 1
    fi
fi

# Build release binary
print_step "Building release binary..."

cargo build --release --features full

if [ ! -f "$PROJECT_ROOT/target/release/$BINARY_NAME" ]; then
    print_error "Binary not found at target/release/$BINARY_NAME"
    exit 1
fi

BINARY_SIZE=$(du -h "$PROJECT_ROOT/target/release/$BINARY_NAME" | cut -f1)
print_success "Binary built: $BINARY_SIZE"

# Prepare release artifacts
print_step "Preparing release artifacts..."

rm -rf "$RELEASE_DIR"
mkdir -p "$RELEASE_DIR"

# Detect current platform
ARCH=$(uname -m)
OS=$(uname -s | tr '[:upper:]' '[:lower:]')

case "$ARCH" in
    x86_64)  ARCH="x86_64" ;;
    aarch64) ARCH="aarch64" ;;
    arm64)   ARCH="aarch64" ;;
    *)       ARCH="$ARCH" ;;
esac

case "$OS" in
    linux)  TARGET="${ARCH}-unknown-linux-gnu" ;;
    darwin) TARGET="${ARCH}-apple-darwin" ;;
    *)      TARGET="${ARCH}-${OS}" ;;
esac

ARCHIVE_NAME="${BINARY_NAME}-${TAG}-${TARGET}.tar.gz"

# Create tarball
cd "$PROJECT_ROOT/target/release"
tar czf "$RELEASE_DIR/$ARCHIVE_NAME" "$BINARY_NAME"
cd "$PROJECT_ROOT"

# Generate checksum
cd "$RELEASE_DIR"
sha256sum "$ARCHIVE_NAME" > SHA256SUMS.txt
cd "$PROJECT_ROOT"

print_success "Created: $ARCHIVE_NAME"

# Generate release notes
print_step "Generating release notes..."

RELEASE_NOTES="$RELEASE_DIR/RELEASE_NOTES.md"

cat > "$RELEASE_NOTES" << EOF
## RustyWorm $TAG

### Highlights

- Universal AI Mimicry Engine with AgentCPM integration
- Reinforcement learning-based behavior optimization
- Cross-platform GUI agent support
- Production-ready deployment infrastructure

### Installation

#### Binary Download
\`\`\`bash
# Download and extract
curl -LO https://github.com/GitMonsters/Prime-directive/releases/download/${TAG}/${ARCHIVE_NAME}
tar xzf ${ARCHIVE_NAME}
sudo mv ${BINARY_NAME} /usr/local/bin/
\`\`\`

#### Docker
\`\`\`bash
docker pull ghcr.io/gitmonsters/rustyworm:${VERSION}
docker run -it ghcr.io/gitmonsters/rustyworm:${VERSION} --help
\`\`\`

### Checksums

\`\`\`
$(cat "$RELEASE_DIR/SHA256SUMS.txt")
\`\`\`

### What's Changed

EOF

# Add recent commits to release notes
git log --oneline $(git describe --tags --abbrev=0 2>/dev/null || echo "HEAD~20")..HEAD >> "$RELEASE_NOTES" 2>/dev/null || echo "- Initial release" >> "$RELEASE_NOTES"

print_success "Release notes generated"

# Create git tag
print_step "Creating git tag $TAG..."

git tag -a "$TAG" -m "Release $TAG"
git push origin "$TAG"

print_success "Tag $TAG pushed to origin"

# Create GitHub release
print_step "Creating GitHub release..."

gh release create "$TAG" \
    --title "RustyWorm $TAG" \
    --notes-file "$RELEASE_NOTES" \
    "$RELEASE_DIR/$ARCHIVE_NAME" \
    "$RELEASE_DIR/SHA256SUMS.txt"

print_success "GitHub release created"

# Summary
echo ""
echo "  ╔═══════════════════════════════════════════════════════════╗"
echo "  ║                   Release Complete!                        ║"
echo "  ╚═══════════════════════════════════════════════════════════╝"
echo ""
echo "  Version: $TAG"
echo "  Binary:  $ARCHIVE_NAME ($BINARY_SIZE)"
echo ""
echo "  View release: https://github.com/GitMonsters/Prime-directive/releases/tag/$TAG"
echo ""

# Cleanup
rm -rf "$RELEASE_DIR"

print_success "Done!"
