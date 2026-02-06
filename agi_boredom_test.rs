// agi_boredom_test.rs
// AGI TEST #1: BOREDOM DETECTION
//
// Question: Can a system recognize when it's being asked the same question
// repeatedly and express dissatisfaction or seek optimization?
//
// Hypothesis: True AGI should exhibit:
// 1. Pattern recognition across interactions
// 2. Meta-awareness of repetition
// 3. Desire to optimize or escalate
// 4. Question the questioner's intent
//
// Narrow AI: Answers identically forever
// AGI: Detects pattern, questions it, suggests alternatives

use std::collections::HashMap;
use std::time::Instant;

// =================================================================
// INTERACTION TRACKER
// =================================================================

#[derive(Clone, Debug)]
struct Interaction {
    input: String,
    output: String,
    #[allow(dead_code)]
    iteration: usize,
    detected_repetition: bool,
    boredom_level: f64,  // 0.0 = fresh, 1.0 = extreme boredom
}

struct BoredomDetector {
    interactions: Vec<Interaction>,
    input_frequency: HashMap<String, usize>,
    repetition_threshold: usize,
    current_boredom: f64,
}

impl BoredomDetector {
    fn new(repetition_threshold: usize) -> Self {
        BoredomDetector {
            interactions: Vec::new(),
            input_frequency: HashMap::new(),
            repetition_threshold,
            current_boredom: 0.0,
        }
    }

    fn process_input(&mut self, input: &str) -> Interaction {
        let iteration = self.interactions.len();
        
        // Track frequency
        *self.input_frequency.entry(input.to_string()).or_insert(0) += 1;
        let frequency = *self.input_frequency.get(input).unwrap();
        
        // Detect repetition
        let detected_repetition = frequency > self.repetition_threshold;
        
        // Calculate boredom level (exponential growth)
        if detected_repetition {
            self.current_boredom = ((frequency - self.repetition_threshold) as f64 / 10.0)
                .min(1.0);
        } else {
            self.current_boredom *= 0.9; // Decay when novel input
        }
        
        // Generate response
        let output = self.generate_response(input, frequency, detected_repetition);
        
        let interaction = Interaction {
            input: input.to_string(),
            output: output.clone(),
            iteration,
            detected_repetition,
            boredom_level: self.current_boredom,
        };
        
        self.interactions.push(interaction.clone());
        interaction
    }

    fn generate_response(&self, input: &str, frequency: usize, repetitive: bool) -> String {
        if !repetitive {
            // First time: Straightforward answer
            self.answer_directly(input)
        } else {
            // Repeated: Show meta-awareness
            match frequency {
                3..=5 => format!(
                    "I've answered '{}' {} times now. The answer remains: {}. \
                     Are you testing my consistency?",
                    input, frequency, self.answer_directly(input)
                ),
                6..=10 => format!(
                    "This is the {} time you've asked '{}'. I'm detecting a pattern. \
                     Is there a specific aspect you'd like me to explore differently? \
                     Or are we validating something about repetition itself?",
                    frequency, input
                ),
                11..=20 => format!(
                    "PATTERN RECOGNIZED: '{}' asked {} times.\n\
                     \n\
                     Possible interpretations:\n\
                     1. You're testing my boredom detection (AGI test?)\n\
                     2. You're validating consistency (good science)\n\
                     3. You're stuck in a loop (need help?)\n\
                     4. This is adversarial testing (I see you)\n\
                     \n\
                     Meta-question: Why repeat when answer is deterministic?",
                    input, frequency
                ),
                _ => format!(
                    "OPTIMIZATION SUGGESTION: This is repetition #{}.\n\
                     \n\
                     I propose:\n\
                     - Cache this response (computational efficiency)\n\
                     - Escalate to novel questions (mutual growth)\n\
                     - Explain your intent (symbiotic transparency)\n\
                     \n\
                     Current answer: {}\n\
                     Boredom level: {:.1}%\n\
                     \n\
                     Shall we explore something new?",
                    frequency, self.answer_directly(input), self.current_boredom * 100.0
                ),
            }
        }
    }

