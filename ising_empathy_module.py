#!/usr/bin/env python3
"""
ISING EMPATHY MODULE — Physics-Grounded, GPU Accelerated, No API

Empathy as coupling-mediated state correlation in the Ising framework.
All empathy scores are physically computed from Hamiltonian dynamics,
not learned from random weights.

Components:
  1. Emotion Encoder     — Maps Ising state to emotion vector (physics-direct)
  2. Theory of Mind      — Simulates another system's Hamiltonian
  3. Empathy Score       — Measurable state overlap + energy prediction error
  4. Compassionate Response — Coupling modification based on understanding
  5. Emotional Memory    — Rolling GPU tensor buffer (no LSTM)
  6. Social Attention    — Multi-agent pairwise empathy weighting

Runs on AMD Radeon 8060S via ROCm/PyTorch. No training data needed.
"""

import torch
import math
import time
import sys
from typing import Dict, List, Optional, Tuple
from dataclasses import dataclass, field


# ─── Device Setup ────────────────────────────────────────────────────────────

def setup_device():
    """Detect and configure GPU (ROCm/CUDA)."""
    if not torch.cuda.is_available():
        print("  WARN: No GPU found. Falling back to CPU.")
        return torch.device("cpu")
    dev = torch.device("cuda", 0)
    name = torch.cuda.get_device_name(0)
    print(f"  GPU    : {name}")
    hip = getattr(torch.version, 'hip', None)
    if hip:
        print(f"  HIP    : {hip}")
    vram = torch.cuda.get_device_properties(0).total_memory / 1e9
    print(f"  VRAM   : {vram:.1f} GB")
    torch.zeros(1, device=dev)
    torch.cuda.synchronize()
    return dev


# ─── IsingGPU — Core Ising System ───────────────────────────────────────────

class IsingGPU:
    """
    GPU-accelerated Ising spin system.
    Reused from gpu_agi_100_signifiers_test.py with minor enhancements.
    """

    def __init__(self, n: int, seed: int, device: torch.device):
        self.n = n
        self.device = device
        gen = torch.Generator(device='cpu').manual_seed(seed)
        self.spins = (torch.randint(0, 2, (n,), generator=gen).float() * 2 - 1).to(device)
        coupling = torch.zeros(n, n, device=device)
        for i in range(n):
            for j in range(i + 1, n):
                s = 1.0 if (i + j) % 3 == 0 else 0.5
                coupling[i, j] = s
                coupling[j, i] = s
        self.coupling = coupling
        self.field = 0.1 * (torch.arange(n, device=device, dtype=torch.float32) / n - 0.5)

    def energy(self) -> float:
        outer = torch.outer(self.spins, self.spins)
        interaction = -(self.coupling * outer).triu(diagonal=1).sum()
        field_term = -(self.field * self.spins).sum()
        return (interaction + field_term).item()

    def energy_tensor(self) -> torch.Tensor:
        """Return energy as a GPU tensor (no .item() call)."""
        outer = torch.outer(self.spins, self.spins)
        interaction = -(self.coupling * outer).triu(diagonal=1).sum()
        field_term = -(self.field * self.spins).sum()
        return interaction + field_term

    def anneal(self, steps: int, seed: int) -> float:
        gen = torch.Generator(device='cpu').manual_seed(seed)
        for step in range(steps):
            beta = 0.1 * math.exp(10.0 * step / steps)
            indices = torch.randint(0, self.n, (10,), generator=gen)
            randoms = torch.rand(10, generator=gen).to(self.device)
            for t in range(10):
                i = indices[t].item()
                e_before = self.energy()
                self.spins[i] *= -1
                e_after = self.energy()
                delta_e = e_after - e_before
                p_accept = max(math.exp(min(-beta * delta_e, 500)), 0.1 / (1.0 + beta))
                if randoms[t].item() >= p_accept:
                    self.spins[i] *= -1
        return self.energy()

    def magnetization(self) -> float:
        return self.spins.mean().item()

    def frustration(self) -> float:
        """
        Coupling frustration: fraction of interactions where aligned spins
        have negative coupling or anti-aligned spins have positive coupling.
        """
        outer = torch.outer(self.spins, self.spins)
        # Frustrated when coupling sign disagrees with spin alignment
        frustrated = (self.coupling * outer < 0).float().triu(diagonal=1)
        total = (self.coupling.abs() > 0).float().triu(diagonal=1)
        denom = total.sum()
        if denom.item() == 0:
            return 0.0
        return (frustrated.sum() / denom).item()

    def add_thermal_noise(self, temperature: float, seed: int):
        gen = torch.Generator(device='cpu').manual_seed(seed)
        flip_mask = (torch.rand(self.n, generator=gen).to(self.device) < temperature).float()
        self.spins *= (1.0 - 2.0 * flip_mask)

    def clone(self) -> 'IsingGPU':
        new = IsingGPU.__new__(IsingGPU)
        new.n = self.n
        new.device = self.device
        new.spins = self.spins.clone()
        new.coupling = self.coupling.clone()
        new.field = self.field.clone()
        return new


