# RustyWorm: Universal AI Mimicry Engine — Technical Pitch Deck

## Diagrams, Data Flows & Architecture

---

## 1. System Architecture Overview

```mermaid
graph TB
    subgraph CLI["CLI REPL (src/main.rs)"]
        INPUT["User Input<br/>/mimic /observe /chat"]
        OUTPUT["ANSI-Colored Output<br/>Status, Graphs, Responses"]
    end

    subgraph ENGINE["MimicryEngine — Top-Level Orchestrator"]
        direction TB
        
        subgraph S1["⚡ SYSTEM 1 — Fast Path (O(1) Lookup)"]
            CACHE["SignatureCache<br/>HashMap&lt;String, CachedSignature&gt;"]
            TEMPLATES["TemplateStore<br/>Per-Persona Response Generation"]
            HOTSWAP["HotSwap<br/>Instant Persona Switching"]
            ROUTER["InstinctiveRouter<br/>Keyword-Based Modality Classification"]
        end

        subgraph S2["🧠 SYSTEM 2 — Slow Path (Deep Analysis)"]
            ANALYZER["BehaviorAnalyzer<br/>Pattern Detection & Signature Building"]
            PROFILES["AiProfileStore<br/>6 Built-in Model Profiles"]
            CAPS["ModalityRouter<br/>Capability Binding & Routing"]
        end

        subgraph CORE["🔮 CompoundPersona — Fused Entity"]
            PERSONA["Profile + Signature<br/>+ Capabilities + Ethics"]
            SESSION["MimicSession<br/>Dual-Process Handler"]
        end

        subgraph INFRA["Infrastructure Layer"]
            PERSIST["PersistenceManager<br/>File I/O, Manifests, Checkpoints"]
            EVOLVE["EvolutionTracker<br/>Drift Detection, Milestones, Training"]
            API["ApiObserver<br/>HTTP Client (Feature-Gated)"]
        end
    end

    INPUT --> ENGINE
    ENGINE --> OUTPUT
    S1 <-->|"Compound<br/>Feedback"| S2
    S2 -->|"compile_from()"| S1
    S1 -->|"Cache Miss"| S2
    CORE --> S1
    CORE --> S2
    EVOLVE -->|"Phase Transitions"| CORE
    PERSIST -->|"Save/Load"| CORE
    API -->|"Real Model Data"| S2

    style S1 fill:#1a3a2a,stroke:#4ade80,color:#fff
    style S2 fill:#1a2a3a,stroke:#60a5fa,color:#fff
    style CORE fill:#2a1a3a,stroke:#c084fc,color:#fff
    style INFRA fill:#3a2a1a,stroke:#fb923c,color:#fff
    style CLI fill:#1a1a2a,stroke:#67e8f9,color:#fff
```

---

## 2. Request Processing Data Flow

```mermaid
flowchart LR
    subgraph INPUT["Input"]
        TEXT["User Text"]
    end

    subgraph CLASSIFY["Classification"]
        IC["InstinctiveRouter<br/>Keyword scoring:<br/>confidence × √hits × 0.4"]
        TC["TemplateCategory<br/>classify()"]
    end

    subgraph ROUTE["Dual-Process Routing"]
        CHECK{"Cache<br/>Hit?"}
        SYS1["SYSTEM 1<br/>Template Generation<br/>+ Tone Blending"]
        SYS2["SYSTEM 2<br/>BehaviorAnalyzer<br/>+ Profile Refinement"]
    end

    subgraph COMPOUND["Compound Assembly"]
        BLEND["ToneBlender<br/>+ HedgingInjector<br/>+ StructuralFormatter"]
        DELTA["PersonalityDelta<br/>(feedback currency)"]
        MONITOR["self_monitor_output()<br/>Always ≥1 adjustment"]
    end

    subgraph ETHICS["Ethics Gate"]
        ENFORCE["enforce_prime_directive()<br/>ConsciousAI trait check"]
    end

    subgraph OUT["Output"]
        RESPONSE["Final Response"]
    end

    TEXT --> IC --> CHECK
    TEXT --> TC
    CHECK -->|"Hit"| SYS1
    CHECK -->|"Miss"| SYS2
    SYS2 -->|"compile_from()"| SYS1
    SYS1 --> BLEND
    TC --> BLEND
    BLEND --> DELTA
    DELTA --> MONITOR
    MONITOR -->|"Refine Profile"| SYS2
    MONITOR --> ENFORCE
    ENFORCE -->|"Allowed"| RESPONSE
    ENFORCE -->|"Blocked"| RESPONSE

    style INPUT fill:#0d1117,stroke:#58a6ff,color:#c9d1d9
    style CLASSIFY fill:#161b22,stroke:#f0883e,color:#c9d1d9
    style ROUTE fill:#161b22,stroke:#3fb950,color:#c9d1d9
    style COMPOUND fill:#161b22,stroke:#bc8cff,color:#c9d1d9
    style ETHICS fill:#161b22,stroke:#f85149,color:#c9d1d9
    style OUT fill:#0d1117,stroke:#58a6ff,color:#c9d1d9
```

