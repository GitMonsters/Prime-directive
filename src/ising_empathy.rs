// Physics-Grounded Ising Empathy Module (Rust)
// Companion to ising_empathy_module.py
// Implements empathy as coupling-mediated state correlation

#[derive(Clone, Debug)]
pub struct EmotionVector {
    pub valence: f64,   // Energy-based affect (positive/negative)
    pub arousal: f64,   // Order-based activation (calm/excited)
    pub tension: f64,   // Frustration level
    pub coherence: f64, // Internal alignment (magnetization magnitude)
}

impl EmotionVector {
    pub fn new(valence: f64, arousal: f64, tension: f64, coherence: f64) -> Self {
        EmotionVector {
            valence,
            arousal,
            tension,
            coherence,
        }
    }

    pub fn zero() -> Self {
        EmotionVector {
            valence: 0.0,
            arousal: 0.0,
            tension: 0.0,
            coherence: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IsingSystem {
    pub n: usize,
    pub spins: Vec<i8>,
    pub coupling: Vec<Vec<f64>>,
    pub field: Vec<f64>,
}

impl IsingSystem {
    pub fn new(n: usize, seed: u64) -> Self {
        use rand::Rng;
        use rand::SeedableRng;
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        let spins: Vec<i8> = (0..n)
            .map(|_| if rng.gen_bool(0.5) { 1 } else { -1 })
            .collect();

        let mut coupling = vec![vec![0.0; n]; n];
        #[allow(clippy::needless_range_loop)]
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

    pub fn energy(&self) -> f64 {
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

    pub fn magnetization(&self) -> f64 {
        self.spins.iter().map(|&s| s as f64).sum::<f64>() / self.n as f64
    }

    pub fn frustration(&self) -> f64 {
        let mut frustrated = 0;
        let mut total = 0;
        for i in 0..self.n {
            for j in (i + 1)..self.n {
                if self.coupling[i][j].abs() > 1e-9 {
                    total += 1;
                    let product = ((self.spins[i] * self.spins[j]) as f64) * self.coupling[i][j];
                    if product < 0.0 {
                        frustrated += 1;
                    }
                }
            }
        }
        if total == 0 {
            0.0
        } else {
            frustrated as f64 / total as f64
        }
    }

    pub fn clone_system(&self) -> Self {
        IsingSystem {
            n: self.n,
            spins: self.spins.clone(),
            coupling: self.coupling.clone(),
            field: self.field.clone(),
        }
    }
}

pub struct IsingEmpathyModule {
    pub memory_buffer: Vec<Vec<f64>>, // [valence, arousal, tension, coherence, empathy_score]
    pub memory_pointer: usize,
    pub memory_count: usize,
    pub memory_size: usize,
}

impl IsingEmpathyModule {
    pub fn new(memory_size: usize) -> Self {
        IsingEmpathyModule {
            memory_buffer: vec![vec![0.0; 5]; memory_size],
            memory_pointer: 0,
            memory_count: 0,
            memory_size,
        }
    }

    /// Encode Ising observables to emotion vector (no learned weights)
    pub fn encode_emotion(&self, system: &IsingSystem) -> EmotionVector {
        let e = system.energy();
        let m = system.magnetization();
        let f = system.frustration();
        let n = system.n as f64;

        let valence = (-e / n).tanh();
        let arousal = 1.0 - m.abs();
        let tension = f;
        let coherence = m.abs();

        EmotionVector::new(valence, arousal, tension, coherence)
    }

    /// Simulate other's Hamiltonian to predict ground state
    pub fn simulate_other(
        &self,
        other: &IsingSystem,
        anneal_steps: usize,
        seed: u64,
    ) -> IsingSystem {
        use rand::Rng;
        use rand::SeedableRng;

        let mut sim = IsingSystem::new(other.n, seed);
        sim.coupling = other.coupling.clone();
        sim.field = other.field.clone();

        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

        for step in 0..anneal_steps {
            let beta = 0.1 * (10.0 * step as f64 / anneal_steps as f64).exp();

            for _ in 0..10 {
                let i = rng.gen_range(0..sim.n);
                let e_before = sim.energy();
                sim.spins[i] *= -1;
                let e_after = sim.energy();
                let delta_e = e_after - e_before;
                let p_accept = (-beta * delta_e).exp().max(0.1 / (1.0 + beta));

                if rng.gen::<f64>() >= p_accept {
                    sim.spins[i] *= -1;
                }
            }
        }

        sim
    }

    /// Measure how well prediction matches actual
    pub fn perspective_accuracy(
        &self,
        predicted: &IsingSystem,
        actual: &IsingSystem,
    ) -> (f64, f64, f64) {
        // State overlap (accounting for Z2 symmetry)
        let match_direct = predicted
            .spins
            .iter()
            .zip(actual.spins.iter())
            .filter(|&(&p, &a)| p == a)
            .count() as f64
            / predicted.n as f64;

        let match_flipped = predicted
            .spins
            .iter()
            .zip(actual.spins.iter())
            .filter(|&(&p, &a)| p == -a)
            .count() as f64
            / predicted.n as f64;

        let state_overlap = match_direct.max(match_flipped);

        // Energy prediction error
        let e_pred = predicted.energy();
        let e_actual = actual.energy();
        let denom = e_actual.abs().max(1.0);
        let energy_error = (e_pred - e_actual).abs() / denom;

        // Magnetization error
        let mag_err = (predicted.magnetization().abs() - actual.magnetization().abs()).abs();

        (state_overlap, energy_error, mag_err)
    }

    /// Coupling similarity (cosine similarity mapped to \[0,1\])
    pub fn coupling_similarity(&self, j1: &[Vec<f64>], j2: &[Vec<f64>]) -> f64 {
        let mut dot = 0.0;
        let mut norm1 = 0.0;
        let mut norm2 = 0.0;
        let n = j1.len();

        for i in 0..n {
            for j in (i + 1)..n {
                dot += j1[i][j] * j2[i][j];
                norm1 += j1[i][j] * j1[i][j];
                norm2 += j2[i][j] * j2[i][j];
            }
        }

        let cos_sim = if norm1 > 0.0 && norm2 > 0.0 {
            dot / (norm1.sqrt() * norm2.sqrt())
        } else {
            0.0
        };

        (cos_sim + 1.0) / 2.0 // Map [-1,1] to [0,1]
    }

    /// Compute physics-grounded empathy score
    pub fn compute_empathy(
        &self,
        self_system: &IsingSystem,
        other_system: &IsingSystem,
        anneal_steps: usize,
        seed: u64,
    ) -> f64 {
        // Simulate other's state
        let predicted = self.simulate_other(other_system, anneal_steps, seed);

        // Perspective accuracy
        let (state_overlap, energy_error, _mag_error) =
            self.perspective_accuracy(&predicted, other_system);

        // Coupling similarity
        let coupling_sim = self.coupling_similarity(&self_system.coupling, &other_system.coupling);

        // Combined empathy score (weighted average)
        (0.4 * state_overlap + 0.3 * (1.0 - energy_error.min(1.0)) + 0.3 * coupling_sim)
            .clamp(0.0, 1.0)
    }

    /// Modify self's coupling based on empathic understanding
    pub fn compassionate_response(
        &self,
        self_system: &mut IsingSystem,
        other_system: &IsingSystem,
        empathy_score: f64,
        coupling_strength: f64,
    ) {
        if empathy_score > 0.5 {
            // High empathy: blend coupling
            let blend = coupling_strength * empathy_score;
            for i in 0..self_system.n {
                for j in 0..self_system.n {
                    self_system.coupling[i][j] = (1.0 - blend) * self_system.coupling[i][j]
                        + blend * other_system.coupling[i][j];
                }
            }
        } else {
            // Low empathy: add thermal noise
            use rand::Rng;
            let temp = 0.1 * (1.0 - empathy_score);
            let mut rng = rand::thread_rng();
            for i in 0..self_system.n {
                if rng.gen::<f64>() < temp {
                    self_system.spins[i] *= -1;
                }
            }
        }
    }

    /// Store emotional state in memory buffer
    pub fn store_memory(&mut self, emotion: &EmotionVector, empathy_score: f64) {
        self.memory_buffer[self.memory_pointer] = vec![
            emotion.valence,
            emotion.arousal,
            emotion.tension,
            emotion.coherence,
            empathy_score,
        ];
        self.memory_pointer = (self.memory_pointer + 1) % self.memory_size;
        self.memory_count = (self.memory_count + 1).min(self.memory_size);
    }

    /// Recall emotional statistics
    pub fn recall_memory(&self) -> (f64, f64, f64, f64, f64, f64) {
        if self.memory_count == 0 {
            return (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        }

        let mut sum = [0.0; 5];
        for i in 0..self.memory_count {
            for (j, s) in sum.iter_mut().enumerate() {
                *s += self.memory_buffer[i][j];
            }
        }

        let count = self.memory_count as f64;
        let avg_valence = sum[0] / count;
        let avg_arousal = sum[1] / count;
        let avg_tension = sum[2] / count;
        let avg_coherence = sum[3] / count;
        let avg_empathy = sum[4] / count;

        // Empathy trend
        let trend = if self.memory_count >= 4 {
            let half = self.memory_count / 2;
            let recent_avg: f64 = (half..self.memory_count)
                .map(|i| self.memory_buffer[i][4])
                .sum::<f64>()
                / (self.memory_count - half) as f64;
            let older_avg: f64 =
                (0..half).map(|i| self.memory_buffer[i][4]).sum::<f64>() / half as f64;
            recent_avg - older_avg
        } else {
            0.0
        };

        (
            avg_valence,
            avg_arousal,
            avg_tension,
            avg_coherence,
            avg_empathy,
            trend,
        )
    }

    /// Multi-agent social attention
    pub fn social_attention(
        &mut self,
        self_system: &IsingSystem,
        others: &[IsingSystem],
    ) -> Vec<f64> {
        let mut empathy_scores = Vec::new();

        for (idx, other) in others.iter().enumerate() {
            let empathy = self.compute_empathy(self_system, other, 80, 7777 + idx as u64);
            empathy_scores.push(empathy);
        }

        // Normalize to attention weights
        let total: f64 = empathy_scores.iter().sum();
        if total > 1e-9 {
            empathy_scores.iter().map(|&e| e / total).collect()
        } else {
            vec![1.0 / empathy_scores.len() as f64; empathy_scores.len()]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotion_encoding() {
        let sys = IsingSystem::new(20, 42);
        let module = IsingEmpathyModule::new(32);
        let emo = module.encode_emotion(&sys);

        assert!(emo.valence >= -1.0 && emo.valence <= 1.0);
        assert!(emo.arousal >= 0.0 && emo.arousal <= 1.0);
        assert!(emo.tension >= 0.0 && emo.tension <= 1.0);
        assert!(emo.coherence >= 0.0 && emo.coherence <= 1.0);
    }

    #[test]
    fn test_energy_conservation() {
        let sys = IsingSystem::new(20, 42);
        let e1 = sys.energy();
        let e2 = sys.energy();
        assert!((e1 - e2).abs() < 1e-10);
    }

    #[test]
    fn test_coupling_similarity() {
        let sys1 = IsingSystem::new(20, 42);
        let sys2 = IsingSystem::new(20, 42);
        let module = IsingEmpathyModule::new(32);

        let sim = module.coupling_similarity(&sys1.coupling, &sys2.coupling);
        assert!((sim - 1.0).abs() < 1e-6); // Same seed => same coupling
    }

    #[test]
    fn test_memory_storage() {
        let mut module = IsingEmpathyModule::new(16);
        for i in 0..10 {
            let emo =
                EmotionVector::new(0.1 * i as f64, 1.0 - 0.1 * i as f64, 0.05, 0.1 * i as f64);
            module.store_memory(&emo, 0.1 * i as f64);
        }

        let (_v, _a, _t, _c, avg_emp, trend) = module.recall_memory();
        assert!(trend > 0.0); // Increasing empathy
    }
}
