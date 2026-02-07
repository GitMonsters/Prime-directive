#!/bin/bash
# Prime-Directive Local Deployment Startup Script

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║   Prime-Directive - Local Deployment Startup                 ║"
echo "║   Starting API Server and Web Interface                       ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Check if Python is available
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 not found. Please install Python 3.8+"
    exit 1
fi

# Check if dependencies are installed
echo "🔧 Checking dependencies..."
python3 -c "import flask" 2>/dev/null || {
    echo "⚠️  Installing Flask..."
    pip install --break-system-packages -q flask flask-cors
}

# Create PID file directory
mkdir -p /tmp/prime-directive
PID_FILE="/tmp/prime-directive/api_server.pid"

# Function to cleanup on exit
cleanup() {
    echo ""
    echo "⚠️  Shutting down servers..."
    if [ -f "$PID_FILE" ]; then
        PID=$(cat "$PID_FILE")
        kill $PID 2>/dev/null
        rm "$PID_FILE"
    fi
    echo "✅ Servers stopped"
}

trap cleanup EXIT

# Start API Server
echo ""
echo "🚀 Starting API Server (Port 5000)..."
cd /home/worm/Prime-directive
python3 api_server.py > /tmp/prime-directive/api_server.log 2>&1 &
API_PID=$!
echo $API_PID > "$PID_FILE"
echo "✅ API Server started (PID: $API_PID)"

sleep 2

# Display access information
echo ""
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║                   PRIME-DIRECTIVE LIVE                        ║"
echo "╠════════════════════════════════════════════════════════════════╣"
echo "║                                                                ║"
echo "║  🌐 Web Interface:  http://localhost:8080                      ║"
echo "║  🔌 API Server:     http://localhost:5000                      ║"
echo "║                                                                ║"
echo "║  Available Endpoints:                                          ║"
echo "║    GET    /status           - System status                   ║"
echo "║    POST   /query            - Submit physics query             ║"
echo "║    GET    /domains          - List all 24 domains              ║"
echo "║    GET    /domain/{name}    - Get domain information           ║"
echo "║    POST   /simulate         - Run physics simulation           ║"
echo "║    POST   /detect-domain    - Detect domain from query         ║"
echo "║                                                                ║"
echo "║  Example Query:                                                ║"
echo "║    curl -X POST http://localhost:5000/query \\                 ║"
echo "║      -H \"Content-Type: application/json\" \\                   ║"
echo "║      -d '{\"query\":\"What is quantum entanglement?\"}'         ║"
echo "║                                                                ║"
echo "║  Press Ctrl+C to stop servers                                  ║"
echo "║                                                                ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Keep running
wait $API_PID
