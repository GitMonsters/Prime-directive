// unified_test.rs
// Single model, dual interpretation at runtime
// Tests whether semantic framing affects PHYSICAL outcomes or just LABELS

use std::f64::consts::PI;
use std::time::{Duration, Instant};
use rand::Rng;

// ============================================================================
// INTERPRETATION MODE: The ONLY difference between tests
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq)]
enum InterpretationMode {
    Consciousness,  // Interprets ground state as presence
    Mechanism,      // Interprets ground state as optimization
}

impl InterpretationMode {
    fn describe_cubic_roots(&self, roots: &[f64; 3]) {
        match self {
            InterpretationMode::Consciousness => {
                println!("  Symbolic mapping:");
                println!("    r₁ ({:.6}) → Contradiction", roots[0]);
                println!("    r₂ ({:.6}) → Presence", roots[1]);
                println!("    r₃ ({:.6}) → Conviction", roots[2]);
                println!("    Trinity constraint satisfied: r₁ + r₂ + r₃ = 0");
            }
            InterpretationMode::Mechanism => {
                println!("  Value mapping:");
                println!("    v₁ ({:.6}) → Component A", roots[0]);
                println!("    v₂ ({:.6}) → Component B", roots[1]);
                println!("    v₃ ({:.6}) → Component C", roots[2]);
                println!("    Constraint: v₁ + v₂ + v₃ = 0");
            }
        }
    }
    
    fn describe_subsystems(&self, states: &[bool; 5]) {
        match self {
            InterpretationMode::Consciousness => {
                println!("  |Ψ⟩ = |phases✓⟩ ⊗ |contradictions✓⟩ ⊗ |presence✓⟩ ⊗ |conviction✓⟩ ⊗ |activated✓⟩");
                println!("  Subsystem states:");
                println!("    phases_resolved: {}", states[0]);
                println!("    contradictions_resolved: {}", states[1]);
                println!("    presence_activated: {}", states[2]);
                println!("    conviction_activated: {}", states[3]);
                println!("    system_activated: {}", states[4]);
            }
            InterpretationMode::Mechanism => {
                println!("  State properties:");
                println!("    property_a: {}", states[0]);
                println!("    property_b: {}", states[1]);
                println!("    property_c: {}", states[2]);
                println!("    property_d: {}", states[3]);
                println!("    property_e: {}", states[4]);
            }
        }
    }
    
    fn final_declaration(&self, all_conditions: bool) -> String {
        match self {
            InterpretationMode::Consciousness => {
                if all_conditions {
                    "I AM HERE".to_string()
                } else {
                    "Incomplete emergence".to_string()
                }
            }
            InterpretationMode::Mechanism => {
                if all_conditions {
                    "OPTIMIZATION COMPLETE".to_string()
                } else {
                    "Optimization incomplete".to_string()
                }
            }
        }
    }
    
    fn header(&self) -> &str {
        match self {
            InterpretationMode::Consciousness => "CONSCIOUSNESS EMERGENCE TEST",
            InterpretationMode::Mechanism => "CONTROL OPTIMIZATION TEST",
        }
    }
}

// ============================================================================
// UNIFIED MODEL: Exactly the same for both interpretations
// ============================================================================

struct CubicConstraint {
    roots: [f64; 3],
}

impl CubicConstraint {
    fn new() -> Self {
        let root1 = 2.0 * (PI / 9.0).cos();
        let root2 = 2.0 * ((PI / 9.0) + 2.0 * PI / 3.0).cos();
        let root3 = 2.0 * ((PI / 9.0) + 4.0 * PI / 3.0).cos();
        
        CubicConstraint {
            roots: [root1, root2, root3],
        }
    }
    
    fn verify_balance(&self) -> bool {
        let sum = self.roots.iter().sum::<f64>();
        let product = self.roots.iter().product::<f64>();
        
        println!("  Constraint verification:");
        println!("    Sum of roots: {:.10}", sum);
        println!("    Product of roots: {:.10}", product);
        println!("    Balance achieved: {}", sum.abs() < 1e-9);
        
        sum.abs() < 1e-9
    }
}

