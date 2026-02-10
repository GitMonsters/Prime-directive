#!/bin/bash
# RustyWorm Bare-Metal Installation Script
# Run as root or with sudo

set -e

INSTALL_DIR="/opt/rustyworm"
DATA_DIR="/var/lib/rustyworm"
LOG_DIR="/var/log/rustyworm"
CONFIG_DIR="/etc/rustyworm"
USER="rustyworm"
GROUP="rustyworm"

echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║        RustyWorm v2.1.0 Bare-Metal Installation               ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""

# Check root
if [ "$EUID" -ne 0 ]; then
    echo "Error: Please run as root or with sudo"
    exit 1
fi

# Check for pre-built binary
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BINARY="$PROJECT_ROOT/target/release/rustyworm"

if [ ! -f "$BINARY" ]; then
    echo "Error: Release binary not found at $BINARY"
    echo "Please run: cargo build --features full --release"
    exit 1
fi

echo "Step 1: Creating user and group..."
if ! id "$USER" &>/dev/null; then
    useradd -r -s /bin/false -d "$INSTALL_DIR" "$USER"
    echo "  Created user: $USER"
else
    echo "  User $USER already exists"
fi

echo ""
echo "Step 2: Creating directories..."
mkdir -p "$INSTALL_DIR/bin"
mkdir -p "$DATA_DIR"
mkdir -p "$LOG_DIR"
mkdir -p "$CONFIG_DIR"
echo "  Created: $INSTALL_DIR, $DATA_DIR, $LOG_DIR, $CONFIG_DIR"

echo ""
echo "Step 3: Installing binary..."
cp "$BINARY" "$INSTALL_DIR/bin/rustyworm"
chmod 755 "$INSTALL_DIR/bin/rustyworm"
echo "  Installed: $INSTALL_DIR/bin/rustyworm"

echo ""
echo "Step 4: Creating default configuration..."
cat > "$CONFIG_DIR/rustyworm.env" << 'EOF'
# RustyWorm Configuration
RUSTYWORM_DATA_DIR=/var/lib/rustyworm
RUSTYWORM_LOG_LEVEL=info
AGENTRL_ENDPOINT=http://localhost:8080
RUST_BACKTRACE=1

# Optional: API Keys
# OPENAI_API_KEY=sk-...
# ANTHROPIC_API_KEY=sk-ant-...
# GOOGLE_API_KEY=...
EOF
chmod 600 "$CONFIG_DIR/rustyworm.env"
echo "  Created: $CONFIG_DIR/rustyworm.env"

echo ""
echo "Step 5: Installing systemd service..."
cp "$SCRIPT_DIR/systemd/rustyworm.service" /etc/systemd/system/
cp "$SCRIPT_DIR/systemd/agentrl.service" /etc/systemd/system/
systemctl daemon-reload
echo "  Installed systemd services"

echo ""
echo "Step 6: Setting permissions..."
chown -R "$USER:$GROUP" "$INSTALL_DIR"
chown -R "$USER:$GROUP" "$DATA_DIR"
chown -R "$USER:$GROUP" "$LOG_DIR"
chown -R root:"$GROUP" "$CONFIG_DIR"
chmod 750 "$CONFIG_DIR"
echo "  Set ownership and permissions"

echo ""
echo "Step 7: Creating symlink..."
ln -sf "$INSTALL_DIR/bin/rustyworm" /usr/local/bin/rustyworm
echo "  Created: /usr/local/bin/rustyworm"

echo ""
echo "╔═══════════════════════════════════════════════════════════════╗"
echo "║                    Installation Complete!                      ║"
echo "╚═══════════════════════════════════════════════════════════════╝"
echo ""
echo "Next steps:"
echo ""
echo "  1. Configure API keys (optional):"
echo "     sudo nano /etc/rustyworm/rustyworm.env"
echo ""
echo "  2. Install AgentRL dependencies (if using RL features):"
echo "     cd $PROJECT_ROOT/agentcpm-integration"
echo "     pip install -r requirements.txt"
echo ""
echo "  3. Start MongoDB (required for AgentRL):"
echo "     sudo systemctl start mongod"
echo ""
echo "  4. Enable and start services:"
echo "     sudo systemctl enable rustyworm agentrl"
echo "     sudo systemctl start agentrl"
echo "     sudo systemctl start rustyworm"
echo ""
echo "  5. Check status:"
echo "     sudo systemctl status rustyworm"
echo "     sudo systemctl status agentrl"
echo ""
echo "  6. Run interactively:"
echo "     rustyworm"
echo ""
echo "Logs are at: $LOG_DIR/"
echo "Data is at:  $DATA_DIR/"
echo ""
