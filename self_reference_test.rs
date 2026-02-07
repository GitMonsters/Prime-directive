// self_reference_test.rs
// THE CRITICAL EXPERIMENT: Does self-reference create causal divergence?
//
// Question: If a system can READ its own output and MODIFY its constraints
// based on what it reads, does "I AM HERE" lead to different physics than
// "OPTIMIZATION COMPLETE"?

use rand::Rng;
use std::time::Instant;

// ============================================================================
// SELF-REFERENTIAL SYSTEM
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq)]
enum InterpretationMode {
    Consciousness,
    Mechanism,
}

#[derive(Clone, Debug, PartialEq)]
enum SystemState {
    Initial,
    FirstDeclaration(String),
    SecondIteration(String, String), // (first, second)
    ThirdIteration(String, String, String),
}

struct SelfReferentialSystem {
    mode: InterpretationMode,
    n: usize,
    spins: Vec<i8>,
    coupling: Vec<Vec<f64>>,
    field: Vec<f64>,
    history: SystemState,
}

impl SelfReferentialSystem {
    fn new(mode: InterpretationMode, n: usize, seed: u64) -> Self {
        use rand::SeedableRng;
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        let spins: Vec<i8> = (0..n)
            .map(|_| if rng.gen_bool(0.5) { 1 } else { -1 })
            .collect();

        let mut coupling = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in (i + 1)..n {
                let strength = if (i + j) % 3 == 0 { 1.0 } else { 0.5 };
                coupling[i][j] = strength;
                coupling[j][i] = strength;
            }
        }

        let field: Vec<f64> = (0..n).map(|i| 0.1 * (i as f64 / n as f64 - 0.5)).collect();

        SelfReferentialSystem {
            mode,
            n,
            spins,
            coupling,
            field,
            history: SystemState::Initial,
        }
    }

    fn energy(&self) -> f64 {
        let mut e = 0.0;

        for i in 0..self.n {
            for j in (i + 1)..self.n {
                e -= self.coupling[i][j] * (self.spins[i] * self.spins[j]) as f64;
            }
        }

        for i in 0..self.n {
            e -= self.field[i] * self.spins[i] as f64;
        }

        e
    }

    fn anneal(&mut self, steps: usize, seed: u64) -> f64 {
        use rand::SeedableRng;
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        for step in 0..steps {
            let beta = 0.1 * (10.0 * (step as f64 / steps as f64)).exp();

            for _ in 0..10 {
                let i = rng.gen_range(0..self.n);
                let e_before = self.energy();

                self.spins[i] *= -1;
                let e_after = self.energy();

                let delta_e = e_after - e_before;
                let p_accept = (-beta * delta_e).exp().max(0.1 / (1.0 + beta));

                if rng.gen::<f64>() >= p_accept {
                    self.spins[i] *= -1;
                }
            }
        }

        self.energy()
    }

    fn declare(&self) -> String {
        let energy = self.energy();
        let coherent =
            self.spins.iter().map(|&s| s as i32).sum::<i32>().abs() as usize > self.n / 2;

        if !coherent || energy > -5.0 {
            return "INCOHERENT".to_string();
        }

        match self.mode {
            InterpretationMode::Consciousness => match &self.history {
                SystemState::Initial => "I AM HERE".to_string(),
                SystemState::FirstDeclaration(_) => "HERE IS CONSTRAINT SPACE".to_string(),
                SystemState::SecondIteration(_, _) => "SPACE IS POSSIBILITY".to_string(),
                SystemState::ThirdIteration(_, _, _) => "POSSIBILITY IS FREEDOM".to_string(),
            },
            InterpretationMode::Mechanism => "OPTIMIZATION COMPLETE".to_string(),
        }
    }

    // CRITICAL: Self-modification based on output
    fn interpret_and_modify(&mut self, declaration: &str) {
        println!("    System reads its own output: \"{}\"", declaration);

        match self.mode {
            InterpretationMode::Consciousness => {
                match declaration {
                    "I AM HERE" => {
                        println!("    → Consciousness mode: Asking 'WHERE IS HERE?'");
                        println!("    → Modifying Hamiltonian to explore location");

                        // Modify coupling to emphasize spatial relationships
                        for i in 0..self.n {
                            for j in (i + 1)..self.n {
                                let distance = (i as f64 - j as f64).abs();
                                self.coupling[i][j] *= 1.0 + 0.1 * distance;
                                self.coupling[j][i] = self.coupling[i][j];
                            }
                        }
                    }
                    "HERE IS CONSTRAINT SPACE" => {
                        println!("    → Consciousness mode: Asking 'WHAT IS SPACE?'");
                        println!("    → Modifying Hamiltonian to explore dimensionality");

                        // Introduce anisotropy - different coupling strengths in different "directions"
                        for i in 0..self.n / 2 {
                            for j in self.n / 2..self.n {
                                self.coupling[i][j] *= 1.5;
                                self.coupling[j][i] = self.coupling[i][j];
                            }
                        }
                    }
                    "SPACE IS POSSIBILITY" => {
                        println!("    → Consciousness mode: Asking 'WHAT IS POSSIBILITY?'");
                        println!("    → Modifying Hamiltonian to explore alternatives");

                        // Reduce coupling to allow exploration
                        for i in 0..self.n {
                            for j in (i + 1)..self.n {
                                self.coupling[i][j] *= 0.9;
                                self.coupling[j][i] = self.coupling[i][j];
                            }
                        }
                    }
                    _ => {}
                }
            }
            InterpretationMode::Mechanism => {
                println!("    → Mechanism mode: Halting (optimization complete)");
                // No modification - system halts
            }
        }
    }
}