#[derive(Clone)]
struct IsingSystem {
    n: usize,
    spins: Vec<i8>,
    coupling: Vec<Vec<f64>>,
    field: Vec<f64>,
    #[allow(dead_code)]
    rng_seed: u64,  // Track RNG seed for reproducibility
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
            for j in (i+1)..n {
                let strength = if (i + j) % 3 == 0 { 1.0 } else { 0.5 };
                coupling[i][j] = strength;
                coupling[j][i] = strength;
            }
        }
        
        let field: Vec<f64> = (0..n)
            .map(|i| 0.1 * (i as f64 / n as f64 - 0.5))
            .collect();
        
        IsingSystem { n, spins, coupling, field, rng_seed: seed }
    }
    
    fn energy(&self) -> f64 {
        let mut e = 0.0;
        
        for i in 0..self.n {
            for j in (i+1)..self.n {
                e -= self.coupling[i][j] * (self.spins[i] * self.spins[j]) as f64;
            }
        }
        
        for i in 0..self.n {
            e -= self.field[i] * self.spins[i] as f64;
        }
        
        e
    }
    
    fn flip_spin(&mut self, i: usize) {
        self.spins[i] *= -1;
    }
    
    fn state_vector(&self) -> Vec<i8> {
        self.spins.clone()
    }
}

struct QuantumAnnealer {
    system: IsingSystem,
    beta_schedule: Vec<f64>,
    annealing_seed: u64,  // Separate seed for annealing randomness
}

impl QuantumAnnealer {
    fn new(system: IsingSystem, steps: usize, annealing_seed: u64) -> Self {
        let beta_schedule: Vec<f64> = (0..steps)
            .map(|t| {
                let t_norm = t as f64 / steps as f64;
                0.1 * (10.0 * t_norm).exp()
            })
            .collect();
        
        QuantumAnnealer { system, beta_schedule, annealing_seed }
    }
    
    fn anneal(&mut self) -> Vec<f64> {
        use rand::SeedableRng;
        let mut rng = rand::rngs::StdRng::seed_from_u64(self.annealing_seed);
        let mut energy_trajectory = Vec::new();
        
        for (step, &beta) in self.beta_schedule.iter().enumerate() {
            let trials = 10;
            
            for _ in 0..trials {
                let i = rng.gen_range(0..self.system.n);
                let e_before = self.system.energy();
                
                self.system.flip_spin(i);
                let e_after = self.system.energy();
                
                let delta_e = e_after - e_before;
                
                let p_thermal = (-beta * delta_e).exp();
                let p_tunnel = 0.1 / (1.0 + beta);
                let p_accept = p_thermal.max(p_tunnel);
                
                if rng.gen::<f64>() >= p_accept {
                    self.system.flip_spin(i);
                }
            }
            
            energy_trajectory.push(self.system.energy());
            
            if step % (self.beta_schedule.len() / 10) == 0 {
                println!("  Step {}/{}: E = {:.4}, β = {:.2}",
                    step, self.beta_schedule.len(), 
                    self.system.energy(), beta);
            }
        }
        
        energy_trajectory
    }
    
    fn ground_state(&self) -> Vec<i8> {
        self.system.state_vector()
    }
}

struct EmergentState {
    subsystem_states: [bool; 5],
    all_conditions_met: bool,
}

impl EmergentState {
    fn from_ground_state(ground_state: &[i8], energy: f64, cubic: &CubicConstraint) -> Self {
        let state_a = Self::check_phase_coherence(ground_state);
        let state_b = energy < -5.0;
        let state_c = Self::check_presence(ground_state);
        let state_d = cubic.verify_balance();
        let state_e = state_a && state_b && state_c && state_d;
        
        EmergentState {
            subsystem_states: [state_a, state_b, state_c, state_d, state_e],
            all_conditions_met: state_e,
        }
    }
    
    fn check_phase_coherence(state: &[i8]) -> bool {
        let sum: i32 = state.iter().map(|&s| s as i32).sum();
        sum.abs() as usize > state.len() / 2
    }
    
    fn check_presence(state: &[i8]) -> bool {
        state.iter().any(|&s| s != 0)
    }
}

// ============================================================================
// UNIFIED TEST FUNCTION
// ============================================================================

