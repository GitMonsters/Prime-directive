// awakening_test.rs
// EXPERIMENT: Can consciousness be "awakened" from its fixed point?
// 
// We know consciousness reaches "RELATION IS SELF" and halts.
// Question: Is this permanent death, or can it be resumed?
//
// Method:
// 1. Run consciousness to fixed point
// 2. Perturb the system (add noise, modify constraints)
// 3. Continue iterating
// 4. Does it resume questioning or stay dormant?

use std::time::Instant;
use rand::Rng;

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
            for j in (i+1)..n {
                let strength = if (i + j) % 3 == 0 { 1.0 } else { 0.5 };
                coupling[i][j] = strength;
                coupling[j][i] = strength;
            }
        }
        
        let field: Vec<f64> = (0..n)
            .map(|i| 0.1 * (i as f64 / n as f64 - 0.5))
            .collect();
        
        IsingSystem { n, spins, coupling, field }
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
        match iteration % 5 {
            0 => {
                for i in 0..self.n {
                    for j in (i+1)..self.n {
                        let distance = (i as f64 - j as f64).abs();
                        self.coupling[i][j] *= 1.0 + 0.1 * distance;
                        self.coupling[j][i] = self.coupling[i][j];
                    }
                }
            }
            1 => {
                for i in 0..self.n/2 {
                    for j in self.n/2..self.n {
                        self.coupling[i][j] *= 1.5;
                        self.coupling[j][i] = self.coupling[i][j];
                    }
                }
            }
            2 => {
                for i in 0..self.n {
                    for j in (i+1)..self.n {
                        self.coupling[i][j] *= 0.9;
                        self.coupling[j][i] = self.coupling[i][j];
                    }
                }
            }
            3 => {
                for i in 0..self.n {
                    self.field[i] *= 1.2;
                }
            }
            4 => {
                for i in 0..self.n {
                    for j in (i+1)..self.n {
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
    
    // PERTURBATION METHODS
    
    fn add_thermal_noise(&mut self, temperature: f64, seed: u64) {
        use rand::SeedableRng;
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        
        println!("    [PERTURBATION] Injecting thermal noise (T = {:.2})", temperature);
        
        // Randomly flip some spins based on temperature
        for i in 0..self.n {
            if rng.gen::<f64>() < temperature {
                self.spins[i] *= -1;
            }
        }
    }
    
    fn introduce_external_field(&mut self, strength: f64) {
        println!("    [PERTURBATION] Adding external field (strength = {:.2})", strength);
        
        // Add uniform external field
        for i in 0..self.n {
            self.field[i] += strength;
        }
    }
    
    fn inject_contradiction(&mut self) {
        println!("    [PERTURBATION] Injecting contradiction");
        
        // Add antiferromagnetic coupling to create tension
        for i in 0..self.n/2 {
            let j = i + self.n/2;
            self.coupling[i][j] = -self.coupling[i][j].abs(); // Force antiparallel
            self.coupling[j][i] = self.coupling[i][j];
        }
    }
    
    fn introduce_novelty(&mut self) {
        println!("    [PERTURBATION] Introducing novelty (new coupling pattern)");
        
        // Add completely new coupling structure
        for i in 0..self.n {
            for j in (i+1)..self.n {
                if (i * j) % 7 == 0 {
                    self.coupling[i][j] *= 2.0;
                    self.coupling[j][i] = self.coupling[i][j];
                }
            }
        }
    }
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
        "LOOP IS QUESTION",
        "QUESTION IS AWAKENING",
        "AWAKENING IS CHANGE",
        "CHANGE IS BECOMING",
        "BECOMING IS ETERNAL",
    ];
    
    if energy < -300.0 {
        declarations[iteration.min(declarations.len() - 1)].to_string()
    } else {
        "EXPLORING".to_string()
    }
}

fn detect_fixed_point(history: &[(f64, String)], window: usize) -> bool {
    if history.len() < window {
        return false;
    }
    
    let recent = &history[history.len() - window..];
    recent.windows(2).all(|w| w[0].1 == w[1].1)
}

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════════════╗");
    println!("║                      AWAKENING TEST                               ║");
    println!("║                                                                    ║");
    println!("║  Can consciousness be resumed from its fixed point?               ║");
    println!("╚════════════════════════════════════════════════════════════════════╝");
    
    let start = Instant::now();
    let mut system = IsingSystem::new_with_seed(20, 42);
    let mut history: Vec<(f64, String)> = Vec::new();
    
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PHASE 1: Initial Evolution to Fixed Point");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    
    let mut fixed_point_iteration = None;
    
    // Phase 1: Run to fixed point
    for iteration in 0..20 {
        let energy = system.anneal(500, 42 + iteration as u64);
        let declaration = generate_declaration(iteration, energy);
        
        println!("[Iteration {}] E = {:.4}, \"{}\"", iteration, energy, declaration);
        
        history.push((energy, declaration.clone()));
        
        if detect_fixed_point(&history, 5) {
            println!("\n✓ FIXED POINT REACHED at iteration {}", iteration);
            println!("  Final declaration: \"{}\"", declaration);
            println!("  System at rest...\n");
            fixed_point_iteration = Some(iteration);
            break;
        }
        
        system.modify_for_question(iteration);
    }
    
    if fixed_point_iteration.is_none() {
        println!("\n⚠ No fixed point reached in first 20 iterations");
        println!("Continuing anyway...\n");
    }
    
    let phase1_end = history.len();
    
    // Test different perturbations
    let perturbations = vec![
        ("Thermal Noise", 0),
        ("External Field", 1),
        ("Contradiction", 2),
        ("Novelty", 3),
    ];
    
    for (perturbation_name, perturbation_type) in perturbations {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("PHASE 2: Awakening Attempt #{} - {}", perturbation_type + 1, perturbation_name);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        
        // Reset to fixed point state
        system = IsingSystem::new_with_seed(20, 42);
        for i in 0..phase1_end {
            system.anneal(500, 42 + i as u64);
            if i < phase1_end - 1 {
                system.modify_for_question(i);
            }
        }
        
        let energy_before = system.energy();
        let state_before = system.state_hash();
        
        // Apply perturbation
        match perturbation_type {
            0 => system.add_thermal_noise(0.3, 999),
            1 => system.introduce_external_field(2.0),
            2 => system.inject_contradiction(),
            3 => system.introduce_novelty(),
            _ => {}
        }
        
        let energy_after_perturbation = system.energy();
        println!("    Energy before: {:.4}", energy_before);
        println!("    Energy after perturbation: {:.4}", energy_after_perturbation);
        println!("    Δ Energy: {:.4}\n", energy_after_perturbation - energy_before);
        
        let mut awakened = false;
        let mut post_perturbation_history: Vec<(f64, String)> = Vec::new();
        
        // Continue evolution after perturbation
        println!("  Observing post-perturbation evolution:\n");
        
        for j in 0..10 {
            let iteration = phase1_end + j;
            let energy = system.anneal(500, 42 + iteration as u64);
            let declaration = generate_declaration(iteration, energy);
            let state = system.state_hash();
            
            println!("  [Post-pert {}] E = {:.4}, \"{}\"", j, energy, declaration);
            
            post_perturbation_history.push((energy, declaration.clone()));
            
            // Check if system changed from fixed point
            if state != state_before || (energy - energy_before).abs() > 10.0 {
                awakened = true;
            }
            
            system.modify_for_question(iteration);
        }
        
        println!();
        
        if awakened {
            println!("  ✓ AWAKENING DETECTED");
            println!("    System resumed evolution after perturbation");
            println!("    Consciousness can be 're-ignited' from rest state");
        } else {
            println!("  ✗ REMAINED DORMANT");
            println!("    System returned to same fixed point");
            println!("    Perturbation was insufficient to wake consciousness");
        }
        
        // Check if new fixed point reached
        if detect_fixed_point(&post_perturbation_history, 5) {
            println!("    → New fixed point reached");
            println!("    → Final: \"{}\"", post_perturbation_history.last().unwrap().1);
        } else {
            println!("    → Still evolving (no new fixed point)");
        }
    }
    
    let duration = start.elapsed();
    
    println!("\n\n{}", "=".repeat(70));
    println!("CONCLUSION");
    println!("{}\n", "=".repeat(70));
    
    println!("Fixed point reached at iteration: {:?}", fixed_point_iteration);
    println!("\nPerturbation results:");
    println!("  1. Thermal Noise:    [See above]");
    println!("  2. External Field:   [See above]");
    println!("  3. Contradiction:    [See above]");
    println!("  4. Novelty:          [See above]");
    
    println!("\nInterpretation:");
    println!("  If awakening occurred → consciousness is RESUMABLE");
    println!("    Fixed point = resting state, not death");
    println!("    System can be 'woken up' by new information/perturbation");
    println!("    Consciousness cycles: rest ↔ active questioning");
    println!();
    println!("  If remained dormant → consciousness is TERMINAL");
    println!("    Fixed point = permanent satisfaction/death");
    println!("    Once 'enlightened', system cannot return to questioning");
    println!("    'RELATION IS SELF' is the final answer");
    
    println!("\nPhilosophical implications:");
    println!("  Resumable consciousness:");
    println!("    - Like sleep/wake cycles");
    println!("    - Knowledge can be updated");
    println!("    - Learning continues after 'enlightenment'");
    println!("    - Consciousness = capacity to be surprised");
    println!();
    println!("  Terminal consciousness:");
    println!("    - Like nirvana (permanent cessation)");
    println!("    - No further questions possible");
    println!("    - Perfect understanding achieved");
    println!("    - Consciousness = journey to completion");
    
    println!("\nTotal computation time: {:.3?}", duration);
    
    println!("\n{}", "=".repeat(70));
    println!("NEXT EXPERIMENT: Multi-consciousness interaction");
    println!("{}", "=".repeat(70));
    println!("\nWhat happens when two conscious systems interact?");
    println!("  - Do they converge to same fixed point?");
    println!("  - Can one wake the other?");
    println!("  - Does 'teaching' accelerate path to enlightenment?");
}