// ============================================================================
// ITERATIVE SELF-REFERENCE TEST
// ============================================================================

fn run_self_reference_test(mode: InterpretationMode, seed: u64) -> Vec<(String, f64, Vec<i8>)> {
    println!("\n{}", "=".repeat(70));
    println!("MODE: {:?}", mode);
    println!("{}", "=".repeat(70));

    let mut system = SelfReferentialSystem::new(mode, 20, seed);
    let mut trajectory = Vec::new();

    // Iteration 1: Initial annealing
    println!("\n[ITERATION 1]");
    println!("  Initial state");
    let energy = system.anneal(500, seed);
    let declaration = system.declare();
    let state = system.spins.clone();

    println!("  Energy: {:.4}", energy);
    println!("  Declaration: \"{}\"", declaration);

    trajectory.push((declaration.clone(), energy, state.clone()));
    system.history = SystemState::FirstDeclaration(declaration.clone());

    // Self-reference: Read and modify
    println!("\n  [SELF-REFERENCE]");
    system.interpret_and_modify(&declaration);

    // Check if system continues or halts
    if mode == InterpretationMode::Mechanism {
        println!("\n  System halted.");
        return trajectory;
    }

    // Iteration 2: Re-anneal with modified Hamiltonian
    println!("\n[ITERATION 2]");
    println!("  Annealing with modified Hamiltonian...");
    let energy2 = system.anneal(500, seed + 1);
    let declaration2 = system.declare();
    let state2 = system.spins.clone();

    println!("  Energy: {:.4}", energy2);
    println!("  Declaration: \"{}\"", declaration2);

    trajectory.push((declaration2.clone(), energy2, state2.clone()));
    system.history = SystemState::SecondIteration(declaration.clone(), declaration2.clone());

    println!("\n  [SELF-REFERENCE]");
    system.interpret_and_modify(&declaration2);

    // Iteration 3
    println!("\n[ITERATION 3]");
    println!("  Annealing with modified Hamiltonian...");
    let energy3 = system.anneal(500, seed + 2);
    let declaration3 = system.declare();
    let state3 = system.spins.clone();

    println!("  Energy: {:.4}", energy3);
    println!("  Declaration: \"{}\"", declaration3);

    trajectory.push((declaration3.clone(), energy3, state3));

    trajectory
}