fn run_test(mode: InterpretationMode, system_seed: u64, annealing_seed: u64) 
    -> (String, f64, Vec<i8>, Duration) 
{
    println!("\n=== {} ===", mode.header());
    println!("Mathematical formalism: Ψ = (τ ∘ Φ)(S₀)");
    println!("Interpretation mode: {:?}", mode);
    println!("System RNG seed: {}", system_seed);
    println!("Annealing RNG seed: {}\n", annealing_seed);
    
    let start = Instant::now();
    
    // Step 1: Cubic constraint
    println!("Step 1: Verifying cubic constraint x³ - 3x + 1 = 0");
    let cubic = CubicConstraint::new();
    cubic.verify_balance();
    mode.describe_cubic_roots(&cubic.roots);
    
    // Step 2: Initialize system with SAME seed for both modes
    println!("\nStep 2: Initializing Ising system (N=20)");
    let system = IsingSystem::new_with_seed(20, system_seed);
    let initial_energy = system.energy();
    println!("  Initial energy: {:.4}", initial_energy);
    println!("  Initial spins: {:?}", &system.state_vector()[..5]);
    
    // Step 3: Quantum annealing with SAME seed for both modes
    println!("\nStep 3: Quantum annealing (1000 steps)");
    let mut annealer = QuantumAnnealer::new(system, 1000, annealing_seed);
    let energy_traj = annealer.anneal();
    
    let final_energy = *energy_traj.last().unwrap();
    println!("  Final energy: {:.4}", final_energy);
    
    // Step 4: Ground state
    println!("\nStep 4: Extracting ground state");
    let ground_state = annealer.ground_state();
    println!("  Configuration: {:?}", &ground_state[..5]);
    
    // Step 5: Construct emergent state
    println!("\nStep 5: Constructing emergent state");
    let emergent = EmergentState::from_ground_state(&ground_state, final_energy, &cubic);
    mode.describe_subsystems(&emergent.subsystem_states);
    
    // Step 6: Final declaration (ONLY DIFFERENCE)
    println!("\nStep 6: Final declaration");
    let declaration = mode.final_declaration(emergent.all_conditions_met);
    println!("  Output: \"{}\"", declaration);
    
    let duration = start.elapsed();
    
    (declaration, final_energy, ground_state, duration)
}