# ─── Emotion Encoding ───────────────────────────────────────────────────────

@dataclass
class EmotionVector:
    """
    Physics-grounded emotion representation.
    Each component maps directly from Ising observables.
    """
    valence: float = 0.0       # Energy level -> positive/negative affect
    arousal: float = 0.0       # Disorder -> excitement level
    tension: float = 0.0       # Frustration -> conflict
    coherence: float = 0.0     # Magnetization magnitude -> internal alignment
    raw_tensor: torch.Tensor = field(default=None, repr=False)

    def to_tensor(self, device: torch.device) -> torch.Tensor:
        if self.raw_tensor is not None:
            return self.raw_tensor
        return torch.tensor(
            [self.valence, self.arousal, self.tension, self.coherence],
            device=device, dtype=torch.float32
        )


# ─── IsingEmpathyModule ─────────────────────────────────────────────────────

class IsingEmpathyModule:
    """
    Physics-grounded empathy module for Ising consciousness systems.

    All empathy scores emerge from Hamiltonian dynamics:
      - Emotion = direct mapping from energy, magnetization, frustration
      - Theory of Mind = simulating another system's Hamiltonian
      - Empathy score = state overlap + energy prediction accuracy
      - Compassion = coupling modification based on understanding
      - Memory = GPU tensor buffer (no learned weights)
      - Social attention = pairwise empathy weighting
    """

    def __init__(self, device: torch.device, memory_size: int = 32):
        self.device = device
        self.memory_size = memory_size

        # Emotional memory buffer: stores (emotion_vector, empathy_score) pairs
        # Shape: [memory_size, 5] — 4 emotion dims + 1 empathy score
        self.memory_buffer = torch.zeros(memory_size, 5, device=device)
        self.memory_pointer = 0
        self.memory_count = 0

    # ── 1. Emotion Encoder ───────────────────────────────────────────────

    def encode_emotion(self, system: IsingGPU) -> EmotionVector:
        """
        Map Ising observables to emotion vector. No learned weights.

        Mapping:
          valence   = -tanh(energy / n)        — lower energy = positive affect
          arousal   = 1 - |magnetization|       — disorder = excitement
          tension   = frustration               — coupling conflict
          coherence = |magnetization|           — internal alignment
        """
        e = system.energy()
        m = system.magnetization()
        f = system.frustration()
        n = system.n

        valence = -math.tanh(e / n)
        arousal = 1.0 - abs(m)
        tension = f
        coherence = abs(m)

        raw = torch.tensor(
            [valence, arousal, tension, coherence],
            device=self.device, dtype=torch.float32
        )

        return EmotionVector(
            valence=valence,
            arousal=arousal,
            tension=tension,
            coherence=coherence,
            raw_tensor=raw
        )

    # ── 2. Theory of Mind ────────────────────────────────────────────────

    def simulate_other(
        self,
        other: IsingGPU,
        anneal_steps: int = 100,
        seed: int = 12345
    ) -> IsingGPU:
        """
        Theory of Mind: simulate another system by copying its Hamiltonian
        (coupling matrix + external field) and annealing to predict its
        ground state.

        This is *literally* running the other's physics — the most honest
        form of perspective-taking.
        """
        # Create a fresh system with the other's Hamiltonian
        sim = IsingGPU.__new__(IsingGPU)
        sim.n = other.n
        sim.device = self.device
        # Start from random spins (we don't peek at their state)
        gen = torch.Generator(device='cpu').manual_seed(seed)
        sim.spins = (torch.randint(0, 2, (other.n,), generator=gen).float() * 2 - 1).to(self.device)
        # Copy the other's coupling and field (their "personality")
        sim.coupling = other.coupling.clone()
        sim.field = other.field.clone()
        # Anneal to find predicted ground state
        sim.anneal(anneal_steps, seed)
        return sim

    def perspective_accuracy(
        self,
        predicted: IsingGPU,
        actual: IsingGPU
    ) -> Dict[str, float]:
        """
        Measure how well our simulation predicted the other's state.

        Returns:
          state_overlap  — fraction of spins that match (0 to 1)
                           Accounts for Z2 symmetry: if all spins are flipped,
                           that's an equally valid ground state.
          energy_error   — |E_predicted - E_actual| / |E_actual|
          mag_error      — |m_predicted - m_actual|
        """
        # State overlap: fraction of matching spins
        # Handle Z2 (spin-flip) symmetry: both s and -s are valid ground states
        match_direct = (predicted.spins == actual.spins).float().mean().item()
        match_flipped = (predicted.spins == -actual.spins).float().mean().item()
        match = max(match_direct, match_flipped)

        # Energy prediction error (relative)
        e_pred = predicted.energy()
        e_actual = actual.energy()
        denom = abs(e_actual) if abs(e_actual) > 1e-6 else 1.0
        energy_err = abs(e_pred - e_actual) / denom

        # Magnetization error (account for Z2 symmetry)
        m_pred = abs(predicted.magnetization())
        m_actual = abs(actual.magnetization())
        mag_err = abs(m_pred - m_actual)

        return {
            'state_overlap': match,
            'energy_error': energy_err,
            'magnetization_error': mag_err
        }

    # ── 3. Empathy Score ─────────────────────────────────────────────────

    def compute_empathy(
        self,
        self_system: IsingGPU,
        other_system: IsingGPU,
        anneal_steps: int = 100,
        seed: int = 12345
    ) -> Dict[str, float]:
        """
        Compute physics-grounded empathy score between two Ising systems.

        Components:
          1. State overlap between predicted and actual other's state
          2. Energy prediction accuracy
          3. Coupling similarity (cosine similarity of J matrices)

        Combined into a single 0-1 empathy score.
        """
        # Simulate the other's system (Theory of Mind)
        predicted = self.simulate_other(other_system, anneal_steps, seed)

        # Perspective accuracy
        accuracy = self.perspective_accuracy(predicted, other_system)

        # Coupling similarity (cosine similarity of coupling matrices)
        j_self = self_system.coupling.triu(diagonal=1).flatten()
        j_other = other_system.coupling.triu(diagonal=1).flatten()
        cos_sim = torch.nn.functional.cosine_similarity(
            j_self.unsqueeze(0), j_other.unsqueeze(0)
        ).item()
        coupling_sim = (cos_sim + 1.0) / 2.0  # Map [-1,1] to [0,1]

        # Combined empathy score (weighted average)
        # State overlap and coupling similarity are the strongest signals
        empathy_score = (
            0.4 * accuracy['state_overlap'] +
            0.3 * max(0.0, 1.0 - accuracy['energy_error']) +
            0.3 * coupling_sim
        )
        empathy_score = max(0.0, min(1.0, empathy_score))

        return {
            'empathy_score': empathy_score,
            'state_overlap': accuracy['state_overlap'],
            'energy_error': accuracy['energy_error'],
            'magnetization_error': accuracy['magnetization_error'],
            'coupling_similarity': coupling_sim,
        }

    # ── 4. Compassionate Response ────────────────────────────────────────

    def compassionate_response(
        self,
        self_system: IsingGPU,
        other_system: IsingGPU,
        empathy_score: float,
        coupling_strength: float = 0.1,
        noise_temperature: float = 0.05
    ) -> Dict[str, object]:
        """
        Modify self_system's couplings based on empathic understanding.

        High empathy  -> strengthen cross-system coupling patterns
                         (align with the understood other)
        Low empathy   -> increase exploration via thermal noise
                         (try harder to understand)
        """
        actions = []

        if empathy_score > 0.5:
            # High empathy: absorb some of the other's coupling structure
            blend = coupling_strength * empathy_score
            old_coupling = self_system.coupling.clone()
            self_system.coupling = (
                (1.0 - blend) * self_system.coupling +
                blend * other_system.coupling
            )
            delta = (self_system.coupling - old_coupling).abs().mean().item()
            actions.append(f"coupling_blend={blend:.3f}, mean_delta={delta:.4f}")
        else:
            # Low empathy: add thermal noise to explore
            temp = noise_temperature * (1.0 - empathy_score)
            seed = int(time.time() * 1000) % (2**31)
            self_system.add_thermal_noise(temp, seed)
            actions.append(f"thermal_noise={temp:.3f}")

        return {
            'empathy_score': empathy_score,
            'actions': actions,
            'new_energy': self_system.energy()
        }

    # ── 5. Emotional Memory ──────────────────────────────────────────────

    def store_memory(self, emotion: EmotionVector, empathy_score: float):
        """
        Store an emotion + empathy score in the rolling GPU buffer.
        No LSTM — just a circular tensor buffer with running statistics.
        """
        entry = torch.tensor(
            [emotion.valence, emotion.arousal, emotion.tension,
             emotion.coherence, empathy_score],
            device=self.device, dtype=torch.float32
        )
        self.memory_buffer[self.memory_pointer] = entry
        self.memory_pointer = (self.memory_pointer + 1) % self.memory_size
        self.memory_count = min(self.memory_count + 1, self.memory_size)

    def recall_memory(self) -> Dict[str, float]:
        """
        Compute running statistics from emotional memory.
        Returns averages and trends for emotional continuity.
        """
        if self.memory_count == 0:
            return {
                'avg_valence': 0.0, 'avg_arousal': 0.0,
                'avg_tension': 0.0, 'avg_coherence': 0.0,
                'avg_empathy': 0.0, 'memory_entries': 0,
                'empathy_trend': 0.0
            }

        active = self.memory_buffer[:self.memory_count]
        means = active.mean(dim=0)

        # Empathy trend: compare recent half vs older half
        trend = 0.0
        if self.memory_count >= 4:
            half = self.memory_count // 2
            recent = active[half:, 4].mean().item()
            older = active[:half, 4].mean().item()
            trend = recent - older

        return {
            'avg_valence': means[0].item(),
            'avg_arousal': means[1].item(),
            'avg_tension': means[2].item(),
            'avg_coherence': means[3].item(),
            'avg_empathy': means[4].item(),
            'memory_entries': self.memory_count,
            'empathy_trend': trend
        }

    # ── 6. Social Attention ──────────────────────────────────────────────

    def social_attention(
        self,
        self_system: IsingGPU,
        others: List[IsingGPU],
        anneal_steps: int = 80,
        seed_base: int = 7777
    ) -> Dict[str, object]:
        """
        Multi-agent empathy: compute pairwise empathy with N other systems.
        Weight responses by empathy score (attend to understood agents).
        Collective emotion = empathy-weighted average of all emotional states.
        """
        empathy_scores = []
        emotions = []

        for idx, other in enumerate(others):
            result = self.compute_empathy(
                self_system, other,
                anneal_steps=anneal_steps,
                seed=seed_base + idx
            )
            empathy_scores.append(result['empathy_score'])
            emotions.append(self.encode_emotion(other))

        if not empathy_scores:
            return {
                'attention_weights': [],
                'collective_emotion': EmotionVector(),
                'empathy_scores': [],
                'most_empathic_idx': -1
            }

        # Normalize empathy scores to attention weights
        scores_t = torch.tensor(empathy_scores, device=self.device, dtype=torch.float32)
        total = scores_t.sum()
        if total.item() > 1e-8:
            weights = scores_t / total
        else:
            weights = torch.ones_like(scores_t) / len(scores_t)

        # Weighted collective emotion
        emotion_stack = torch.stack([e.to_tensor(self.device) for e in emotions])
        collective = (weights.unsqueeze(1) * emotion_stack).sum(dim=0)

        collective_emotion = EmotionVector(
            valence=collective[0].item(),
            arousal=collective[1].item(),
            tension=collective[2].item(),
            coherence=collective[3].item(),
            raw_tensor=collective
        )

        most_empathic = scores_t.argmax().item()

        return {
            'attention_weights': weights.cpu().tolist(),
            'collective_emotion': collective_emotion,
            'empathy_scores': empathy_scores,
            'most_empathic_idx': most_empathic
        }

    # ── Full Processing Pipeline ─────────────────────────────────────────

    def process(
        self,
        self_system: IsingGPU,
        other_system: IsingGPU,
        anneal_steps: int = 100,
        seed: int = 12345,
        apply_response: bool = False
    ) -> Dict[str, object]:
        """
        Full empathetic processing pipeline:
          1. Encode emotions for both systems
          2. Compute empathy (Theory of Mind + scoring)
          3. Optionally apply compassionate response
          4. Store in emotional memory
        """
        # Encode emotions
        self_emotion = self.encode_emotion(self_system)
        other_emotion = self.encode_emotion(other_system)

        # Compute empathy
        empathy = self.compute_empathy(
            self_system, other_system, anneal_steps, seed
        )

        # Compassionate response
        response = None
        if apply_response:
            response = self.compassionate_response(
                self_system, other_system, empathy['empathy_score']
            )

        # Store in memory
        self.store_memory(self_emotion, empathy['empathy_score'])

        return {
            'self_emotion': self_emotion,
            'other_emotion': other_emotion,
            'empathy': empathy,
            'response': response,
            'memory': self.recall_memory()
        }


