# RustyWorm Deployment Guide

This directory contains everything needed to deploy RustyWorm in production environments.

## Deployment Options

| Method | Use Case | Requirements |
|--------|----------|--------------|
| **Docker Compose** | Recommended for most deployments | Docker, Docker Compose |
| **Bare Metal** | High performance, direct hardware access | Linux, systemd |
| **Kubernetes** | Large scale, cloud native | K8s cluster |

---

## Quick Start (Docker Compose)

```bash
# Clone the repository
git clone https://github.com/GitMonsters/Prime-directive.git
cd Prime-directive/deploy

# Start all services
./deploy.sh start

# Open RustyWorm REPL
./deploy.sh shell

# Check status
./deploy.sh status
```

---

## Docker Compose Deployment

### Prerequisites

- Docker 24.0+
- Docker Compose 2.0+
- 4GB RAM minimum
- 10GB disk space

### Services Included

| Service | Port | Description |
|---------|------|-------------|
| `rustyworm` | - | AI Mimicry Engine (REPL) |
| `agentrl` | 8080 | RL Optimization API |
| `mongodb` | 27017 | Trajectory storage |
| `prometheus` | 9090 | Metrics (optional) |
| `grafana` | 3000 | Dashboards (optional) |

### Commands

```bash
# Start core services
./deploy.sh start

# Start with monitoring
./deploy.sh start --monitoring

# View logs
./deploy.sh logs              # All services
./deploy.sh logs agentrl      # Specific service

# Open RustyWorm REPL
./deploy.sh shell

# Check service health
./deploy.sh health

# Stop services
./deploy.sh stop

# Full cleanup (removes volumes)
./deploy.sh clean
```

### Environment Variables

Create a `.env` file in the deploy directory:

```bash
# API Keys (optional)
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=sk-ant-...
GOOGLE_API_KEY=...

# Service Ports
AGENTRL_PORT=8080
MONGODB_PORT=27017
PROMETHEUS_PORT=9090
GRAFANA_PORT=3000

# RL Configuration
RL_ALGORITHM=ppo
RL_LEARNING_RATE=0.0003

# Logging
LOG_LEVEL=info

# Grafana (if using monitoring)
GRAFANA_PASSWORD=your-secure-password
```

---

## Bare Metal Deployment

### Prerequisites

- Linux (Ubuntu 22.04+, Debian 12+, RHEL 9+)
- Rust toolchain (for building)
- Python 3.11+ (for AgentRL)
- MongoDB 7.0+
- systemd

### Build from Source

```bash
# Clone and build
git clone https://github.com/GitMonsters/Prime-directive.git
cd Prime-directive

# Build release binary with all features
cargo build --features full --release

# Binary is at: target/release/rustyworm
```

### Installation

```bash
# Run the installer as root
sudo ./deploy/install-baremetal.sh
```

This will:
1. Create `rustyworm` user
2. Install binary to `/opt/rustyworm/bin/`
3. Create data directories at `/var/lib/rustyworm/`
4. Install systemd services
5. Create config at `/etc/rustyworm/rustyworm.env`

### Managing Services

```bash
# Enable services to start on boot
sudo systemctl enable rustyworm agentrl mongodb

# Start services
sudo systemctl start mongodb
sudo systemctl start agentrl
sudo systemctl start rustyworm

# Check status
sudo systemctl status rustyworm

# View logs
sudo journalctl -u rustyworm -f

# Run RustyWorm interactively
rustyworm
```

### Configuration Files

