#!/bin/bash
# RustyWorm Remote Deployment Script
# Deploy to a remote server via SSH
#
# Usage: ./deploy-remote.sh <user@host> [options]
#
# Examples:
#   ./deploy-remote.sh root@myserver.com
#   ./deploy-remote.sh deploy@192.168.1.100 --docker
#   ./deploy-remote.sh ubuntu@aws-instance --baremetal

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Default values
DEPLOY_METHOD="docker"
REMOTE_DIR="/opt/rustyworm"
SSH_KEY=""

print_banner() {
    echo -e "${BLUE}"
    echo "╔═══════════════════════════════════════════════════════════════╗"
    echo "║           RustyWorm Remote Deployment                         ║"
    echo "╚═══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

print_help() {
    print_banner
    echo "Usage: $0 <user@host> [options]"
    echo ""
    echo "Arguments:"
    echo "  user@host       SSH connection string (e.g., root@myserver.com)"
    echo ""
    echo "Options:"
    echo "  --docker        Deploy using Docker Compose (default)"
    echo "  --baremetal     Deploy as systemd services"
    echo "  --binary-only   Just copy the binary, no service setup"
    echo "  --key <path>    SSH private key path"
    echo "  --dir <path>    Remote installation directory (default: /opt/rustyworm)"
    echo "  -h, --help      Show this help"
    echo ""
    echo "Examples:"
    echo "  $0 root@myserver.com"
    echo "  $0 ubuntu@aws-instance --docker --key ~/.ssh/aws.pem"
    echo "  $0 deploy@192.168.1.100 --baremetal"
    echo ""
    echo "Prerequisites on remote server:"
    echo "  Docker method:    Docker, Docker Compose"
    echo "  Baremetal method: Python 3.11+, MongoDB, systemd"
    echo ""
}

check_binary() {
    BINARY="$PROJECT_ROOT/target/release/rustyworm"
    if [ ! -f "$BINARY" ]; then
        echo -e "${YELLOW}Building release binary...${NC}"
        cd "$PROJECT_ROOT"
        cargo build --features full --release
    fi
    echo -e "${GREEN}Binary ready: $BINARY${NC}"
}

ssh_cmd() {
    if [ -n "$SSH_KEY" ]; then
        ssh -i "$SSH_KEY" "$SSH_HOST" "$@"
    else
        ssh "$SSH_HOST" "$@"
    fi
}

scp_cmd() {
    if [ -n "$SSH_KEY" ]; then
        scp -i "$SSH_KEY" "$@"
    else
        scp "$@"
    fi
}

deploy_docker() {
    echo -e "${BLUE}Deploying with Docker Compose...${NC}"
    echo ""
    
    # Check Docker on remote
    echo "Checking Docker on remote..."
    if ! ssh_cmd "docker --version" &>/dev/null; then
        echo -e "${RED}Error: Docker not installed on remote server${NC}"
        echo "Install Docker first: https://docs.docker.com/engine/install/"
        exit 1
    fi
    
    # Create remote directory
    echo "Creating remote directory..."
    ssh_cmd "mkdir -p $REMOTE_DIR"
    
    # Copy deployment files
    echo "Copying deployment files..."
    scp_cmd -r "$SCRIPT_DIR/"* "$SSH_HOST:$REMOTE_DIR/"
    scp_cmd "$PROJECT_ROOT/Cargo.toml" "$SSH_HOST:$REMOTE_DIR/../"
    scp_cmd "$PROJECT_ROOT/Cargo.lock" "$SSH_HOST:$REMOTE_DIR/../"
    scp_cmd -r "$PROJECT_ROOT/src" "$SSH_HOST:$REMOTE_DIR/../"
    scp_cmd -r "$PROJECT_ROOT/agentcpm-integration" "$SSH_HOST:$REMOTE_DIR/../"
    
    # Copy .env if exists
    if [ -f "$SCRIPT_DIR/.env" ]; then
        scp_cmd "$SCRIPT_DIR/.env" "$SSH_HOST:$REMOTE_DIR/"
    else
        scp_cmd "$SCRIPT_DIR/.env.example" "$SSH_HOST:$REMOTE_DIR/.env"
    fi
    
    # Start services on remote
    echo "Starting services on remote..."
    ssh_cmd "cd $REMOTE_DIR && chmod +x deploy.sh && ./deploy.sh start"
    
    echo ""
    echo -e "${GREEN}Deployment complete!${NC}"
    echo ""
    echo "Connect to RustyWorm:"
    echo "  ssh $SSH_HOST 'cd $REMOTE_DIR && ./deploy.sh shell'"
    echo ""
    echo "Check status:"
    echo "  ssh $SSH_HOST 'cd $REMOTE_DIR && ./deploy.sh status'"
}

deploy_baremetal() {
    echo -e "${BLUE}Deploying as bare metal (systemd)...${NC}"
    echo ""
    
    check_binary
    
    # Create remote directory
    echo "Creating remote directory..."
    ssh_cmd "sudo mkdir -p $REMOTE_DIR/bin $REMOTE_DIR/deploy"
    
    # Copy binary
    echo "Copying binary (this may take a moment)..."
    scp_cmd "$PROJECT_ROOT/target/release/rustyworm" "$SSH_HOST:/tmp/rustyworm"
    ssh_cmd "sudo mv /tmp/rustyworm $REMOTE_DIR/bin/ && sudo chmod +x $REMOTE_DIR/bin/rustyworm"
    
    # Copy deployment files
    echo "Copying deployment files..."
    scp_cmd -r "$SCRIPT_DIR/"* "$SSH_HOST:/tmp/deploy/"
    ssh_cmd "sudo mv /tmp/deploy/* $REMOTE_DIR/deploy/"
    
    # Copy agentcpm-integration
    echo "Copying AgentRL service..."
    scp_cmd -r "$PROJECT_ROOT/agentcpm-integration" "$SSH_HOST:/tmp/"
    ssh_cmd "sudo mv /tmp/agentcpm-integration $REMOTE_DIR/"
    
    # Run installer
    echo "Running bare metal installer..."
    ssh_cmd "sudo chmod +x $REMOTE_DIR/deploy/install-baremetal.sh && sudo $REMOTE_DIR/deploy/install-baremetal.sh"
    
    echo ""
    echo -e "${GREEN}Deployment complete!${NC}"
    echo ""
    echo "Start services:"
    echo "  ssh $SSH_HOST 'sudo systemctl start mongodb agentrl rustyworm'"
    echo ""
    echo "Check status:"
    echo "  ssh $SSH_HOST 'sudo systemctl status rustyworm'"
    echo ""
    echo "Run interactively:"
    echo "  ssh $SSH_HOST 'rustyworm'"
}

deploy_binary_only() {
    echo -e "${BLUE}Deploying binary only...${NC}"
    echo ""
    
    check_binary
    
    # Create remote directory
    echo "Creating remote directory..."
    ssh_cmd "mkdir -p $REMOTE_DIR/bin"
    
    # Copy binary
    echo "Copying binary..."
    scp_cmd "$PROJECT_ROOT/target/release/rustyworm" "$SSH_HOST:$REMOTE_DIR/bin/"
    ssh_cmd "chmod +x $REMOTE_DIR/bin/rustyworm"
    
    # Add to PATH
    ssh_cmd "echo 'export PATH=\$PATH:$REMOTE_DIR/bin' >> ~/.bashrc"
    
    echo ""
    echo -e "${GREEN}Binary deployed!${NC}"
    echo ""
    echo "Run RustyWorm:"
    echo "  ssh $SSH_HOST '$REMOTE_DIR/bin/rustyworm'"
}

# Parse arguments
if [ $# -eq 0 ]; then
    print_help
    exit 0
fi

SSH_HOST="$1"
shift

while [[ $# -gt 0 ]]; do
    case $1 in
        --docker)
            DEPLOY_METHOD="docker"
            shift
            ;;
        --baremetal)
            DEPLOY_METHOD="baremetal"
            shift
            ;;
        --binary-only)
            DEPLOY_METHOD="binary"
            shift
            ;;
        --key)
            SSH_KEY="$2"
            shift 2
            ;;
        --dir)
            REMOTE_DIR="$2"
            shift 2
            ;;
        -h|--help)
            print_help
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            print_help
            exit 1
            ;;
    esac
done

# Validate SSH host
if [[ ! "$SSH_HOST" =~ @ ]]; then
    echo -e "${RED}Error: Invalid SSH host format. Use user@host${NC}"
    exit 1
fi

print_banner
echo -e "Target: ${GREEN}$SSH_HOST${NC}"
echo -e "Method: ${GREEN}$DEPLOY_METHOD${NC}"
echo -e "Remote dir: ${GREEN}$REMOTE_DIR${NC}"
echo ""

# Test SSH connection
echo "Testing SSH connection..."
if ! ssh_cmd "echo 'SSH OK'" &>/dev/null; then
    echo -e "${RED}Error: Cannot connect to $SSH_HOST${NC}"
    echo "Check your SSH configuration and try again."
    exit 1
fi
echo -e "${GREEN}SSH connection successful${NC}"
echo ""

# Deploy based on method
case $DEPLOY_METHOD in
    docker)
        deploy_docker
        ;;
    baremetal)
        deploy_baremetal
        ;;
    binary)
        deploy_binary_only
        ;;
esac