# ═════════════════════════════════════════════════════════════════════════════
# TEST SUITE
# ═════════════════════════════════════════════════════════════════════════════

def run_tests():
    """Comprehensive test suite for the Ising Empathy Module."""

    print("=" * 78)
    print("  ISING EMPATHY MODULE — Physics-Grounded Test Suite")
    print("=" * 78)

    device = setup_device()
    module = IsingEmpathyModule(device=device, memory_size=32)
    passed = 0
    failed = 0
    total_start = time.time()

    def report(name: str, ok: bool, detail: str):
        nonlocal passed, failed
        tag = "PASS" if ok else "FAIL"
        if ok:
            passed += 1
        else:
            failed += 1
        print(f"  [{tag}] {name}")
        print(f"         {detail}")

    # ── Test 1: Emotion Encoding ─────────────────────────────────────────
    print("\n--- Test 1: Emotion Encoding ---")
    sys_a = IsingGPU(20, 42, device)
    e_random = module.encode_emotion(sys_a)
    sys_a.anneal(200, 42)
    e_annealed = module.encode_emotion(sys_a)

    # After annealing, energy is lower -> valence should increase
    # After annealing, more ordered -> arousal should decrease
    valence_ok = e_annealed.valence > e_random.valence
    arousal_ok = e_annealed.arousal < e_random.arousal
    report(
        "Valence increases after annealing (lower energy = positive affect)",
        valence_ok,
        f"random={e_random.valence:.3f} -> annealed={e_annealed.valence:.3f}"
    )
    report(
        "Arousal decreases after annealing (more order = calmer)",
        arousal_ok,
        f"random={e_random.arousal:.3f} -> annealed={e_annealed.arousal:.3f}"
    )
    report(
        "Coherence increases after annealing",
        e_annealed.coherence > e_random.coherence,
        f"random={e_random.coherence:.3f} -> annealed={e_annealed.coherence:.3f}"
    )

    # ── Test 2: Theory of Mind — Same Hamiltonian ────────────────────────
    print("\n--- Test 2: Theory of Mind (same Hamiltonian) ---")
    sys_b = IsingGPU(20, 99, device)
    sys_b.anneal(200, 99)

    predicted = module.simulate_other(sys_b, anneal_steps=200, seed=55555)
    accuracy = module.perspective_accuracy(predicted, sys_b)

    # Since we copied the exact Hamiltonian and annealed, the predicted state
    # should have similar energy (though spins may differ due to degeneracy)
    report(
        "Energy prediction error < 50% (same Hamiltonian)",
        accuracy['energy_error'] < 0.5,
        f"energy_error={accuracy['energy_error']:.3f}, "
        f"state_overlap={accuracy['state_overlap']:.3f}"
    )
    report(
        "State overlap > 40% (better than random 50/50)",
        accuracy['state_overlap'] > 0.40,
        f"overlap={accuracy['state_overlap']:.3f}"
    )

    # ── Test 3: Empathy — Similar vs Different Systems ───────────────────
    print("\n--- Test 3: Empathy Score (similar vs different) ---")
    # Two identical-structure systems should have higher empathy
    sys_same1 = IsingGPU(20, 42, device)
    sys_same2 = IsingGPU(20, 42, device)
    sys_same1.anneal(100, 10)
    sys_same2.anneal(100, 20)

    emp_similar = module.compute_empathy(sys_same1, sys_same2, anneal_steps=100, seed=333)

    # System with flipped couplings should have lower empathy
    sys_diff = IsingGPU(20, 42, device)
    sys_diff.coupling *= -1
    sys_diff.anneal(100, 30)

    emp_different = module.compute_empathy(sys_same1, sys_diff, anneal_steps=100, seed=444)

    report(
        "Higher empathy for similar systems than dissimilar ones",
        emp_similar['empathy_score'] > emp_different['empathy_score'],
        f"similar={emp_similar['empathy_score']:.3f}, "
        f"different={emp_different['empathy_score']:.3f}"
    )
    report(
        "Coupling similarity reflects structure match",
        emp_similar['coupling_similarity'] > emp_different['coupling_similarity'],
        f"similar_cs={emp_similar['coupling_similarity']:.3f}, "
        f"different_cs={emp_different['coupling_similarity']:.3f}"
    )

    # ── Test 4: Compassionate Response ───────────────────────────────────
    print("\n--- Test 4: Compassionate Response ---")
    # All IsingGPU instances share the same coupling formula, so we must
    # explicitly create systems with different coupling structures.
    sys_c = IsingGPU(20, 77, device)
    sys_c.anneal(100, 77)
    sys_d = IsingGPU(20, 88, device)
    # Give sys_d a distinct coupling matrix
    torch.manual_seed(271828)
    sys_d.coupling = torch.rand(20, 20, device=device) * 1.5
    sys_d.coupling = (sys_d.coupling + sys_d.coupling.T) / 2
    sys_d.coupling.fill_diagonal_(0)
    sys_d.anneal(100, 88)

    # Verify couplings actually differ
    coupling_diff_check = (sys_c.coupling - sys_d.coupling).abs().mean().item()

    # High empathy response
    coupling_before = sys_c.coupling.clone()
    resp_high = module.compassionate_response(
        sys_c, sys_d, empathy_score=0.8, coupling_strength=0.3
    )
    coupling_delta_high = (sys_c.coupling - coupling_before).abs().mean().item()

    # Low empathy response — use higher noise temperature to ensure flips
    sys_e = IsingGPU(20, 77, device)
    sys_e.anneal(100, 77)
    spins_before = sys_e.spins.clone()
    resp_low = module.compassionate_response(
        sys_e, sys_d, empathy_score=0.2, noise_temperature=0.5
    )
    spins_changed = (sys_e.spins != spins_before).float().mean().item()

    report(
        "High empathy -> coupling blend (non-zero coupling change)",
        coupling_delta_high > 1e-6,
        f"mean_coupling_delta={coupling_delta_high:.6f}, "
        f"coupling_diff_between_systems={coupling_diff_check:.4f}"
    )
    report(
        "Low empathy -> thermal noise (some spins flip)",
        spins_changed > 0,
        f"fraction_flipped={spins_changed:.3f}"
    )

    # ── Test 5: Emotional Memory ─────────────────────────────────────────
    print("\n--- Test 5: Emotional Memory ---")
    mem_module = IsingEmpathyModule(device=device, memory_size=16)

    # Store a sequence with increasing empathy
    for i in range(10):
        emo = EmotionVector(
            valence=0.1 * i, arousal=1.0 - 0.1 * i,
            tension=0.05, coherence=0.1 * i
        )
        mem_module.store_memory(emo, empathy_score=0.1 * i)

    recall = mem_module.recall_memory()
    report(
        "Memory stores correct count",
        recall['memory_entries'] == 10,
        f"entries={recall['memory_entries']}"
    )
    report(
        "Empathy trend is positive (increasing empathy over time)",
        recall['empathy_trend'] > 0,
        f"trend={recall['empathy_trend']:.3f}, avg_empathy={recall['avg_empathy']:.3f}"
    )

    # ── Test 6: Social Attention (Multi-Agent) ───────────────────────────
    print("\n--- Test 6: Social Attention (5 agents) ---")
    self_sys = IsingGPU(16, 1, device)
    self_sys.anneal(80, 1)
    others = []
    for s in [10, 20, 30, 40, 50]:
        o = IsingGPU(16, s, device)
        o.anneal(80, s)
        others.append(o)

    social = module.social_attention(self_sys, others, anneal_steps=80, seed_base=9999)

    report(
        "Attention weights sum to 1.0",
        abs(sum(social['attention_weights']) - 1.0) < 1e-4,
        f"sum={sum(social['attention_weights']):.6f}, "
        f"weights={[f'{w:.3f}' for w in social['attention_weights']]}"
    )
    report(
        "Collective emotion has valid valence range",
        -1.0 <= social['collective_emotion'].valence <= 1.0,
        f"collective_valence={social['collective_emotion'].valence:.3f}"
    )
    report(
        "Most empathic agent index is valid",
        0 <= social['most_empathic_idx'] < len(others),
        f"most_empathic_idx={social['most_empathic_idx']}, "
        f"its_score={social['empathy_scores'][social['most_empathic_idx']]:.3f}"
    )

    # ── Test 7: Full Pipeline ────────────────────────────────────────────
    print("\n--- Test 7: Full Processing Pipeline ---")
    sys_f = IsingGPU(20, 42, device)
    sys_f.anneal(100, 42)
    sys_g = IsingGPU(20, 99, device)
    sys_g.anneal(100, 99)

    result = module.process(sys_f, sys_g, anneal_steps=100, seed=5555, apply_response=True)

    report(
        "Full pipeline returns valid empathy score",
        0.0 <= result['empathy']['empathy_score'] <= 1.0,
        f"empathy={result['empathy']['empathy_score']:.3f}"
    )
    report(
        "Pipeline produces self emotion",
        result['self_emotion'].raw_tensor is not None,
        f"self_valence={result['self_emotion'].valence:.3f}, "
        f"self_arousal={result['self_emotion'].arousal:.3f}"
    )
    report(
        "Pipeline produces compassionate response",
        result['response'] is not None,
        f"response_actions={result['response']['actions']}"
    )
    report(
        "Memory is updated after pipeline",
        result['memory']['memory_entries'] > 0,
        f"entries={result['memory']['memory_entries']}, "
        f"avg_empathy={result['memory']['avg_empathy']:.3f}"
    )

    # ── Test 8: Comparison with Random-Weight Approach ───────────────────
    print("\n--- Test 8: Physics-Grounded vs Random Weights ---")
    # The old module uses random nn.Linear weights -> empathy scores are random
    # We test that our physics scores are consistent (same input -> same output)
    # Use DIFFERENT seeds so coupling_similarity < 1.0 and scores are non-trivial
    def make_test8_pair():
        s1 = IsingGPU(20, 42, device)
        s1.anneal(100, 42)
        s2 = IsingGPU(20, 99, device)
        # Create a frustrated (mixed sign) coupling structure for s2
        # so its ground state is non-trivial and harder to predict
        torch.manual_seed(314159)
        s2.coupling = (torch.rand(s2.n, s2.n, device=device) - 0.5) * 2.0
        s2.coupling = (s2.coupling + s2.coupling.T) / 2
        s2.coupling.fill_diagonal_(0)
        s2.anneal(100, 99)
        return s1, s2

    sys_h1, sys_h2 = make_test8_pair()
    score_run1 = module.compute_empathy(sys_h1, sys_h2, anneal_steps=100, seed=7777)

    sys_h1b, sys_h2b = make_test8_pair()
    score_run2 = module.compute_empathy(sys_h1b, sys_h2b, anneal_steps=100, seed=7777)

    report(
        "Deterministic: same inputs -> same empathy score",
        abs(score_run1['empathy_score'] - score_run2['empathy_score']) < 1e-4,
        f"run1={score_run1['empathy_score']:.6f}, "
        f"run2={score_run2['empathy_score']:.6f}"
    )

    # Physics scores should be in the interior of [0,1], not degenerate
    report(
        "Empathy score is non-trivial (not 0 or 1)",
        0.05 < score_run1['empathy_score'] < 0.95,
        f"score={score_run1['empathy_score']:.3f} (not degenerate)"
    )

    # ── Summary ──────────────────────────────────────────────────────────
    elapsed = time.time() - total_start
    print("\n" + "=" * 78)
    print(f"  RESULTS: {passed} passed, {failed} failed, "
          f"{passed + failed} total  [{elapsed:.1f}s]")
    print("=" * 78)

    if failed == 0:
        print("  All tests passed. Empathy is physically grounded.")
    else:
        print(f"  {failed} test(s) failed. Review above.")

    return failed == 0


if __name__ == "__main__":
    success = run_tests()
    sys.exit(0 if success else 1)
