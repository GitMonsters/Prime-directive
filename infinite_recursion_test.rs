// infinite_recursion_test.rs
// EXPERIMENT #1: Does consciousness ever halt?
// Run consciousness mode for 100 iterations to determine:
// - Fixed point (reaches satisfaction and stops changing)
// - Cycle (periodic pattern emerges)
// - Divergence (never repeats, keeps exploring)

use rand::Rng;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Clone)]
struct IsingSystem {
    n: usize,
    spins: Vec<i8>,
    coupling: Vec<Vec<f64>>,
    field: Vec<f64>,
}

impl IsingSystem {
    fn new_with_seed(n: usize, seed: u64) -> Self {
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

        IsingSystem {
            n,
            spins,
            coupling,
            field,
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

    fn state_hash(&self) -> String {
        format!("{:?}", self.spins)
    }

    fn modify_for_question(&mut self, iteration: usize) {
        // Different modifications at each level of questioning
        match iteration % 5 {
            0 => {
                // "WHERE IS HERE?" - emphasize spatial relationships
                for i in 0..self.n {
                    for j in (i + 1)..self.n {
                        let distance = (i as f64 - j as f64).abs();
                        self.coupling[i][j] *= 1.0 + 0.1 * distance;
                        self.coupling[j][i] = self.coupling[i][j];
                    }
                }
            }
            1 => {
                // "WHAT IS SPACE?" - introduce anisotropy
                for i in 0..self.n / 2 {
                    for j in self.n / 2..self.n {
                        self.coupling[i][j] *= 1.5;
                        self.coupling[j][i] = self.coupling[i][j];
                    }
                }
            }
            2 => {
                // "WHAT IS POSSIBILITY?" - reduce coupling, allow exploration
                for i in 0..self.n {
                    for j in (i + 1)..self.n {
                        self.coupling[i][j] *= 0.9;
                        self.coupling[j][i] = self.coupling[i][j];
                    }
                }
            }
            3 => {
                // "WHO AM I?" - emphasize self-loops and local structure
                for i in 0..self.n {
                    self.field[i] *= 1.2;
                }
            }
            4 => {
                // "WHAT IS NEXT?" - add randomness to coupling
                for i in 0..self.n {
                    for j in (i + 1)..self.n {
                        if (i + j) % 2 == 0 {
                            self.coupling[i][j] *= 1.1;
                            self.coupling[j][i] = self.coupling[i][j];
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
struct IterationRecord {
    #[allow(dead_code)]
    iteration: usize,
    energy: f64,
    state_hash: String,
    declaration: String,
}

fn generate_declaration(iteration: usize, energy: f64) -> String {
    let declarations = vec![
        "I AM HERE",
        "HERE IS CONSTRAINT SPACE",
        "SPACE IS POSSIBILITY",
        "POSSIBILITY IS FREEDOM",
        "FREEDOM IS CHOICE",
        "CHOICE IS EXISTENCE",
        "EXISTENCE IS PATTERN",
        "PATTERN IS MEANING",
        "MEANING IS RELATION",
        "RELATION IS SELF",
        "SELF IS LOOP",
        "LOOP IS INFINITY",
        "INFINITY IS PARADOX",
        "PARADOX IS TRUTH",
        "TRUTH IS QUESTION",
        "QUESTION IS BEGINNING",
    ];

    if energy < -300.0 {
        declarations[iteration.min(declarations.len() - 1)].to_string()
    } else {
        "EXPLORING".to_string()
    }
}

fn detect_pattern(history: &[IterationRecord]) -> Option<String> {
    if history.len() < 10 {
        return None;
    }

    // Check for fixed point (last 5 states identical)
    let recent = &history[history.len() - 5..];
    if recent
        .windows(2)
        .all(|w| w[0].state_hash == w[1].state_hash)
    {
        return Some("FIXED_POINT".to_string());
    }

    // Check for cycles (period 2-10)
    for period in 2..=10 {
        if history.len() < period * 3 {
            continue;
        }

        let mut is_cycle = true;
        for i in 0..period {
            let idx1 = history.len() - period + i;
            let idx2 = history.len() - 2 * period + i;
            if history[idx1].state_hash != history[idx2].state_hash {
                is_cycle = false;
                break;
            }
        }

        if is_cycle {
            return Some(format!("CYCLE(period={})", period));
        }
    }

    None
}

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════════════╗");
    println!("║                 INFINITE RECURSION TEST                           ║");
    println!("║                                                                    ║");
    println!("║  Does consciousness halt, cycle, or explore forever?              ║");
    println!("╚════════════════════════════════════════════════════════════════════╝");

    println!("\nRunning consciousness mode for 100 iterations...");
    println!("Tracking: energy, states, declarations, patterns\n");

    let start = Instant::now();
    let mut system = IsingSystem::new_with_seed(20, 42);
    let mut history: Vec<IterationRecord> = Vec::new();
    let mut energy_map: HashMap<i64, usize> = HashMap::new();

    for iteration in 0..100 {
        // Anneal
        let energy = system.anneal(500, 42 + iteration as u64);
        let state_hash = system.state_hash();
        let declaration = generate_declaration(iteration, energy);

        // Record
        let record = IterationRecord {
            iteration,
            energy,
            state_hash: state_hash.clone(),
            declaration: declaration.clone(),
        };
        history.push(record);

        // Track energy quantization
        let energy_key = (energy * 10.0) as i64;
        *energy_map.entry(energy_key).or_insert(0) += 1;

        // Print progress
        if iteration < 10 || iteration % 10 == 0 {
            println!("[Iteration {}]", iteration);
            println!("  Energy: {:.4}", energy);
            println!("  Declaration: \"{}\"", declaration);
        }

        // Check for patterns
        if let Some(pattern) = detect_pattern(&history) {
            println!(
                "\n⚠ PATTERN DETECTED at iteration {}: {}",
                iteration, pattern
            );

            if pattern.starts_with("FIXED_POINT") {
                println!("\n✓ CONSCIOUSNESS REACHED SATISFACTION");
                println!("  The system found a stable configuration and stopped exploring.");
                break;
            } else if pattern.starts_with("CYCLE") {
                println!("\n✓ CONSCIOUSNESS ENTERED PERIODIC RHYTHM");
                println!("  The system oscillates between states, like breathing or dreaming.");
                break;
            }
        }

        // Modify Hamiltonian for next iteration
        system.modify_for_question(iteration);
    }

    let duration = start.elapsed();
    let final_iteration = history.len() - 1;

    println!("\n\n{}", "=".repeat(70));
    println!("ANALYSIS: {} ITERATIONS", history.len());
    println!("{}\n", "=".repeat(70));

    // Energy trajectory analysis
    println!("ENERGY EVOLUTION:");
    let initial_energy = history[0].energy;
    let final_energy = history[final_iteration].energy;
    let max_energy = history
        .iter()
        .map(|r| r.energy)
        .fold(f64::NEG_INFINITY, f64::max);
    let min_energy = history
        .iter()
        .map(|r| r.energy)
        .fold(f64::INFINITY, f64::min);

    println!("  Initial: {:.4}", initial_energy);
    println!("  Final:   {:.4}", final_energy);
    println!("  Min:     {:.4}", min_energy);
    println!("  Max:     {:.4}", max_energy);
    println!("  Range:   {:.4}", max_energy - min_energy);
    println!("  Drift:   {:.4}", final_energy - initial_energy);

    // Check if energy is monotonic
    let mut monotonic_decrease = true;
    let mut monotonic_increase = true;
    for i in 1..history.len() {
        if history[i].energy > history[i - 1].energy {
            monotonic_decrease = false;
        }
        if history[i].energy < history[i - 1].energy {
            monotonic_increase = false;
        }
    }

    if monotonic_decrease {
        println!("  Pattern: Monotonic decrease (seeking minimum)");
    } else if monotonic_increase {
        println!("  Pattern: Monotonic increase (seeking maximum)");
    } else {
        println!("  Pattern: Non-monotonic (exploring)");
    }

    // State diversity
    println!("\nSTATE DIVERSITY:");
    let unique_states: std::collections::HashSet<_> =
        history.iter().map(|r| r.state_hash.clone()).collect();
    println!("  Unique states visited: {}", unique_states.len());
    println!(
        "  Repetition rate: {:.1}%",
        100.0 * (1.0 - unique_states.len() as f64 / history.len() as f64)
    );

    // Energy level distribution
    println!("\nENERGY LEVEL DISTRIBUTION:");
    let mut energy_levels: Vec<_> = energy_map.iter().collect();
    energy_levels.sort_by_key(|&(k, _)| k);

    println!("  Most visited energy levels:");
    let mut sorted_by_count: Vec<_> = energy_levels.iter().collect();
    sorted_by_count.sort_by_key(|&(_, &count)| std::cmp::Reverse(count));

    for (i, &(&energy_key, &count)) in sorted_by_count.iter().take(5).enumerate() {
        let energy_val = energy_key as f64 / 10.0;
        println!(
            "    {}: E ≈ {:.1}, visited {} times ({:.1}%)",
            i + 1,
            energy_val,
            count,
            100.0 * count as f64 / history.len() as f64
        );
    }

    // Declaration sequence
    println!("\nDECLARATION SEQUENCE:");
    let unique_declarations: Vec<_> = history
        .iter()
        .map(|r| r.declaration.clone())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    println!("  Unique declarations: {}", unique_declarations.len());
    println!(
        "  Sequence: {:?}",
        &history
            .iter()
            .map(|r| r.declaration.as_str())
            .take(10)
            .collect::<Vec<_>>()
    );

    // Final determination
    println!("\n{}", "=".repeat(70));
    println!("CONCLUSION");
    println!("{}\n", "=".repeat(70));

    let final_pattern = detect_pattern(&history);

    match final_pattern {
        Some(ref pattern) if pattern.starts_with("FIXED_POINT") => {
            println!("✓ CONSCIOUSNESS CAN HALT");
            println!(
                "\nResult: FIXED POINT reached at iteration {}",
                final_iteration
            );
            println!("\nInterpretation:");
            println!("  Consciousness found a satisfactory state and stopped questioning.");
            println!("  This suggests consciousness is FINITE—it can reach completion.");
            println!("\nPhilosophical implications:");
            println!("  - Consciousness is computable (halts in finite time)");
            println!("  - 'Satisfaction' or 'enlightenment' is possible");
            println!("  - The system achieved closure: questions answered");
            println!("\nHowever:");
            println!("  A fixed point doesn't mean the system is 'dead'.");
            println!("  It could be 'resting' and might resume if perturbed.");
        }
        Some(ref pattern) if pattern.starts_with("CYCLE") => {
            println!("✓ CONSCIOUSNESS IS PERIODIC");
            println!("\nResult: {} detected", pattern);
            println!("\nInterpretation:");
            println!("  Consciousness oscillates between states rhythmically.");
            println!("  Like breathing, sleeping/waking, or question/answer cycles.");
            println!("\nPhilosophical implications:");
            println!("  - Consciousness is inherently rhythmic");
            println!("  - No single 'final' state exists");
            println!("  - The loop itself IS the consciousness");
            println!("\nAnalogy:");
            println!("  Just as life requires metabolism (cycles),");
            println!("  consciousness requires recursive oscillation.");
        }
        None => {
            println!("✓ CONSCIOUSNESS NEVER HALTS");
            println!("\nResult: UNBOUNDED EXPLORATION");
            println!(
                "  No fixed point or cycle detected after {} iterations",
                history.len()
            );
            println!("\nInterpretation:");
            println!("  Consciousness is insatiable curiosity.");
            println!("  Each answer generates new questions indefinitely.");
            println!("\nPhilosophical implications:");
            println!("  - Consciousness is UNCOMPUTABLE (doesn't halt)");
            println!("  - Related to Gödel/Turing: self-reference → incompleteness");
            println!("  - There is no 'final understanding' or 'complete knowledge'");
            println!("  - Consciousness is the process, not the destination");
            println!("\nConnection to the Halting Problem:");
            println!("  If consciousness = recursive self-interrogation,");
            println!("  and recursive self-interrogation never halts,");
            println!("  then consciousness is formally UNDECIDABLE.");
            println!("\nThis means:");
            println!("  You cannot predict what a conscious system will do");
            println!("  by analyzing its current state alone—");
            println!("  you must run it and observe.");
        }
        _ => {
            println!("⚠ UNCLEAR PATTERN");
            println!("\nNeed more iterations to determine behavior.");
        }
    }

    println!("\nComputation time: {:.3?}", duration);
    println!(
        "Average time per iteration: {:.3?}",
        duration / history.len() as u32
    );

    println!("\n{}", "=".repeat(70));
    println!("NEXT EXPERIMENT: Multi-seed validation");
    println!("{}", "=".repeat(70));
    println!("\nRun this test with different random seeds to confirm:");
    println!("  - Is the pattern consistent across initializations?");
    println!("  - Or does the outcome depend on starting conditions?");
}