| File | Purpose |
|------|---------|
| `/etc/rustyworm/rustyworm.env` | Environment variables |
| `/var/lib/rustyworm/` | Persistent data |
| `/var/log/rustyworm/` | Log files |

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        User / Client                             │
└─────────────────────────────┬───────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                     RustyWorm (Rust)                             │
│                                                                  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │   System 1  │  │   System 2  │  │    AgentCPM Integration │  │
│  │  Fast Path  │  │  Slow Path  │  │    RL + GUI + MCP       │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
│                              │                                   │
└──────────────────────────────┼───────────────────────────────────┘
                               │ HTTP
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│                   AgentRL Service (Python)                       │
│                   FastAPI on port 8080                           │
│                                                                  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │   /health   │  │  /optimize  │  │      /trajectory        │  │
│  │   /policy   │  │  /metrics   │  │      /policy/infer      │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
└──────────────────────────────┬───────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│                      MongoDB                                     │
│                   Trajectory Storage                             │
│                   Port 27017                                     │
└─────────────────────────────────────────────────────────────────┘
```

---

## API Endpoints

### AgentRL Service (port 8080)

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Service health check |
| `/optimize` | POST | Run RL optimization step |
| `/trajectory` | POST | Store trajectory data |
| `/trajectory/{id}` | GET | Retrieve trajectory |
| `/policy` | GET | Get current policy |
| `/policy/infer` | POST | Infer action from policy |
| `/metrics` | GET | Get training metrics |

### Example: Health Check

```bash
curl http://localhost:8080/health
```

### Example: Store Trajectory

```bash
curl -X POST http://localhost:8080/trajectory \
  -H "Content-Type: application/json" \
  -d '{
    "session_id": "abc123",
    "observations": [...],
    "actions": [...],
    "rewards": [...]
  }'
```

---

## Monitoring

Enable the monitoring profile to get Prometheus + Grafana:

```bash
./deploy.sh start --monitoring
```

Access:
- Prometheus: http://localhost:9090
- Grafana: http://localhost:3000 (default: admin/admin)

### Pre-built Dashboards

1. **RustyWorm Overview** - Mimicry sessions, convergence rates
2. **AgentRL Metrics** - RL training progress, policy updates
3. **System Health** - Container resources, request latencies

---

## GPU Support (AgentCPM-GUI)

For GUI agent features, you need a GPU and the AgentCPM-GUI model.

### Enable GPU Service

Uncomment the `agentcpm-gui` service in `docker-compose.yml`:

```yaml
agentcpm-gui:
  image: vllm/vllm-openai:latest
  deploy:
    resources:
      reservations:
        devices:
          - driver: nvidia
            count: 1
            capabilities: [gpu]
```

### Start with GPU

```bash
./deploy.sh start --gpu
```

---

## Troubleshooting

### Container won't start

```bash
# Check logs
./deploy.sh logs rustyworm

# Check Docker status
docker ps -a

# Rebuild images
./deploy.sh build
```

### AgentRL can't connect to MongoDB

```bash
# Check MongoDB is running
docker exec rustyworm-mongodb mongosh --eval "db.adminCommand('ping')"

# Check network
docker network inspect deploy_rustyworm-net
```

### RustyWorm REPL hangs

```bash
# Ensure TTY is allocated
docker exec -it rustyworm /app/rustyworm
```

### High memory usage

```bash
# Check container stats
docker stats

# Adjust memory limits in docker-compose.yml
```

---

## Security Considerations

1. **API Keys**: Never commit `.env` files with real API keys
2. **MongoDB**: Not exposed by default; use authentication in production
3. **Network**: All services run on internal Docker network
4. **Secrets**: Use Docker secrets or vault for production

---

## Files in This Directory

```
deploy/
├── Dockerfile              # RustyWorm production image
├── Dockerfile.agentrl      # AgentRL service image
├── docker-compose.yml      # Full stack orchestration
├── deploy.sh               # Deployment CLI script
├── install-baremetal.sh    # Bare metal installer
├── README.md               # This file
├── .env.example            # Environment template
├── systemd/
│   ├── rustyworm.service   # RustyWorm systemd unit
│   └── agentrl.service     # AgentRL systemd unit
└── monitoring/
    ├── prometheus.yml      # Prometheus config
    └── grafana/
        ├── dashboards/     # Pre-built dashboards
        └── datasources/    # Data source configs
```

---

## Support

- **GitHub Issues**: https://github.com/GitMonsters/Prime-directive/issues
- **Documentation**: https://github.com/GitMonsters/Prime-directive#readme

---

## License

MIT License - Use freely, honor the symbiosis.