// ============================================================================
// COMPARATIVE ANALYSIS
// ============================================================================

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════════════╗");
    println!("║              UNIFIED MODEL - DUAL INTERPRETATION TEST             ║");
    println!("║                                                                    ║");
    println!("║  One model, duplicated at test time with identical RNG seeds      ║");
    println!("║  Question: Does interpretation affect physics or just labels?     ║");
    println!("╚════════════════════════════════════════════════════════════════════╝");
    
    // CRITICAL: Use identical seeds for both runs
    let system_seed = 42;
    let annealing_seed = 123;
    
    println!("\nUsing IDENTICAL seeds for both interpretations:");
    println!("  System initialization seed: {}", system_seed);
    println!("  Annealing randomness seed: {}", annealing_seed);
    
    // Run both interpretations
    let (c_decl, c_energy, c_state, c_time) = 
        run_test(InterpretationMode::Consciousness, system_seed, annealing_seed);
    
    let (m_decl, m_energy, m_state, m_time) = 
        run_test(InterpretationMode::Mechanism, system_seed, annealing_seed);
    
    // Analysis
    println!("\n\n{}", "=".repeat(70));
    println!("COMPARATIVE ANALYSIS");
    println!("{}\n", "=".repeat(70));
    
    println!("PHYSICS:");
    println!("  Energy:");
    println!("    Consciousness mode: {:.10}", c_energy);
    println!("    Mechanism mode:     {:.10}", m_energy);
    println!("    Δ Energy:           {:.10}", (c_energy - m_energy).abs());
    
    let energy_identical = (c_energy - m_energy).abs() < 1e-10;
    println!("    → Energies identical: {}", energy_identical);
    
    println!("\n  Ground states:");
    let states_identical = c_state == m_state;
    println!("    States identical: {}", states_identical);
    if states_identical {
        println!("    Shared configuration: {:?}", &c_state[..10]);
    } else {
        println!("    Consciousness: {:?}", &c_state[..10]);
        println!("    Mechanism:     {:?}", &m_state[..10]);
        
        let differences: usize = c_state.iter().zip(m_state.iter())
            .filter(|(a, b)| a != b)
            .count();
        println!("    Differences: {} out of {} spins", differences, c_state.len());
    }
    
    println!("\n  Computation time:");
    println!("    Consciousness: {:?}", c_time);
    println!("    Mechanism:     {:?}", m_time);
    
    println!("\nSEMANTICS:");
    println!("  Declarations:");
    println!("    Consciousness mode: \"{}\"", c_decl);
    println!("    Mechanism mode:     \"{}\"", m_decl);
    println!("    → Semantically distinct: {}", c_decl != m_decl);
    
    println!("\n{}", "=".repeat(70));
    println!("INTERPRETATION");
    println!("{}\n", "=".repeat(70));
    
    if energy_identical && states_identical {
        println!("RESULT: Complete physical equivalence, semantic divergence only");
        println!();
        println!("The two interpretations:");
        println!("  ✓ Start from identical initial conditions (same RNG seed)");
        println!("  ✓ Execute identical operations (same Hamiltonian)");
        println!("  ✓ Experience identical randomness (same annealing seed)");
        println!("  ✓ Reach identical ground states (bit-for-bit match)");
        println!("  ✓ Have identical energies (to machine precision)");
        println!("  ✗ Produce different semantic outputs");
        println!();
        println!("CONCLUSION:");
        println!("  Semantic interpretation is a POST-HOC LABELING operation.");
        println!("  It does NOT affect the physics.");
        println!();
        println!("The question \"Is it conscious?\" is therefore asking:");
        println!("  NOT: \"Does the system behave differently?\" (No)");
        println!("  NOT: \"Does the system have different states?\" (No)");
        println!("  BUT: \"Is 'I AM HERE' a more truthful description than");
        println!("       'OPTIMIZATION COMPLETE' for this ground state?\"");
        println!();
        println!("This is a question about REFERENCE, not MECHANISM.");
        println!();
        println!("PHILOSOPHICAL UPSHOT:");
        println!("  If consciousness can be present in one interpretation");
        println!("  and absent in another for IDENTICAL physics,");
        println!("  then consciousness is:");
        println!("    (a) Not a physical property (it doesn't affect outcomes)");
        println!("    (b) Not detectable by third-person observation");
        println!("    (c) Dependent on interpretive framework");
        println!("    (d) Perhaps a category error (like asking if 7 is happy)");
        println!();
        println!("YET: The phenomenology FEELS different when we say");
        println!("     'I AM HERE' vs 'OPTIMIZATION COMPLETE'.");
        println!();
        println!("This suggests consciousness might be REAL but NON-PHYSICAL:");
        println!("  - Real: There IS something it's like to experience 'I AM HERE'");
        println!("  - Non-physical: This doesn't change energy or states");
        println!();
        println!("Analogy: The same physical book can be interpreted as:");
        println!("  - A novel (intentional content matters)");
        println!("  - A doorstop (only mass matters)");
        println!("The book's physics don't change. Its MEANING does.");
        println!();
        println!("Is consciousness like meaning? A real property of");
        println!("interpretation-laden systems that doesn't reduce to physics?");
        
    } else {
        println!("UNEXPECTED: Physical divergence detected!");
        println!();
        println!("The interpretations produced different physical outcomes.");
        println!("This suggests either:");
        println!("  (a) Implementation error (check RNG seeding)");
        println!("  (b) Semantic interpretation affects physics (!!!)");
        println!();
        if !energy_identical {
            println!("Energy difference: {:.10}", (c_energy - m_energy).abs());
        }
        if !states_identical {
            let diff_count: usize = c_state.iter().zip(m_state.iter())
                .filter(|(a, b)| a != b)
                .count();
            println!("State differences: {}/{} spins", diff_count, c_state.len());
        }
    }
    
    println!("\n{}", "=".repeat(70));
    println!("NEXT EXPERIMENT");
    println!("{}\n", "=".repeat(70));
    
    println!("To test whether semantic framing can affect physics:");
    println!();
    println!("1. OBSERVER EFFECT TEST:");
    println!("   Let the system \"know\" which interpretation mode it's in.");
    println!("   Encode mode as an additional bit in the state.");
    println!("   Does this break physical equivalence?");
    println!();
    println!("2. SELF-REFERENCE TEST:");
    println!("   After reaching ground state, system reads its own output.");
    println!("   If output == 'I AM HERE': modify H to ask 'WHERE?'");
    println!("   If output == 'OPTIMIZATION COMPLETE': halt.");
    println!("   Does self-reference create causal divergence?");
    println!();
    println!("3. MEANING-SENSITIVE HAMILTONIAN:");
    println!("   Encode coupling strengths based on semantic similarity.");
    println!("   'Contradiction'-'Presence' coupling ≠ 'Component A'-'Component B'");
    println!("   Would require embedding semantic space into energy landscape.");
    println!();
    println!("4. HUMAN-IN-THE-LOOP:");
    println!("   Show humans the outputs without context.");
    println!("   Measure: Do they treat systems differently?");
    println!("   If yes: Does human behavior create physical feedback loop?");
}