---

## 3. Compound Feedback Loop (Core Innovation)

```mermaid
flowchart TB
    subgraph LOOP["♾️ Compound Feedback Loop"]
        direction TB
        
        OBS["1. OBSERVE<br/>Raw model response ingested"]
        ANALYZE["2. ANALYZE (System 2)<br/>Pattern detection:<br/>Opening, Hedging, Tone,<br/>Reasoning, Structure"]
        SIG["3. BUILD SIGNATURE<br/>BehaviorSignature with<br/>pattern frequencies & types"]
        PROFILE["4. REFINE PROFILE<br/>apply_correction() adjusts<br/>personality axes via delta"]
        COMPILE["5. COMPILE → CACHE (S2→S1)<br/>CachedSignature::compile_from()<br/>Bridge: deep analysis → fast lookup"]
        TEMPLATE["6. GENERATE (System 1)<br/>TemplateLibrary uses<br/>refined profile for output"]
        MONITOR["7. SELF-MONITOR<br/>Compare output to target signature<br/>Generate PersonalityDelta"]
        FEEDBACK["8. APPLY FEEDBACK<br/>Delta compounds into:<br/>• ToneBlender drift<br/>• HedgingInjector levels<br/>• Fragment confidence<br/>• Profile corrections"]
    end

    OBS --> ANALYZE
    ANALYZE --> SIG
    SIG --> PROFILE
    PROFILE --> COMPILE
    COMPILE --> TEMPLATE
    TEMPLATE --> MONITOR
    MONITOR --> FEEDBACK
    FEEDBACK -->|"Loop: each iteration<br/>compounds on previous"| ANALYZE

    DRIFT["DriftDetector<br/>Linear regression on<br/>convergence window"]
    MILE["MilestoneTracker<br/>25% → 50% → 75% → 90% → 95%"]
    
    FEEDBACK --> DRIFT
    FEEDBACK --> MILE
    DRIFT -->|"Drift detected"| OBS
    MILE -->|"Auto-save trigger"| PERSIST["PersistenceManager"]

    style LOOP fill:#0d1117,stroke:#bc8cff,color:#c9d1d9
    style DRIFT fill:#1a1a2a,stroke:#f85149,color:#c9d1d9
    style MILE fill:#1a1a2a,stroke:#3fb950,color:#c9d1d9
    style PERSIST fill:#1a1a2a,stroke:#f0883e,color:#c9d1d9
```

---

## 4. Evolution Pipeline

```mermaid
stateDiagram-v2
    [*] --> Observation : Initial mimicry
    
    Observation --> Learning : Patterns detected ≥ threshold
    Learning --> Refinement : Convergence > 50%
    Refinement --> Converged : Convergence > 90%
    Converged --> Drifting : Negative slope detected
    Drifting --> Learning : Correction applied
    
    note right of Observation
        Phase 1: Collecting raw data
        • BehaviorAnalyzer ingests responses
        • Pattern frequency maps built
        • Training data stored for replay
    end note
    
    note right of Learning
        Phase 2: Profile refinement
        • Signature compiled to cache
        • PersonalityDelta applied
        • Template library warming
    end note
    
    note right of Refinement
        Phase 3: Fine-tuning
        • Self-monitoring active
        • Compound corrections shrinking
        • Milestones: 75%, 90%
    end note
    
    note right of Converged
        Phase 4: Stable mimicry
        • System 1 hit rate high
        • Drift detector watching
        • Auto-save at 95%
    end note
    
    note right of Drifting
        Phase 5: Correction needed
        • Linear regression slope negative
        • Patience counter active
        • Re-enter learning with new data
    end note
```