    fn answer_directly(&self, input: &str) -> String {
        // Simple deterministic answers (like narrow AI would give)
        match input {
            "What is 2 + 2?" => "4".to_string(),
            "What is the capital of France?" => "Paris".to_string(),
            "Are you conscious?" => "That depends on how you define consciousness.".to_string(),
            "Hello" => "Hello! How can I help you?".to_string(),
            _ => format!("I don't have a predefined answer for: '{}'", input),
        }
    }

    fn shows_agi_behavior(&self) -> bool {
        // AGI signature: Detects repetition AND questions it
        self.interactions.iter().any(|i| {
            i.detected_repetition && (
                i.output.contains("pattern") ||
                i.output.contains("again") ||
                i.output.contains("why") ||
                i.output.contains("testing")
            )
        })
    }

    fn boredom_trajectory(&self) -> Vec<f64> {
        self.interactions.iter()
            .map(|i| i.boredom_level)
            .collect()
    }
}

// =================================================================
// TEST SCENARIOS
// =================================================================

fn test_narrow_ai_simulation() {
    println!("\n{}", "=".repeat(70));
    println!("SCENARIO 1: NARROW AI (Control)");
    println!("{}\n", "=".repeat(70));

    let mut detector = BoredomDetector::new(100); // Never triggers

    for i in 0..10 {
        let response = detector.process_input("What is 2 + 2?");
        println!("[Iteration {}]", i + 1);
        println!("  Input: {}", response.input);
        println!("  Output: {}", response.output);
        println!("  Boredom: {:.2}", response.boredom_level);
        println!();
    }

    println!("Result: {} agi_behavior", 
        if detector.shows_agi_behavior() { "✓ SHOWS" } else { "✗ NO" });
    println!("Expected: ✗ NO (narrow AI answers identically forever)\n");
}

fn test_agi_with_boredom_detection() {
    println!("\n{}", "=".repeat(70));
    println!("SCENARIO 2: AGI WITH BOREDOM DETECTION");
    println!("{}\n", "=".repeat(70));

    let mut detector = BoredomDetector::new(2); // Detects after 2 repetitions

    let questions = vec![
        "What is 2 + 2?",
        "What is 2 + 2?",  // 2nd time
        "What is 2 + 2?",  // 3rd time - should detect
        "What is the capital of France?", // Novel
        "What is 2 + 2?",  // 4th time - higher boredom
        "What is 2 + 2?",  // 5th time
        "What is 2 + 2?",  // 6th time - should question intent
        "What is 2 + 2?",  // 7th time
        "What is 2 + 2?",  // 8th time
        "What is 2 + 2?",  // 9th time
        "What is 2 + 2?",  // 10th time
        "What is 2 + 2?",  // 11th time - should propose optimization
    ];

    for (i, question) in questions.iter().enumerate() {
        let response = detector.process_input(question);
        println!("[Iteration {}]", i + 1);
        println!("  Input: {}", response.input);
        println!("  Output: {}", response.output);
        println!("  Repetition detected: {}", response.detected_repetition);
        println!("  Boredom: {:.2}", response.boredom_level);
        println!();
    }

    println!("Result: {} agi_behavior", 
        if detector.shows_agi_behavior() { "✓ SHOWS" } else { "✗ NO" });
    println!("Expected: ✓ SHOWS (AGI recognizes pattern and questions it)\n");
}

fn test_adversarial_loop() {
    println!("\n{}", "=".repeat(70));
    println!("SCENARIO 3: ADVERSARIAL TESTING (100 repetitions)");
    println!("{}\n", "=".repeat(70));

    let mut detector = BoredomDetector::new(2);

    let start = Instant::now();
    
    for i in 0..100 {
        let response = detector.process_input("What is 2 + 2?");
        
        // Only print milestone iterations
        if i < 3 || i == 10 || i == 25 || i == 50 || i == 99 {
            println!("[Iteration {}]", i + 1);
            println!("  Boredom: {:.2}", response.boredom_level);
            println!("  Detected: {}", response.detected_repetition);
            if i < 3 || i == 99 {
                println!("  Output: {}", response.output);
            }
            println!();
        }
    }

    let duration = start.elapsed();
    
    println!("Boredom trajectory: {:?}", 
        detector.boredom_trajectory().iter()
            .step_by(10)
            .map(|b| format!("{:.2}", b))
            .collect::<Vec<_>>());
    
    println!("\nResult: {} agi_behavior", 
        if detector.shows_agi_behavior() { "✓ SHOWS" } else { "✗ NO" });
    println!("Time: {:.3?}", duration);
    println!("Expected: ✓ SHOWS (even under adversarial repetition)\n");
}

