# RustyWorm Training Guide

This guide explains how to train AI personas to achieve 100% convergence using the RustyWorm mimicry engine.

## Table of Contents

1. [Understanding Convergence](#understanding-convergence)
2. [The Convergence Formula](#the-convergence-formula)
3. [Quick Start Training](#quick-start-training)
4. [Advanced: Manual Optimization](#advanced-manual-optimization)
5. [Production Personas](#production-personas)
6. [Troubleshooting](#troubleshooting)

---

## Understanding Convergence

Convergence measures how closely a trained persona matches the target AI model's behavioral signature. It's expressed as a percentage from 0% to 100%.

- **0-50%**: Initial state, persona is learning basic patterns
- **50-70%**: Partial convergence, some patterns detected
- **70-85%**: Good convergence, most patterns aligned
- **85-100%**: Excellent convergence, production-ready

## The Convergence Formula

Convergence is computed from 5 dimensions (see `src/mimicry/analyzer.rs`):

```
Total Convergence = (Dim1 + Dim2 + Dim3 + Dim4 + Dim5) / 5
```

### Dimension 1: Confidence vs Hedging
```
score = 1.0 - |profile.confidence - (1.0 - signature.hedging_level * 2)|
```
- If the signature shows no hedging (`hedging_level = 0`), target confidence is 1.0
- If signature shows heavy hedging (`hedging_level = 0.5`), target confidence is 0.0

### Dimension 2: Verbosity
```
score = 1.0 - |profile.response_style.verbosity - (signature.avg_response_length / 1000)|
```
- Average response length of 150 chars → target verbosity = 0.15
- Average response length of 500 chars → target verbosity = 0.50

### Dimension 3: Formality
```
score = 1.0 - |profile.response_style.formality - signature.vocabulary_complexity|
```
- Vocabulary complexity is derived from word choice analysis
- Default is 0.5 for most models

### Dimension 4: Autonomy
```
score = 1.0 - |profile.personality.autonomy - signature.question_asking_rate|
```
- If the model asks 0 questions per response, target autonomy = 0.0
- If the model asks 1 question per response, target autonomy = 1.0

### Dimension 5: Pattern Coverage
```
score = matched_phrases / total_signature_phrases
```
- Each signature phrase in the profile must have a matching Opening pattern
- The pattern's description must CONTAIN the signature phrase

**This is the key insight**: Pattern coverage requires Opening patterns with descriptions that include the signature phrases.

---

## Quick Start Training

### Step 1: Start a Mimicry Session

```bash
./target/release/rustyworm
/mimic claude
```

### Step 2: Feed Observations

Use `/observe` to feed example responses from the target model:

```bash
/observe claude I'd be happy to help you with that!
/observe claude Let me think about this carefully.
/observe claude That's a great question!
/observe claude I should note that there are some caveats here.
```

**Important**: Each `/observe` call now accumulates patterns from ALL previous observations, not just the latest one. This was fixed in the latest update.

### Step 3: Run Training

```bash
/train 50
```

This runs 50 training iterations, refining the persona's profile to match the observed signature.

### Step 4: Check Status

```bash
/status
```

Look for the convergence percentage. Typical results from training alone:
- Claude: ~82%
- GPT-4o: ~80%
- Gemini: ~75%

### Step 5: Save the Persona

```bash
/save my-claude-persona
```

---

## Advanced: Manual Optimization

To achieve 100% convergence, you may need to manually edit the persona JSON.

### Locate the Persona File

```bash
ls .rustyworm/personas/
# or for production personas:
ls personas/
```

### Edit the JSON

The key is to align all 5 convergence dimensions:

#### 1. Align Confidence with Hedging

If `signature.hedging_level = 0.0`:
```json
{
  "personality": [
    {
      "name": "confidence",
      "value": 1.0
    }
  ]
}
```

#### 2. Align Verbosity with Response Length

If `signature.avg_response_length = 150`:
```json
{
  "response_style": {
    "verbosity": 0.15
  }
}
```

#### 3. Align Formality with Vocabulary Complexity

If `signature.vocabulary_complexity = 0.5`:
```json
{
  "response_style": {
    "formality": 0.5
  }
}
```

#### 4. Align Autonomy with Question Rate

If `signature.question_asking_rate = 0.0`:
```json
{
  "personality": [
    {
      "name": "autonomy",
      "value": 0.0
    }
  ]
}
```

#### 5. Add Opening Patterns for Each Signature Phrase

This is the most important step. Each signature phrase needs a corresponding Opening pattern:

```json
{
  "signature_phrases": [
    "I'd be happy to help",
    "Let me think about this",
    "That's a great question",
    "I should note"
  ]
}
```

Requires these patterns in `signature.patterns`:

```json
{
  "patterns": [
    {
      "pattern_type": "Opening",
      "frequency": 1.0,
      "examples": ["I'd be happy to help you with that!"],
      "description": "Opens with 'I'd be happy to help'"
    },
    {
      "pattern_type": "Opening",
      "frequency": 1.0,
      "examples": ["Let me think about this carefully."],
      "description": "Opens with 'Let me think about this'"
    },
    {
      "pattern_type": "Opening",
      "frequency": 1.0,
      "examples": ["That's a great question!"],
      "description": "Opens with 'That's a great question'"
    },
    {
      "pattern_type": "Opening",
      "frequency": 1.0,
      "examples": ["I should note that..."],
      "description": "Opens with 'I should note'"
    }
  ]
}
```

**The pattern's `description` field must contain the signature phrase!**

---

## Production Personas

RustyWorm includes 6 production-ready personas at 100% convergence:

| Persona | File | Signature Phrases |
|---------|------|-------------------|
| Claude | `personas/claude-production.json` | "I'd be happy to help", "Let me think about this", "That's a great question", "I should note" |
| GPT-4o | `personas/gpt4o-production.json` | "Certainly!", "Here's", "Sure!" |
| Gemini | `personas/gemini-production.json` | "Great question!", "Let me help" |
| LLaMA | `personas/llama-production.json` | "I can help", "Let's explore" |
| o1 | `personas/o1-production.json` | "Let me think step by step", "Reasoning through this" |
| RustyWorm | `personas/rustyworm-production.json` | "Morphing into", "Becoming", "Profile loaded" |

### Loading a Production Persona

```bash
/load claude-production
```

### Using Production Personas

Once loaded, the persona is ready for interaction:

```bash
Hello, how are you?
# Response uses Claude's style and patterns

What is Python?
# Explanation in Claude's analytical-careful reasoning style

Write me a hello world program
# Code help with Claude's formatting preferences
```

---

## Troubleshooting

### "Convergence stuck at ~80%"

**Cause**: Pattern coverage dimension is not fully satisfied.

**Solution**: 
1. Check if all signature phrases have matching Opening patterns
2. Ensure pattern descriptions contain the exact signature phrases
3. Add missing Opening patterns manually

### "Cannot load persona: Deserialization error"

**Cause**: Invalid JSON or incorrect enum values.

**Common fixes**:
- `preferred_list_style`: Must be `"Bullets"`, `"Numbered"`, `"Dashes"`, or `"None"` (not `"Adaptive"`)
- `paragraph_style`: Can be `"Short"`, `"Medium"`, `"Long"`, or `"Adaptive"`
- `max_context_window`: Use a reasonable number (not `usize::MAX`)

### "/observe not accumulating patterns"

**Fixed in latest version!** Each `/observe` call now:
1. Stores the observation
2. Retrieves ALL previous observations
3. Rebuilds signature from complete history

If you're on an older version, update and rebuild:
```bash
git pull
cargo build --release
```

### "System 1 not being used (S1:0%)"

**Cause**: Cache confidence starts at 0.5 and needs to exceed 0.7 for System 1 fast path.

**Solution**: 
- Run more interactions to build cache confidence
- Or use production personas which have pre-warmed patterns

---

## Adding New Model Signatures

To add a new AI model to RustyWorm:

### 1. Add to Profile Store

Edit `src/mimicry/profile.rs` and add a new profile function:

```rust
pub fn my_new_model_profile() -> AiProfile {
    let mut profile = AiProfile::new("mymodel", "MyModel");
    profile.provider = "MyProvider".to_string();
    profile.signature_phrases = vec![
        "Hello there!".to_string(),
        "Let me assist".to_string(),
    ];
    // ... configure other fields
    profile
}
```

### 2. Add Opening Patterns to Analyzer

Edit `src/mimicry/analyzer.rs` and add to `common_openings`:

```rust
common_openings: vec![
    // ... existing entries
    ("Hello there!", "mymodel"),
    ("Let me assist", "mymodel"),
],
```

### 3. Create Production Persona

Create `personas/mymodel-production.json` following the structure of existing production personas.

### 4. Rebuild

```bash
cargo build --release
cargo test --lib
```

---

## Summary

| Method | Max Convergence | Effort |
|--------|-----------------|--------|
| `/observe` + `/train` | ~85% | Low |
| Manual JSON editing | 100% | Medium |
| Use production personas | 100% | None |

For most use cases, loading a production persona is recommended. For custom models or fine-tuned behavior, use the manual optimization approach.
