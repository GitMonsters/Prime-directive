# Prime-Directive Local Deployment Guide

**Version**: 2.0
**Date**: February 7, 2026
**Status**: ✅ Production Ready

---

## Quick Start

### Option 1: Using Startup Script (Recommended)

```bash
cd /home/worm/Prime-directive
bash START_LOCAL.sh
```

This will automatically:
- Check dependencies
- Start API server on port 5000
- Display access information
- Run until you press Ctrl+C

### Option 2: Manual Startup

```bash
cd /home/worm/Prime-directive

# Install dependencies (if needed)
pip install flask flask-cors

# Start API server
python3 api_server.py

# In another terminal, access the web interface
# Open browser to http://localhost:8080
```

---

## System Requirements

### Software
- Python 3.8+
- pip package manager
- 4GB RAM minimum
- 500MB disk space

### Dependencies
- flask (web framework)
- flask-cors (CORS support)
- sympy (symbolic math)
- numpy (numerical computing)
- matplotlib (visualization)
- scikit-learn (ML)
- torch (optional, for GPU acceleration)

### Ports
- **Port 5000**: REST API Server
- **Port 8080**: Web Interface (HTTP)

---

## Access Points

### REST API (Port 5000)

Base URL: `http://localhost:5000`

#### Available Endpoints

**System Status**
```
GET /status
```
Response: System health and capabilities

**Submit Query**
```
POST /query
Content-Type: application/json

{
  "query": "What is quantum entanglement?",
  "domain": "quantum_mechanics" (optional)
}
```
Response: Physics answer with domain classification

**List Domains**
```
GET /domains
```
Response: All 24 available physics domains

**Get Domain Info**
```
GET /domain/{domain_name}
```
Example: `/domain/quantum_mechanics`
Response: Domain laws and principles

**Run Simulation**
```
POST /simulate
Content-Type: application/json

{
  "domain": "classical_mechanics",
  "duration": 2.0,
  "dt": 0.01
}
```
Response: Simulation results

**Detect Domain**
```
POST /detect-domain
Content-Type: application/json

{
  "query": "Why does light bend?"
}
```
Response: Domain prediction with confidence

### Web Interface (Port 8080)

Open in browser: `http://localhost:8080`

Features:
- Interactive chat interface
- Real-time query processing
- Domain visualization
- Live status dashboard

---

## Usage Examples

### Example 1: Query API with cURL

```bash
curl -X POST http://localhost:5000/query \
  -H "Content-Type: application/json" \
  -d '{
    "query": "What is the binding energy of a nucleus?"
  }'
```

Response:
```json
{
  "query": "What is the binding energy of a nucleus?",
  "domain": "nuclear_physics",
  "confidence": 0.95,
  "answer": "The binding energy is the energy holding nucleons together...",
  "related_laws": ["binding_energy", "radioactive_decay"],
  "processing_time_ms": 45
}
```

### Example 2: Get Domain Information

```bash
curl http://localhost:5000/domain/nuclear_physics
```

Response:
```json
{
  "domain": "nuclear_physics",
  "laws": {
    "binding_energy": {
      "equation": "BE = (Zm_p + Nm_n - M)*c²",
      "description": "Energy holding nucleus together"
    },
    "radioactive_decay": {...},
    "nuclear_reaction": {...}
  },
  "principles": [
    "Mass Defect",
    "Stability Valley",
    "Fission and Fusion"
  ]
}
```

### Example 3: Run Simulation

```bash
curl -X POST http://localhost:5000/simulate \
  -H "Content-Type: application/json" \
  -d '{
    "domain": "quantum_mechanics",
    "duration": 1.0,
    "dt": 0.01,
    "initial_conditions": {
      "center": 0,
      "width": 1.0
    }
  }'
```

### Example 4: Python Client

```python
import requests
import json

API_URL = "http://localhost:5000"

# Query physics
response = requests.post(
    f"{API_URL}/query",
    json={"query": "Explain quantum entanglement"}
)
result = response.json()
print(f"Domain: {result['domain']}")
print(f"Confidence: {result['confidence']}")
print(f"Answer: {result['answer']}")

# Get domain info
response = requests.get(f"{API_URL}/domain/quantum_mechanics")
domain_info = response.json()
print(f"Laws: {list(domain_info['laws'].keys())}")

# Run simulation
response = requests.post(
    f"{API_URL}/simulate",
    json={
        "domain": "classical_mechanics",
        "duration": 2.0,
        "dt": 0.01
    }
)
sim_result = response.json()
print(f"Simulation steps: {len(sim_result['positions'])}")
```

---

## Configuration

### Environment Variables

```bash
# Set API port (default: 5000)
export PRIME_DIRECTIVE_API_PORT=5000

# Set Web port (default: 8080)
export PRIME_DIRECTIVE_WEB_PORT=8080

# Enable debug mode
export PRIME_DIRECTIVE_DEBUG=true

# Set log level
export PRIME_DIRECTIVE_LOG_LEVEL=INFO
```

### Configuration File

Create `config.yaml` in Prime-directive directory:

```yaml
api:
  host: localhost
  port: 5000
  debug: false

web:
  host: localhost
  port: 8080

logging:
  level: INFO
  file: /tmp/prime-directive/app.log

features:
  enable_gpu: false
  enable_caching: true
  max_query_length: 1000
```

