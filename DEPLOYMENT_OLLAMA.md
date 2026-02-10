# RustyWorm Docker Deployment with Ollama

## Overview

This guide walks through deploying RustyWorm in Docker with local Ollama for AI model observation, mimicry, and persistence.

**Status**: ✅ **Fully Tested and Working**

## Quick Start

### Prerequisites

1. **Docker**: Installed and running
2. **Ollama**: Installed and running (`ollama serve`)
3. **Model pulled**: `ollama pull llama3` or `ollama pull deepseek-r1:7b`

### Setup (5 minutes)

```bash
# 1. Create persistence directory
mkdir -p ~/.rustyworm

# 2. Pull the Docker image
docker pull ghcr.io/gitmonsters/prime-directive/rustyworm:latest
# OR build locally:
cd /home/worm/Prime-directive/Prime-directive
docker build -t rustyworm:local .

# 3. Run interactive REPL (Linux/native Docker)
docker run --rm -it \
  --network host \
  -v ~/.rustyworm:/data/.rustyworm \
  rustyworm:local
```

**For macOS/Windows** (where `--network host` doesn't work):
```bash
docker run --rm -it \
  -v ~/.rustyworm:/data/.rustyworm \
  -e OLLAMA_HOST=host.docker.internal:11434 \
  rustyworm:local
```

## Inside the RustyWorm REPL

### 1. Configure Ollama API

```bash
/api-config ollama
```

**Output:**
```
✓ API provider Ollama configured [ready]
✓ Profile mapping: Ollama -> llama
```

### 2. Observe a Model Response

```bash
/api-observe llama "Explain tail recursion in one sentence"
```

**What happens:**
- Sends prompt to local Ollama instance (llama3 by default)
- Records response (latency, token count)
- Analyzes behavioral patterns
- Integrates into mimicry training

**Example output:**
```
=== API OBSERVATION: Ollama (llama3) ===
Latency: 7953ms | Tokens: 176
Response (925 chars):
Tail recursion is a technique where the last statement executed is a 
recursive call, allowing the function to reuse its stack frame...

--- Mimicry Pipeline ---
Observed llama response (925 chars).
Patterns detected: 2
Hedging level: 0.00
Avg length: 925
Training samples: 1
Cached: yes
```

### 3. Create a Persona from Observations

```bash
/mimic llama
```

**Output:**
```
=== MORPHING INTO LLaMA ===
I am observing LLaMA. Convergence: 0.0%. I AM HERE.

Capabilities:
✓ Loaded Capability Modules:
  [llama-caps] v1.0
    - text-generation (text) [Advanced]
    - code-generation (code) [Advanced]

Ready. Type anything to chat as LLaMA.
```

### 4. Chat as the Persona

```bash
/chat Tell me about machine learning approaches
```

The engine generates responses using learned patterns from observations.

### 5. Save Persona for Later

```bash
/save my-llama-persona
```

**Output:**
```
✓ Saved persona 'my-llama-persona' (2782 bytes, convergence: 0.0%) 
| Disk: Saved persona 'my-llama-persona' -> 
  .rustyworm/personas/my-llama-persona.json (2782 bytes, convergence: 0.0%)
```

### 6. Persist and Reload

**Next session:**
```bash
/load my-llama-persona
```

**Verification:**
```bash
/status
```

Shows:
- Loaded persona name and convergence score
- System 1 cache hit rate
- Evolution phase
- Saved personas in disk summary

## Persistence Structure

Saved data is stored in `~/.rustyworm/`:

```
~/.rustyworm/
├── personas/           # Saved AI personas (JSON)
│   └── my-llama-persona.json
├── profiles/           # AI model profiles
├── sessions/           # Multi-turn observation sessions
├── checkpoints/        # Full engine state snapshots
└── manifest.json       # Persistence index
```

**Example persona file:**
```json
{
  "profile": {
    "id": "llama",
    "display_name": "LLaMA",
    "version": "3.3-70B",
    "reasoning_style": "DirectWithDepth",
    "personality": [
      {"name": "helpfulness", "value": 0.6},
      {"name": "creativity", "value": 0.5}
    ],
    "response_style": {
      "verbosity": 0.5,
      "formality": 0.5,
      "uses_markdown": true,
      "uses_code_blocks": true
    }
  },
  "signature": {...},
  "capabilities": {...},
  "convergence_score": 0.0,
  "created_at": "session"
}
```

## Complete Workflow Example

```bash
# Terminal 1: Start Ollama
ollama serve

# Terminal 2: Create persistence dir
mkdir -p ~/.rustyworm

# Terminal 3: Run RustyWorm Docker
docker run --rm -it \
  --network host \
  -v ~/.rustyworm:/data/.rustyworm \
  rustyworm:local

# Inside REPL:
/api-config ollama                     # Configure local Ollama
/api-observe llama "What is AI?"       # Observe response
/api-observe llama "Explain neurons"   # Multiple observations
/api-observe llama "How do you learn?" # Build training data

/mimic llama                           # Create persona
/chat Hello, what do you think?        # Chat with persona
/evolve 5                              # Improve persona (5 iterations)

/save smart-llama                      # Save state
/status                                # View status

# Leave and re-enter:
/quit

# Later session:
/load smart-llama                      # Restore persona
/status                                # Verify loaded
/chat How much have you improved?      # Continue where we left off
/quit
```

## Troubleshooting

### Issue: "404 Not Found: model 'llama3' not found"

**Solution:**
Pull the model first:
```bash
ollama pull llama3
```

Available models:
- `llama3` (lightweight, 4.7 GB)
- `deepseek-r1:7b` (reasoning-focused, 4.6 GB)
- `neural-chat` (optimized, 4.0 GB)

### Issue: Container won't start interactively

**On macOS/Windows**, use `host.docker.internal`:
```bash
docker run --rm -it \
  -v ~/.rustyworm:/data/.rustyworm \
  -e OLLAMA_HOST=host.docker.internal:11434 \
  rustyworm:local
```

### Issue: Persistence not working

**Check:**
```bash
# Verify volume is mounted
docker inspect <container-id> | grep Mounts -A 10

# Check directory exists
ls -la ~/.rustyworm/
```

## Advanced: Adding Groq API Support (Future)

To add **Groq** as an API provider (free tier: 14,400 requests/day):

1. **Sign up**: https://console.groq.com
2. **Get API key**: Starts with `gsk_...`
3. **Use as Custom provider** (already supported):
   ```bash
   /api-config groq gsk_YOUR_KEY_HERE
   ```

Future enhancement: Add native `Groq` variant to `ApiProvider` enum in `src/mimicry/api.rs`.

## Commands Reference

### API Operations
- `/api-config <provider> [key]` - Configure API provider
- `/api-observe <provider> <prompt>` - Send prompt to API, observe response
- `/api-study <provider> [n]` - Comprehensive study with n prompts
- `/api-compare <prompt>` - Compare prompt across all providers
- `/api-status` - Show API observer status

### Mimicry
- `/mimic <model>` - Start mimicking a model
- `/observe <model> <text>` - Manual observation
- `/identify <text>` - Identify which model produced text

### Evolution
- `/evolve [n]` - Run n evolution iterations
- `/train [n]` - Train from stored observations
- `/graph` - Show convergence graph

### Persistence
- `/save [name]` - Save persona snapshot
- `/load <name>` - Load saved persona
- `/checkpoint` - Save full engine checkpoint
- `/persist` - Show persistence summary

### Info
- `/help` - Show all commands
- `/status` - Show engine status
- `/list` - List available models and personas
- `/quit` - Exit

## Performance Notes

### Latency
- **First observation**: ~8-15s (model inference)
- **Subsequent observations**: Same (no caching of API responses)
- **Mimicry generation**: ~100-500ms (local, no API call)

### Memory
- **Container size**: ~90 MB
- **Per persona**: ~2-5 KB (JSON)
- **Cache size**: Configurable (default: 1000 entries)

### Storage
Each saved persona is roughly 2-5 KB. You can store hundreds without disk concerns.

## Next Steps

1. **Test with multiple models**:
   ```bash
   /api-observe llama "prompt1"
   /api-observe deepseek-r1 "prompt1"  # Compare responses
   ```

2. **Enable evolution for convergence**:
   ```bash
   /evolve 20  # Run more iterations to improve convergence
   ```

3. **Add Groq for cloud-based AI**:
   ```bash
   GROQ_API_KEY=gsk_... docker run ...
   /api-config groq
   ```

4. **Blend multiple personas**:
   ```bash
   /mimic llama+deepseek 0.7,0.3  # 70% LLaMA, 30% DeepSeek
   ```

## Architecture

```
User Input (REPL)
    ↓
parse_command()
    ↓
api_observe() → ApiClient → Ollama (localhost:11434)
    ↓
BehaviorAnalyzer::build_signature()
    ↓
EvolutionTracker (System 2)
    ↓
SignatureCache (System 1)
    ↓
Persona Output / Persistence
```

**Key components:**
- **System 1 Cache**: Fast pattern matching (hit rate tracking)
- **System 2 Evolution**: Slow analytical improvement
- **Persistence Layer**: Automatic JSON serialization to disk
- **API Observer**: Manages sessions, configurations, studies

## Additional Resources

- **Source**: `/home/worm/Prime-directive/Prime-directive/`
- **Tests**: 139 integration tests (`src/mimicry/engine.rs`)
- **Docker**: `Dockerfile` multi-stage build
- **API**: Supports OpenAI, Anthropic, Google, Ollama, custom endpoints

---

**Last tested**: February 10, 2025
**RustyWorm version**: 2.0.0
**Status**: ✅ Production Ready