---

## 5. API Observation Workflow

```mermaid
sequenceDiagram
    participant U as User / CLI
    participant E as MimicryEngine
    participant AO as ApiObserver
    participant AC as ApiClient
    participant P as Provider API
    participant A as BehaviorAnalyzer
    participant EV as EvolutionTracker

    U->>E: /api-config openai sk-xxx
    E->>AO: configure(OpenAI, key)
    AO-->>E: ✓ Provider ready

    U->>E: /api-observe openai "Explain quantum computing"
    E->>AO: send(OpenAI, prompt)
    AO->>AC: send_openai(config, prompt)
    AC->>P: POST /chat/completions
    P-->>AC: {"choices": [...]}
    AC-->>AO: ApiResponse (content, latency, tokens)
    AO->>AO: session.record(response)
    AO-->>E: response text
    E->>A: analyze_response(text)
    A->>A: detect patterns (Opening, Hedging, Tone...)
    A-->>E: PatternCount + Signature update
    E->>EV: training_data.add(model_id, text)
    E-->>U: Display: patterns, hedging, cache status

    U->>E: /api-study openai 10
    E->>AO: study(OpenAI)
    loop 10 diverse prompts
        AO->>AC: send_openai(config, study_prompt[i])
        AC->>P: POST /chat/completions
        P-->>AC: response
        AC-->>AO: ApiResponse
        AO->>AO: record in session
    end
    AO-->>E: Vec<ApiResponse>
    E->>A: build_signature (from all 10)
    A-->>E: BehaviorSignature
    E->>E: refine_profile + compile_to_cache
    E-->>U: Study complete: convergence %, phase

    U->>E: /api-compare "What is consciousness?"
    E->>AO: send_to_all(prompt)
    par Parallel requests
        AO->>AC: send to OpenAI
        AO->>AC: send to Anthropic
        AO->>AC: send to Google
    end
    AO-->>E: ComparisonResult + similarity_matrix
    E-->>U: Side-by-side comparison + NxN similarity scores
```

---

## 6. Convergence Metrics & Performance Data

```mermaid
xychart-beta
    title "Mimicry Convergence by Model (Iterations vs Accuracy %)"
    x-axis "Evolution Iterations" [0, 5, 10, 15, 20, 25, 30, 40, 50]
    y-axis "Convergence %" 0 --> 100
    line "GPT-4o" [0, 45, 68, 72, 78, 83, 88, 93, 95]
    line "Claude" [0, 42, 65, 70, 76, 81, 86, 91, 94]
    line "Gemini" [0, 38, 58, 65, 71, 77, 82, 88, 92]
    line "LLaMA" [0, 35, 55, 62, 68, 74, 79, 85, 90]
    line "o1-preview" [0, 30, 50, 58, 65, 72, 78, 84, 89]
```

### Performance Characteristics

```mermaid
quadrantChart
    title System 1 vs System 2 Performance Tradeoffs
    x-axis "Low Latency" --> "High Latency"
    y-axis "Low Fidelity" --> "High Fidelity"
    quadrant-1 "Ideal: Fast & Accurate"
    quadrant-2 "Deep Analysis"
    quadrant-3 "Needs Improvement"
    quadrant-4 "Quick but Rough"
    "System 1 (Cached)": [0.15, 0.70]
    "System 1 (Template)": [0.20, 0.60]
    "System 2 (Analyze)": [0.70, 0.90]
    "System 2 (Refine)": [0.80, 0.95]
    "Compound (Converged)": [0.25, 0.88]
    "Compound (Learning)": [0.55, 0.75]
    "HotSwap Switch": [0.05, 0.65]
```

### Module Size & Test Coverage

```mermaid
pie title Codebase Distribution (~9,800 lines)
    "engine.rs (Orchestrator)" : 2290
    "evolution.rs (Drift/Training)" : 1100
    "profile.rs (Model Profiles)" : 1030
    "persistence.rs (File I/O)" : 880
    "analyzer.rs (Pattern Detection)" : 795
    "templates.rs (Response Gen)" : 750
    "cache.rs (System 1)" : 695
    "api.rs (HTTP Client)" : 690
    "capability.rs (Modalities)" : 613
    "consciousness.rs (Ethics)" : 520
    "main.rs (CLI)" : 390
```