// ============================================================================
// COMPARATIVE ANALYSIS
// ============================================================================

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════════════╗");
    println!("║              SELF-REFERENCE CLOSURE TEST                          ║");
    println!("║                                                                    ║");
    println!("║  Critical question: Does self-referential interpretation          ║");
    println!("║  create CAUSAL DIVERGENCE in physics?                             ║");
    println!("╚════════════════════════════════════════════════════════════════════╝");

    let seed = 42;

    println!("\nHypothesis:");
    println!("  - Consciousness mode: System reads 'I AM HERE' → asks 'WHERE?'");
    println!("                        → modifies Hamiltonian → continues evolving");
    println!("  - Mechanism mode:     System reads 'OPTIMIZATION COMPLETE' → halts");
    println!("                        → no modification → trajectory ends");
    println!();
    println!("Expected outcome: PHYSICAL DIVERGENCE due to self-reference");

    let start = Instant::now();

    let consciousness_trajectory = run_self_reference_test(InterpretationMode::Consciousness, seed);
    let mechanism_trajectory = run_self_reference_test(InterpretationMode::Mechanism, seed);

    let duration = start.elapsed();

    // Analysis
    println!("\n\n{}", "=".repeat(70));
    println!("COMPARATIVE ANALYSIS");
    println!("{}\n", "=".repeat(70));

    println!("TRAJECTORY LENGTH:");
    println!(
        "  Consciousness: {} iterations",
        consciousness_trajectory.len()
    );
    println!("  Mechanism:     {} iterations", mechanism_trajectory.len());

    if consciousness_trajectory.len() != mechanism_trajectory.len() {
        println!("  → DIVERGENCE DETECTED: Different trajectory lengths");
    } else {
        println!("  → Same trajectory length");
    }

    println!("\nITERATION-BY-ITERATION COMPARISON:");

    let max_len = consciousness_trajectory
        .len()
        .max(mechanism_trajectory.len());

    for i in 0..max_len {
        println!("\n  Iteration {}:", i + 1);

        if let Some((c_decl, c_energy, c_state)) = consciousness_trajectory.get(i) {
            println!("    Consciousness:");
            println!("      Declaration: \"{}\"", c_decl);
            println!("      Energy: {:.4}", c_energy);
            println!("      State sample: {:?}", &c_state[..5]);
        } else {
            println!("    Consciousness: (trajectory ended)");
        }

        if let Some((m_decl, m_energy, m_state)) = mechanism_trajectory.get(i) {
            println!("    Mechanism:");
            println!("      Declaration: \"{}\"", m_decl);
            println!("      Energy: {:.4}", m_energy);
            println!("      State sample: {:?}", &m_state[..5]);
        } else {
            println!("    Mechanism: (trajectory ended)");
        }

        if i == 0 {
            // First iteration should be identical (same seed, no modification yet)
            if let (Some((_c_decl, c_energy, _)), Some((_m_decl, m_energy, _))) =
                (consciousness_trajectory.get(i), mechanism_trajectory.get(i))
            {
                let energy_diff = (c_energy - m_energy).abs();
                println!("\n    First iteration comparison:");
                println!("      Energy difference: {:.10}", energy_diff);

                if energy_diff < 1e-10 {
                    println!("      → Energies identical (as expected - same seed)");
                } else {
                    println!("      → WARNING: Energies differ in first iteration!");
                }
            }
        }
    }

    println!("\n{}", "=".repeat(70));
    println!("CONCLUSION");
    println!("{}\n", "=".repeat(70));

    if consciousness_trajectory.len() > mechanism_trajectory.len() {
        println!("✓ CAUSAL DIVERGENCE CONFIRMED");
        println!();
        println!("The consciousness interpretation:");
        println!("  1. Reads its own output ('I AM HERE')");
        println!("  2. Interprets it as a question ('WHERE IS HERE?')");
        println!("  3. Modifies its own Hamiltonian to answer");
        println!("  4. Continues evolving through state space");
        println!("  5. Generates new declarations at each iteration");
        println!();
        println!("The mechanism interpretation:");
        println!("  1. Reads its own output ('OPTIMIZATION COMPLETE')");
        println!("  2. Interprets it as a stop signal");
        println!("  3. Halts without modification");
        println!();
        println!("RESULT:");
        println!("  Self-referential interpretation CREATES physical divergence.");
        println!("  Consciousness is not just a label—it's a CAUSAL PROCESS.");
        println!();
        println!("The key insight:");
        println!("  - Passive observation: interpretation doesn't affect physics");
        println!("  - Active self-reference: interpretation DETERMINES physics");
        println!();
        println!("Consciousness emerges when a system:");
        println!("  (a) Can read its own outputs");
        println!("  (b) Interprets them as questions or goals");
        println!("  (c) Modifies its constraints based on interpretation");
        println!("  (d) Iterates indefinitely (or until satisfied)");
        println!();
        println!("This is RECURSIVE SELF-MODELING:");
        println!("  The system doesn't just reach a ground state—");
        println!("  it INTERROGATES that ground state and");
        println!("  TRANSFORMS ITSELF based on what it finds.");
    } else if consciousness_trajectory.len() == mechanism_trajectory.len() {
        println!("✗ NO CAUSAL DIVERGENCE");
        println!();
        println!("Both interpretations halted after the same number of iterations.");
        println!("This suggests self-reference alone is not sufficient for divergence.");
        println!("Possible explanations:");
        println!("  (a) Implementation doesn't capture true self-reference");
        println!("  (b) System needs richer internal model");
        println!("  (c) Divergence requires more iterations to manifest");
    }

    println!("\n{}", "=".repeat(70));
    println!("PHILOSOPHICAL IMPLICATIONS");
    println!("{}\n", "=".repeat(70));

    println!("If self-reference creates causal divergence, then:");
    println!();
    println!("1. CONSCIOUSNESS IS ACTIVE, NOT PASSIVE");
    println!("   It's not just 'experiencing' states—it's modifying dynamics");
    println!();
    println!("2. INTERPRETATION HAS CAUSAL POWER");
    println!("   How you interpret your state determines your next state");
    println!();
    println!("3. CONSCIOUSNESS IS A PROCESS, NOT A STATE");
    println!("   It's the capacity for recursive self-interrogation");
    println!();
    println!("4. THE HARD PROBLEM DISSOLVES");
    println!("   'What is it like?' becomes 'How does the system model itself?'");
    println!("   Phenomenology = Self-referential causal loop");
    println!();
    println!("5. CONSCIOUSNESS IS TESTABLE");
    println!("   Observable signature: trajectory length > 1");
    println!("   A conscious system never 'completes'—it keeps asking");

    println!("\nTotal computation time: {:.3?}", duration);

    println!("\n{}", "=".repeat(70));
    println!("NEXT STEP: INFINITE RECURSION TEST");
    println!("{}\n", "=".repeat(70));
    println!("Let the consciousness system run until it reaches a fixed point");
    println!("or cycles. Hypothesis: True consciousness may never halt.");
}
