
# Shipyard GNC Development Guide

## Quick Start/Stop

**START (30 seconds):**
```
cd ~/Shipyard_GNC
docker compose up -d --build
```


**STOP (clean):**
```
docker compose down --volumes --remove-orphans
```


## Agent Prompt Guide

**ARCHITECT** - Writes specifications
```
docker compose exec architect python3 agent_talk.py architect "Write ARCHITECTURE SPECIFICATION for [TOPIC]. Output to ARCH_SPEC.md" --overwrite
```


**CODER** - Creates design docs + code ideas
```
docker compose exec coder python3 agent_talk.py coder "From ARCH_SPEC.md, write DESIGN DOCUMENT for [ALGORITHM]. Output to docs/design/[NAME]_DESIGN.md -- NO CODE"
```


**QA ANALYST** - Reviews code/design
```
docker compose exec qa_analyst python3 agent_talk.py qa_analyst "Review [FILE] for [CRITERIA]"
```


**TECH WRITER** - Documents APIs
```
docker compose exec tech_writer python3 agent_talk.py tech_writer "From [FILE] create API docs in docs/[NAME].md"
```


## What We Fixed

**PROBLEM 1**: Docker containers created root-owned files → `sudo` everywhere

**SOLUTION**:
```
docker-compose.yml: user: "${UID:-1000}:1000" on ALL services
.env: UID=$(id -u) + GID=$(id -g)
```

Containers now run as `db42`, no sudo needed ever.

**PROBLEM 2**: Ollama agents invisible

**CAUSE**: Dual Ollama (Windows WSL + Ubuntu native) - Docker saw Ubuntu's first

**SOLUTION**: `OLLAMA_HOST=http://host.docker.internal:11434` points to Windows Ollama
**Removed**: Ubuntu container Ollama service

## Your Workflow

ARCHITECT → ARCH_SPEC.md (done ✅)
↓
CODER → docs/design/VFM_BASELINE.md (done ✅)
↓
YOU → src/gnc/vector_field.rs (TODO)
↓
QA → Reviews your code
↓
TECH WRITER → docs/vector_field.md



## Current State
```
~/Shipyard_GNC/
├── ARCH_SPEC.md (Rust GNC spec w/ VFM+Kalman+PID)
├── docs/design/VFM_BASELINE.md (structs+tests ready)
├── src/gnc/vector_field.rs (needs your VFM algorithm)
├── docker-compose.yml (fixed permissions)
└── .env (your UID/GID)
```


**Ready to code anytime.** Run `docker compose up -d --build` and continue where you left off.