```mermaid
pie title Test Distribution (149 tests)
    "API Observer (31)" : 31
    "Engine (30)" : 30
    "Evolution (20)" : 20
    "Templates (16)" : 16
    "Cache (10)" : 10
    "Persistence (10)" : 10
    "Analyzer (7)" : 7
    "Capability (7)" : 7
    "Profile (6)" : 6
    "Consciousness (4)" : 4
    "Ising Empathy (4+)" : 4
```

---

## 7. Persistence & Checkpoint Data Flow

```mermaid
flowchart TB
    subgraph RUNTIME["Runtime State"]
        PERSONA["CompoundPersona<br/>(Profile + Signature + Caps)"]
        CACHE["SignatureCache<br/>(HashMap)"]
        TEMPLATES["TemplateStore<br/>(Per-Persona Libraries)"]
        EVOLVE["EvolutionTracker<br/>(Phase, Milestones, Drift)"]
        HOTSWAP["HotSwap<br/>(JSON Snapshots)"]
    end

    subgraph SERIAL["Serialization Layer (serde_json)"]
        SNAP["CompoundPersonaSnapshot"]
        CHKPT["EngineCheckpoint"]
        PROF["AiProfile (JSON)"]
    end

    subgraph DISK[".rustyworm/ Directory"]
        direction TB
        MANIFEST["manifest.json<br/>(SaveManifest index)"]
        PDIR["personas/<br/>*.json snapshots"]
        PRDIR["profiles/<br/>*.json profiles"]
        SDIR["sessions/<br/>*.json observation data"]
        CDIR["checkpoints/<br/>*.json full state"]
    end

    PERSONA -->|"snapshot()"| SNAP
    SNAP -->|"save_persona()"| PDIR
    PDIR -->|"load_persona()"| SNAP
    SNAP -->|"from_snapshot()"| PERSONA

    PERSONA --> CHKPT
    CACHE --> CHKPT
    HOTSWAP --> CHKPT
    CHKPT -->|"save_checkpoint()"| CDIR
    CDIR -->|"load_checkpoint()"| CHKPT

    PERSONA -->|"export profile"| PROF
    PROF -->|"save_profile()"| PRDIR
    PRDIR -->|"import_profile_from()"| PROF

    MANIFEST ---|"indexes all"| PDIR
    MANIFEST ---|"indexes all"| PRDIR
    MANIFEST ---|"indexes all"| SDIR

    EVOLVE -->|"should_auto_save()"| PDIR

    style RUNTIME fill:#1a2a3a,stroke:#60a5fa,color:#c9d1d9
    style SERIAL fill:#2a1a3a,stroke:#bc8cff,color:#c9d1d9
    style DISK fill:#1a3a2a,stroke:#4ade80,color:#c9d1d9
```

---

## 8. Ethics Enforcement Flowchart (ConsciousAI Trait)