---

## Monitoring & Debugging

### Check Server Status

```bash
# Check if API is running
curl http://localhost:5000/status

# Check system logs
tail -f /tmp/prime-directive/api_server.log

# Monitor system resources
watch -n 1 'ps aux | grep python3'
```

### Performance Monitoring

```bash
# Test query response time
time curl -X POST http://localhost:5000/query \
  -H "Content-Type: application/json" \
  -d '{"query": "Test query"}'
```

### Common Issues

**Issue**: Port 5000 already in use
```bash
# Find process using port 5000
lsof -i :5000

# Kill the process
kill -9 <PID>
```

**Issue**: Module not found errors
```bash
# Reinstall dependencies
pip install --break-system-packages flask flask-cors sympy numpy matplotlib scikit-learn
```

**Issue**: Permission denied on START_LOCAL.sh
```bash
# Make script executable
chmod +x START_LOCAL.sh
```

---

## Advanced Deployment

### Using Gunicorn (Production)

```bash
pip install gunicorn

# Run with multiple workers
gunicorn -w 4 -b 0.0.0.0:5000 api_server:app
```

### Using Docker

```dockerfile
FROM python:3.11-slim

WORKDIR /app

COPY requirements.txt .
RUN pip install -r requirements.txt

COPY . .

EXPOSE 5000 8080

CMD ["python3", "api_server.py"]
```

Build and run:
```bash
docker build -t prime-directive .
docker run -p 5000:5000 -p 8080:8080 prime-directive
```

### Using systemd Service

Create `/etc/systemd/system/prime-directive.service`:

```ini
[Unit]
Description=Prime-Directive Physics System
After=network.target

[Service]
Type=simple
User=worm
WorkingDirectory=/home/worm/Prime-directive
ExecStart=/usr/bin/python3 api_server.py
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

Enable and start:
```bash
sudo systemctl enable prime-directive
sudo systemctl start prime-directive
sudo systemctl status prime-directive
```

---

## Testing Deployment

### Run All Tests

```bash
python3 test_advanced_features.py
```

Expected output:
```
✅ TEST 1: SYMBOLIC MATHEMATICS - PASSING
✅ TEST 2: PHYSICS SIMULATOR - PASSING
✅ TEST 3: VISUALIZATION - PASSING
✅ TEST 4: ML DETECTION - PASSING
✅ TEST 5: EXTENDED DOMAINS - PASSING
✅ TEST 6: INTEGRATION - PASSING

Success Rate: 100% (6/6 tests)
```

### Verify All Domains

```bash
curl http://localhost:5000/domains
```

Should return 24 domains.

---

## Performance Optimization

### Enable Caching

```python
from flask_caching import Cache

cache = Cache(app, config={'CACHE_TYPE': 'simple'})

@app.route('/query', methods=['POST'])
@cache.cached(timeout=300)
def query():
    # Query processing
    pass
```

### GPU Acceleration

```bash
# Check GPU support
python3 -c "import torch; print(torch.cuda.is_available())"

# Enable GPU in API
export ENABLE_GPU=true
```

### Load Balancing

```bash
# Using nginx for reverse proxy
# See production deployment guide
```

---

## Stopping the Server

### Using Ctrl+C (Recommended)
- Press `Ctrl+C` in the terminal running the server
- Graceful shutdown will occur

### Using Kill Command
```bash
# Find process ID
ps aux | grep api_server.py

# Kill process
kill <PID>

# Force kill if needed
kill -9 <PID>
```

### Using systemd
```bash
sudo systemctl stop prime-directive
```

---

## Next Steps

### Immediate
- [ ] Verify all endpoints work
- [ ] Test with sample queries
- [ ] Check performance metrics
- [ ] Review API logs

### Short Term
- [ ] Set up monitoring
- [ ] Configure auto-restart
- [ ] Enable CORS for production
- [ ] Set up SSL/TLS certificates

### Production Deployment
- [ ] Deploy to cloud (AWS/GCP/Azure)
- [ ] Set up load balancing
- [ ] Configure authentication
- [ ] Enable comprehensive logging
- [ ] Set up backup systems

---

## Support & Troubleshooting

### Getting Help

1. **Check Logs**
   ```bash
   tail -f /tmp/prime-directive/api_server.log
   ```

2. **Test Endpoints**
   ```bash
   curl http://localhost:5000/status
   ```

3. **Review Documentation**
   - See `README.md` for overview
   - See `ADVANCED_FEATURES_SESSION_COMPLETE.md` for feature details
   - See `physics_advanced_features.md` for API specifics

### Reporting Issues

Include the following when reporting issues:
- Server logs (`/tmp/prime-directive/api_server.log`)
- System info (Python version, OS, RAM)
- Steps to reproduce
- Expected vs actual behavior

---

## Resources

- **Main README**: See `README.md`
- **Advanced Features**: See `physics_advanced_features.md`
- **API Documentation**: See docstrings in `api_server.py`
- **Examples**: See `test_advanced_features.py`

---

**Happy Physics Computing! 🚀**

For the latest information, visit the Prime-Directive repository or check the documentation files.

