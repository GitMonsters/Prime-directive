#!/bin/bash
# =============================================================================
# RustyWorm API Testing Script
# =============================================================================
# This script tests the live API observation capabilities.
#
# Prerequisites:
#   - Build with: cargo build --release --features api
#   - Set at least one API key:
#       export OPENAI_API_KEY="sk-..."
#       export ANTHROPIC_API_KEY="sk-ant-..."
#       export GOOGLE_API_KEY="..."
#
# Usage:
#   ./scripts/test_api.sh [provider]
#
# Examples:
#   ./scripts/test_api.sh            # Test all configured providers
#   ./scripts/test_api.sh openai     # Test only OpenAI
#   ./scripts/test_api.sh anthropic  # Test only Anthropic
# =============================================================================

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_DIR"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== RustyWorm API Testing ===${NC}"
echo

# Check if built with API feature
if [[ ! -f "target/release/rustyworm" ]]; then
    echo -e "${YELLOW}Building with API feature...${NC}"
    cargo build --release --features api
fi

# Check API keys
echo -e "${BLUE}Checking API keys...${NC}"
CONFIGURED=0

if [[ -n "$OPENAI_API_KEY" ]]; then
    echo -e "  ${GREEN}✓${NC} OPENAI_API_KEY is set"
    CONFIGURED=$((CONFIGURED + 1))
else
    echo -e "  ${RED}✗${NC} OPENAI_API_KEY not set"
fi

if [[ -n "$ANTHROPIC_API_KEY" ]]; then
    echo -e "  ${GREEN}✓${NC} ANTHROPIC_API_KEY is set"
    CONFIGURED=$((CONFIGURED + 1))
else
    echo -e "  ${RED}✗${NC} ANTHROPIC_API_KEY not set"
fi

if [[ -n "$GOOGLE_API_KEY" ]]; then
    echo -e "  ${GREEN}✓${NC} GOOGLE_API_KEY is set"
    CONFIGURED=$((CONFIGURED + 1))
else
    echo -e "  ${RED}✗${NC} GOOGLE_API_KEY not set"
fi

echo

if [[ $CONFIGURED -eq 0 ]]; then
    echo -e "${RED}No API keys configured!${NC}"
    echo "Please set at least one API key:"
    echo "  export OPENAI_API_KEY=\"sk-...\""
    echo "  export ANTHROPIC_API_KEY=\"sk-ant-...\""
    echo "  export GOOGLE_API_KEY=\"...\""
    exit 1
fi

# Create test commands file
PROVIDER="${1:-all}"

create_test_commands() {
    local provider="$1"
    cat << EOF
/api-config $provider
/api-status
/api-observe $provider "What is the capital of France? Please answer briefly."
/api-observe $provider "Write a haiku about programming."
/api-observe $provider "Explain recursion in one sentence."
/status
/list
/quit
EOF
}

run_provider_test() {
    local provider="$1"
    echo -e "${BLUE}Testing $provider API...${NC}"
    
    # Create temp commands file
    CMDS=$(mktemp)
    create_test_commands "$provider" > "$CMDS"
    
    # Run RustyWorm with commands
    if timeout 120 ./target/release/rustyworm < "$CMDS" 2>&1; then
        echo -e "${GREEN}✓ $provider test completed${NC}"
    else
        echo -e "${RED}✗ $provider test failed${NC}"
    fi
    
    rm -f "$CMDS"
    echo
}

# Run tests based on provider selection
case "$PROVIDER" in
    openai|gpt)
        if [[ -n "$OPENAI_API_KEY" ]]; then
            run_provider_test "openai"
        else
            echo -e "${RED}OPENAI_API_KEY not set${NC}"
            exit 1
        fi
        ;;
    anthropic|claude)
        if [[ -n "$ANTHROPIC_API_KEY" ]]; then
            run_provider_test "anthropic"
        else
            echo -e "${RED}ANTHROPIC_API_KEY not set${NC}"
            exit 1
        fi
        ;;
    google|gemini)
        if [[ -n "$GOOGLE_API_KEY" ]]; then
            run_provider_test "google"
        else
            echo -e "${RED}GOOGLE_API_KEY not set${NC}"
            exit 1
        fi
        ;;
    all)
        [[ -n "$OPENAI_API_KEY" ]] && run_provider_test "openai"
        [[ -n "$ANTHROPIC_API_KEY" ]] && run_provider_test "anthropic"
        [[ -n "$GOOGLE_API_KEY" ]] && run_provider_test "google"
        ;;
    *)
        echo "Unknown provider: $PROVIDER"
        echo "Usage: $0 [openai|anthropic|google|all]"
        exit 1
        ;;
esac

echo -e "${GREEN}=== API Testing Complete ===${NC}"