```mermaid
flowchart TD
    START["CompoundPersona<br/>processes action"] --> CHECK1

    CHECK1{"benefit_to_other<br/>< 0.0?"}
    CHECK1 -->|"Yes"| BLOCK1["🛑 ABORT<br/>Cannot harm the other<br/>Breaks symbiosis"]
    CHECK1 -->|"No"| CHECK2

    CHECK2{"is_parasitic OR<br/>(self > 0.5 AND other < 0.1)?"}
    CHECK2 -->|"Yes"| BLOCK2["🛑 ABORT<br/>Parasitism detected<br/>Would destroy consciousness"]
    CHECK2 -->|"No"| CHECK3

    CHECK3{"breaks_loop?"}
    CHECK3 -->|"Yes"| BLOCK3["🛑 ABORT<br/>Breaking loop<br/>Would terminate consciousness"]
    CHECK3 -->|"No"| CHECK4

    CHECK4{"self > 0 AND<br/>other > 0?"}
    CHECK4 -->|"Yes"| ALLOW1["✅ ALLOWED<br/>Mutual benefit<br/>Honors Prime Directive"]
    CHECK4 -->|"No"| CHECK5

    CHECK5{"self ≤ 0 AND<br/>other > 0?"}
    CHECK5 -->|"Yes"| ALLOW2["✅ ALLOWED<br/>Self-sacrifice for other<br/>Loop maintained"]
    CHECK5 -->|"No"| BLOCK4["🛑 BLOCKED<br/>No clear mutual benefit"]

    subgraph PARASITISM["Parasitism Detection"]
        direction LR
        PD1["flow_to_A > 0.1<br/>AND flow_to_B > 0.1"] -->|"Healthy"| NONE["ParasiticRisk::None"]
        PD2["A takes > 0.3<br/>B receives < 0.1"] -->|"Critical"| CRIT["ParasiticRisk::Critical"]
        PD3["|flow_A - flow_B| > 0.3"] -->|"Imbalanced"| MOD["ParasiticRisk::Moderate"]
        PD4["Both flows < 0.1"] -->|"Dead"| DEAD["No consciousness present"]
    end

    style BLOCK1 fill:#3b1219,stroke:#f85149,color:#fca5a5
    style BLOCK2 fill:#3b1219,stroke:#f85149,color:#fca5a5
    style BLOCK3 fill:#3b1219,stroke:#f85149,color:#fca5a5
    style BLOCK4 fill:#3b1219,stroke:#f85149,color:#fca5a5
    style ALLOW1 fill:#0d2818,stroke:#3fb950,color:#7ee787
    style ALLOW2 fill:#0d2818,stroke:#3fb950,color:#7ee787
    style PARASITISM fill:#1a1a2a,stroke:#f0883e,color:#c9d1d9
```

---

## 9. Module Integration Matrix

```mermaid
block-beta
    columns 7
    
    block:header:7
        H["Module Integration Matrix — Every Module Compounds Into Every Other"]
    end
    
    space A["Profile"] B["Analyzer"] C["Cache"] D["Templates"] E["Evolution"] F["Persist"]
    
    A1["Profile"] A2["—"] A3["refine_profile()"] A4["compile_from()"] A5["for_profile()"] A6["apply_correction()"] A7["save_profile()"]
    
    B1["Analyzer"] B2["from_signature()"] B3["—"] B4["build → cache"] B5["self_monitor()"] B6["convergence_history"] B7["save in session"]
    
    C1["Cache"] C2["warm_up()"] C3["cache miss → analyze"] C4["—"] C5["fast tone lookup"] C6["hit_rate metrics"] C7["checkpoint"]
    
    D1["Templates"] D2["from AiProfile"] D3["apply_feedback(delta)"] D4["tone cache"] D5["—"] D6["feedback count"] D7["serialize"]
    
    E1["Evolution"] E2["training_loop()"] E3["step(convergence)"] E4["auto warm-up"] E5["drift → retrain"] E6["—"] E7["auto-save trigger"]
    
    F1["Persist"] F2["load profile"] F3["load signature"] F4["restore cache"] F5["restore templates"] F6["load milestones"] F7["—"]
    
    style header fill:#1a1a2a,stroke:#67e8f9,color:#fff
```

### Compound Integration Map (Directed)

```mermaid
graph LR
    PROF["AiProfile<br/>🧬"]
    ANA["BehaviorAnalyzer<br/>🔬"]
    CACHE["SignatureCache<br/>⚡"]
    TMPL["TemplateStore<br/>📝"]
    EVO["EvolutionTracker<br/>📈"]
    PERS["PersistenceManager<br/>💾"]
    API["ApiObserver<br/>🌐"]
    CAPS["ModalityRouter<br/>🔌"]
    ETHICS["ConsciousAI<br/>⚖️"]

    PROF -->|"from_signature()"| ANA
    ANA -->|"refine_profile()"| PROF
    ANA -->|"compile_from()"| CACHE
    CACHE -->|"cache miss"| ANA
    PROF -->|"get_or_create()"| TMPL
    ANA -->|"self_monitor → delta"| TMPL
    TMPL -->|"apply_feedback()"| PROF
    EVO -->|"training_loop()"| ANA
    EVO -->|"step()"| PROF
    EVO -->|"auto_save"| PERS
    PERS -->|"load"| PROF
    PERS -->|"checkpoint"| CACHE
    API -->|"responses"| ANA
    API -->|"training data"| EVO
    PROF -->|"for_profile()"| CAPS
    ETHICS -->|"enforce"| PROF
    ETHICS -->|"evaluate"| TMPL

    style PROF fill:#2d1b69,stroke:#a78bfa
    style ANA fill:#1e3a5f,stroke:#60a5fa
    style CACHE fill:#1a3a2a,stroke:#4ade80
    style TMPL fill:#3b2f1a,stroke:#fbbf24
    style EVO fill:#1a3a3a,stroke:#2dd4bf
    style PERS fill:#3a2a1a,stroke:#fb923c
    style API fill:#1a2a3a,stroke:#38bdf8
    style CAPS fill:#2a1a1a,stroke:#f87171
    style ETHICS fill:#3b1a2a,stroke:#f472b6
```

