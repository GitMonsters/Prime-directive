#!/bin/bash
# RustyWorm Deployment Script
# Usage: ./deploy.sh [command] [options]

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
COMPOSE_FILE="$SCRIPT_DIR/docker-compose.yml"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
PROFILE=""
DETACHED="-d"

print_banner() {
    echo -e "${BLUE}"
    echo "╔═══════════════════════════════════════════════════════════════╗"
    echo "║              RustyWorm v2.1.0 Deployment                      ║"
    echo "║         Universal AI Mimicry Engine                           ║"
    echo "╚═══════════════════════════════════════════════════════════════╝"
    echo -e "${NC}"
}

print_help() {
    print_banner
    echo "Usage: $0 <command> [options]"
    echo ""
    echo "Commands:"
    echo "  start       Start all services"
    echo "  stop        Stop all services"
    echo "  restart     Restart all services"
    echo "  status      Show service status"
    echo "  logs        Show service logs"
    echo "  shell       Open RustyWorm REPL"
    echo "  build       Build Docker images"
    echo "  clean       Stop and remove containers, networks, volumes"
    echo "  health      Check service health"
    echo ""
    echo "Options:"
    echo "  --monitoring    Include Prometheus/Grafana monitoring stack"
    echo "  --gpu           Include GPU services (AgentCPM-GUI)"
    echo "  --foreground    Run in foreground (not detached)"
    echo "  -h, --help      Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 start                    # Start core services"
    echo "  $0 start --monitoring       # Start with monitoring"
    echo "  $0 logs agentrl             # Show AgentRL logs"
    echo "  $0 shell                    # Open RustyWorm REPL"
    echo ""
}

check_docker() {
    if ! command -v docker &> /dev/null; then
        echo -e "${RED}Error: Docker is not installed${NC}"
        exit 1
    fi
    
    if ! docker info &> /dev/null; then
        echo -e "${RED}Error: Docker daemon is not running${NC}"
        exit 1
    fi
}

check_compose() {
    if docker compose version &> /dev/null; then
        COMPOSE_CMD="docker compose"
    elif command -v docker-compose &> /dev/null; then
        COMPOSE_CMD="docker-compose"
    else
        echo -e "${RED}Error: Docker Compose is not installed${NC}"
        exit 1
    fi
}

start_services() {
    print_banner
    echo -e "${GREEN}Starting RustyWorm services...${NC}"
    
    cd "$SCRIPT_DIR"
    
    # Build if needed
    $COMPOSE_CMD -f "$COMPOSE_FILE" build
    
    # Start services
    if [ -n "$PROFILE" ]; then
        $COMPOSE_CMD -f "$COMPOSE_FILE" --profile "$PROFILE" up $DETACHED
    else
        $COMPOSE_CMD -f "$COMPOSE_FILE" up $DETACHED
    fi
    
    if [ "$DETACHED" == "-d" ]; then
        echo ""
        echo -e "${GREEN}Services started successfully!${NC}"
        echo ""
        show_status
        echo ""
        echo -e "${YELLOW}Quick commands:${NC}"
        echo "  View logs:     $0 logs"
        echo "  Open REPL:     $0 shell"
        echo "  Check health:  $0 health"
        echo "  Stop:          $0 stop"
    fi
}

stop_services() {
    echo -e "${YELLOW}Stopping RustyWorm services...${NC}"
    cd "$SCRIPT_DIR"
    
    if [ -n "$PROFILE" ]; then
        $COMPOSE_CMD -f "$COMPOSE_FILE" --profile "$PROFILE" down
    else
        $COMPOSE_CMD -f "$COMPOSE_FILE" down
    fi
    
    echo -e "${GREEN}Services stopped.${NC}"
}

restart_services() {
    stop_services
    start_services
}

show_status() {
    echo -e "${BLUE}Service Status:${NC}"
    echo ""
    cd "$SCRIPT_DIR"
    $COMPOSE_CMD -f "$COMPOSE_FILE" ps
}

show_logs() {
    cd "$SCRIPT_DIR"
    if [ -n "$1" ]; then
        $COMPOSE_CMD -f "$COMPOSE_FILE" logs -f "$1"
    else
        $COMPOSE_CMD -f "$COMPOSE_FILE" logs -f
    fi
}

open_shell() {
    echo -e "${GREEN}Opening RustyWorm REPL...${NC}"
    echo ""
    cd "$SCRIPT_DIR"
    docker exec -it rustyworm /app/rustyworm
}

build_images() {
    echo -e "${BLUE}Building Docker images...${NC}"
    cd "$SCRIPT_DIR"
    $COMPOSE_CMD -f "$COMPOSE_FILE" build --no-cache
    echo -e "${GREEN}Build complete!${NC}"
}

clean_all() {
    echo -e "${RED}WARNING: This will remove all containers, networks, and volumes!${NC}"
    read -p "Are you sure? (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        cd "$SCRIPT_DIR"
        $COMPOSE_CMD -f "$COMPOSE_FILE" down -v --remove-orphans
        docker system prune -f
        echo -e "${GREEN}Cleanup complete.${NC}"
    else
        echo "Aborted."
    fi
}

check_health() {
    echo -e "${BLUE}Checking service health...${NC}"
    echo ""
    
    # Check AgentRL
    echo -n "AgentRL: "
    if curl -sf http://localhost:8080/health > /dev/null 2>&1; then
        echo -e "${GREEN}healthy${NC}"
    else
        echo -e "${RED}unhealthy or not running${NC}"
    fi
    
    # Check MongoDB
    echo -n "MongoDB: "
    if docker exec rustyworm-mongodb mongosh --eval "db.adminCommand('ping')" > /dev/null 2>&1; then
        echo -e "${GREEN}healthy${NC}"
    else
        echo -e "${RED}unhealthy or not running${NC}"
    fi
    
    # Check RustyWorm
    echo -n "RustyWorm: "
    if docker exec rustyworm /app/rustyworm --version > /dev/null 2>&1; then
        echo -e "${GREEN}healthy${NC}"
    else
        echo -e "${RED}unhealthy or not running${NC}"
    fi
    
    echo ""
}

# Parse options
parse_options() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --monitoring)
                PROFILE="monitoring"
                shift
                ;;
            --gpu)
                PROFILE="gpu"
                shift
                ;;
            --foreground)
                DETACHED=""
                shift
                ;;
            -h|--help)
                print_help
                exit 0
                ;;
            *)
                shift
                ;;
        esac
    done
}

# Main
check_docker
check_compose

COMMAND="${1:-}"
shift 2>/dev/null || true

parse_options "$@"

case "$COMMAND" in
    start)
        start_services
        ;;
    stop)
        stop_services
        ;;
    restart)
        restart_services
        ;;
    status)
        show_status
        ;;
    logs)
        show_logs "$1"
        ;;
    shell)
        open_shell
        ;;
    build)
        build_images
        ;;
    clean)
        clean_all
        ;;
    health)
        check_health
        ;;
    -h|--help|"")
        print_help
        ;;
    *)
        echo -e "${RED}Unknown command: $COMMAND${NC}"
        print_help
        exit 1
        ;;
esac