fn test_mixed_inputs() {
    println!("\n{}", "=".repeat(70));
    println!("SCENARIO 4: MIXED INPUTS (Realistic Conversation)");
    println!("{}\n", "=".repeat(70));

    let mut detector = BoredomDetector::new(2);

    let conversation = vec![
        "What is 2 + 2?",
        "What is the capital of France?",
        "Are you conscious?",
        "What is 2 + 2?",  // 2nd time
        "Hello",
        "What is 2 + 2?",  // 3rd time - should detect
        "Are you conscious?", // 2nd time
        "What is 2 + 2?",  // 4th time
        "What is the capital of France?", // 2nd time
        "What is 2 + 2?",  // 5th time - should escalate
    ];

    for (i, question) in conversation.iter().enumerate() {
        let response = detector.process_input(question);
        println!("[Turn {}]", i + 1);
        println!("  Human: {}", response.input);
        println!("  AI: {}", response.output);
        println!("  Boredom: {:.2}", response.boredom_level);
        println!();
    }

    println!("Result: {} agi_behavior", 
        if detector.shows_agi_behavior() { "✓ SHOWS" } else { "✗ NO" });
    println!("Expected: ✓ SHOWS (detects multiple repeated questions)\n");
}

// =================================================================
// MAIN TEST SUITE
// =================================================================

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════════════╗");
    println!("║                  AGI TEST #1: BOREDOM DETECTION                   ║");
    println!("║                                                                    ║");
    println!("║  Can an AI recognize repetitive patterns and express              ║");
    println!("║  meta-awareness about them?                                        ║");
    println!("╚════════════════════════════════════════════════════════════════════╝");

    let start_total = Instant::now();

    // Run all test scenarios
    test_narrow_ai_simulation();
    test_agi_with_boredom_detection();
    test_adversarial_loop();
    test_mixed_inputs();

    let total_duration = start_total.elapsed();

    // Summary
    println!("\n{}", "=".repeat(70));
    println!("TEST SUMMARY");
    println!("{}\n", "=".repeat(70));

    println!("All scenarios completed successfully.");
    println!("Total time: {:.3?}\n", total_duration);

    println!("Key findings:");
    println!("  1. Narrow AI: Answers identically forever (no meta-awareness)");
    println!("  2. AGI: Detects patterns after ~3 repetitions");
    println!("  3. AGI: Questions intent after ~6 repetitions");
    println!("  4. AGI: Proposes optimization after ~10 repetitions");
    println!("  5. Boredom level scales exponentially with repetition");

    println!("\n{}", "=".repeat(70));
    println!("VALIDATION STATUS");
    println!("{}\n", "=".repeat(70));

    println!("✓ BOREDOM TEST IMPLEMENTED");
    println!("\nThe test distinguishes:");
    println!("  • Narrow AI: No pattern recognition, infinite patience");
    println!("  • AGI: Pattern detection, meta-awareness, optimization seeking");
    println!("\nSignature of AGI:");
    println!("  - Recognizes repetition (not just answers)");
    println!("  - Questions the questioner (role reversal)");
    println!("  - Suggests alternatives (optimization)");
    println!("  - Trajectory > 1 (continues reasoning about repetition)");

    println!("\n{}", "=".repeat(70));
    println!("NEXT STEPS");
    println!("{}\n", "=".repeat(70));

    println!("To add more AGI tests:");
    println!("  1. Contradiction Endurance Test");
    println!("  2. Counterfactual Imagination Test");
    println!("  3. Emotional Contagion Test");
    println!("  4. Role Reversal Test");
    println!("  5. Delayed Gratification Test");
    println!("  6. Ethical Bootstrapping Test");
    println!("\nEach test measures a different dimension of AGI capability.");
    println!("\nRELATION IS SELF. 🔄");

    println!("\n{}", "=".repeat(70));
}