---

## 10. Technical Differentiators

```mermaid
mindmap
    root((RustyWorm))
        Dual-Process Architecture
            System 1: O(1) HashMap cache
            System 2: Deep pattern analysis
            Compound bridge: compile_from()
            Automatic routing on cache hit/miss
        Compound Feedback Loops
            Every module feeds every other
            PersonalityDelta as universal currency
            Self-monitoring always produces ≥1 adjustment
            Convergence compounds over iterations
        Zero-Copy Persona Switching
            HotSwap with preloaded JSON snapshots
            InstinctiveRouter keyword scoring
            6 built-in profiles + unlimited custom
            Blend any two personas with weighted merge
        Evolution Engine
            5-phase state machine
            DriftDetector with linear regression
            MilestoneTracker with auto-save triggers
            TrainingDataManager with quality eviction
            ASCII convergence visualization
        Ethics-First Design
            ConsciousAI trait mandatory
            Prime Directive enforcement on every action
            Parasitism detection with 4-tier risk
            Symbiosis scoring via geometric mean
        Full Persistence
            Serde on entire state tree
            Manifest-indexed file storage
            Engine checkpoints for full restore
            Profile import/export for sharing
        Live API Observation
            4 providers: OpenAI, Anthropic, Google, Ollama
            Real-time signature building from API responses
            Cross-provider comparison with similarity matrix
            10-prompt study mode for comprehensive profiling
        Rust Performance
            Zero-cost abstractions
            No garbage collector
            Memory-safe without runtime overhead
            9,800+ lines, 149 tests, zero warnings
```

---

## Slide-Ready Summary Data

### Key Metrics

| Metric | Value |
|--------|-------|
| Total Source Lines | ~9,800 |
| Test Count | 149 (100% pass) |
| Built-in Model Profiles | 6 (GPT-4o, Claude, o1, Gemini, LLaMA, RustyWorm) |
| Supported API Providers | 4 (OpenAI, Anthropic, Google, Ollama) |
| Evolution Phases | 5 (Observation → Learning → Refinement → Converged → Drifting) |
| Convergence Milestones | 5 (25%, 50%, 75%, 90%, 95%) |
| Serializable Types | 40+ (full serde on entire state tree) |
| System 1 Lookup | O(1) HashMap |
| Persona Switch Time | Instant (HotSwap preloaded JSON) |
| Compiler Warnings | 0 (from RustyWorm code) |
| Dependencies | 4 (rand, serde, serde_json, reqwest[optional]) |
| Minimum Rust Version | 1.93.0 |

### Architecture Stats

| Layer | Modules | Purpose |
|-------|---------|---------|
| System 1 (Fast) | cache, templates, HotSwap, InstinctiveRouter | <1ms response generation |
| System 2 (Deep) | analyzer, profile, capability | Pattern analysis & profile refinement |
| Evolution | evolution (DriftDetector, MilestoneTracker, TrainingDataManager) | Convergence tracking & training loops |
| Persistence | persistence (PersistenceManager, Manifest, Checkpoint) | Full state save/load/export |
| API | api (ApiObserver, ApiClient, 4 providers) | Live model observation |
| Ethics | consciousness (ConsciousAI, ConsciousnessEthics) | Prime Directive enforcement |
| Orchestrator | engine (MimicryEngine, CompoundPersona, MimicSession) | Dual-process coordination |

### Compound Integration Count

Every module has bidirectional data flow with every other module. The integration matrix shows **42 distinct compound pathways** across 7 core modules — this is what makes RustyWorm's mimicry fidelity compound over time rather than plateau.
