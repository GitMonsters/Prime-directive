//! Text Embedder for OCTO RNA Bridge
//!
//! Converts text input to 256-dimensional embeddings for RNA analysis.
//! Uses character frequency, word features, semantic markers, and n-grams.

use std::collections::HashMap;

/// Text embedder that converts input text to fixed-size vectors
#[derive(Debug, Clone)]
pub struct TextEmbedder {
    /// Hidden dimension (must match OCTO's expected input)
    hidden_dim: usize,
}

impl TextEmbedder {
    /// Create a new text embedder with specified dimension
    pub fn new(hidden_dim: usize) -> Self {
        Self { hidden_dim }
    }

    /// Create embedder with default 256 dimensions
    pub fn default_dim() -> Self {
        Self::new(256)
    }

    /// Embed text into a fixed-size vector
    pub fn embed(&self, text: &str) -> Vec<f32> {
        let mut embedding = vec![0.0f32; self.hidden_dim];
        let text_lower = text.to_lowercase();
        let chars: Vec<char> = text.chars().collect();
        let words: Vec<&str> = text.split_whitespace().collect();

        // Dimensions 0-63: Character frequency encoding
        self.encode_char_frequency(&chars, &mut embedding[0..64]);

        // Dimensions 64-95: Word-level features
        self.encode_word_features(&words, &text_lower, &mut embedding[64..96]);

        // Dimensions 96-127: Semantic markers
        self.encode_semantic_markers(&text_lower, &mut embedding[96..128]);

        // Dimensions 128-191: N-gram features
        self.encode_ngrams(&text_lower, &mut embedding[128..192]);

        // Dimensions 192-223: Structural features
        self.encode_structure(&chars, &words, &mut embedding[192..224]);

        // Dimensions 224-255: Question/intent markers
        self.encode_intent(&text_lower, &chars, &mut embedding[224..256]);

        // Normalize the embedding
        self.normalize(&mut embedding);

        embedding
    }

    /// Encode character frequency distribution (64 dims)
    fn encode_char_frequency(&self, chars: &[char], out: &mut [f32]) {
        let total = chars.len().max(1) as f32;

        // Count character frequencies by category
        let mut counts = HashMap::new();
        for c in chars {
            *counts.entry(*c).or_insert(0usize) += 1;
        }

        // Lowercase letters a-z (dims 0-25)
        for (i, c) in ('a'..='z').enumerate() {
            out[i] = *counts.get(&c).unwrap_or(&0) as f32 / total;
        }

        // Digits 0-9 (dims 26-35)
        for (i, c) in ('0'..='9').enumerate() {
            out[26 + i] = *counts.get(&c).unwrap_or(&0) as f32 / total;
        }

        // Punctuation and special chars (dims 36-50)
        let punctuation = [
            '.', ',', '!', '?', ':', ';', '-', '_', '\'', '"', '(', ')', '[', ']', '/',
        ];
        for (i, c) in punctuation.iter().enumerate() {
            out[36 + i] = *counts.get(c).unwrap_or(&0) as f32 / total;
        }

        // Whitespace ratio (dim 51)
        out[51] = chars.iter().filter(|c| c.is_whitespace()).count() as f32 / total;

        // Uppercase ratio (dim 52)
        out[52] = chars.iter().filter(|c| c.is_uppercase()).count() as f32 / total;

        // Symbol ratio (dim 53)
        out[53] = chars
            .iter()
            .filter(|c| !c.is_alphanumeric() && !c.is_whitespace())
            .count() as f32
            / total;

        // Character diversity (dim 54)
        out[54] = (counts.len() as f32 / 100.0).min(1.0);

        // Average word length indicator (dims 55-63 unused, reserved)
    }

    /// Encode word-level features (32 dims)
    fn encode_word_features(&self, words: &[&str], text: &str, out: &mut [f32]) {
        let word_count = words.len().max(1) as f32;
        let char_count = text.len().max(1) as f32;

        // Word count normalized (dim 0)
        out[0] = (word_count / 50.0).min(1.0);

        // Average word length (dim 1)
        let avg_len: f32 = words.iter().map(|w| w.len() as f32).sum::<f32>() / word_count;
        out[1] = (avg_len / 10.0).min(1.0);

        // Max word length (dim 2)
        let max_len = words.iter().map(|w| w.len()).max().unwrap_or(0) as f32;
        out[2] = (max_len / 20.0).min(1.0);

        // Word length variance (dim 3)
        let variance: f32 = words
            .iter()
            .map(|w| (w.len() as f32 - avg_len).powi(2))
            .sum::<f32>()
            / word_count;
        out[3] = (variance / 25.0).min(1.0);

        // Unique word ratio (dim 4)
        let unique: std::collections::HashSet<&str> = words.iter().copied().collect();
        out[4] = unique.len() as f32 / word_count;

        // Short words ratio (len <= 3) (dim 5)
        out[5] = words.iter().filter(|w| w.len() <= 3).count() as f32 / word_count;

        // Long words ratio (len >= 8) (dim 6)
        out[6] = words.iter().filter(|w| w.len() >= 8).count() as f32 / word_count;

        // Capitalized words ratio (dim 7)
        out[7] = words
            .iter()
            .filter(|w| w.chars().next().map(|c| c.is_uppercase()).unwrap_or(false))
            .count() as f32
            / word_count;

        // Words per character (dim 8)
        out[8] = word_count / char_count;

        // Technical indicator words (dims 9-20)
        let tech_words = [
            "function", "code", "error", "api", "data", "system", "process", "file", "module",
            "class", "method", "variable",
        ];
        for (i, tw) in tech_words.iter().enumerate() {
            if text.contains(tw) {
                out[9 + i] = 1.0;
            }
        }

        // Conversational indicators (dims 21-28)
        let conv_words = [
            "please", "thanks", "hello", "hi", "hey", "help", "can", "would",
        ];
        for (i, cw) in conv_words.iter().enumerate() {
            if words.iter().any(|w| w.to_lowercase() == *cw) {
                out[21 + i] = 1.0;
            }
        }
    }

    /// Encode semantic markers (32 dims)
    fn encode_semantic_markers(&self, text: &str, out: &mut [f32]) {
        // Question words (dims 0-7)
        let q_words = ["what", "how", "why", "when", "where", "who", "which", "can"];
        for (i, qw) in q_words.iter().enumerate() {
            if text.starts_with(qw) || text.contains(&format!(" {} ", qw)) {
                out[i] = 1.0;
            }
        }

        // Imperative verbs (dims 8-15)
        let imperatives = [
            "explain", "describe", "show", "list", "tell", "write", "create", "fix",
        ];
        for (i, imp) in imperatives.iter().enumerate() {
            if text.contains(imp) {
                out[8 + i] = 1.0;
            }
        }

        // Domain markers (dims 16-23)
        let domains = [
            "python",
            "rust",
            "javascript",
            "database",
            "network",
            "security",
            "machine learning",
            "web",
        ];
        for (i, domain) in domains.iter().enumerate() {
            if text.contains(domain) {
                out[16 + i] = 1.0;
            }
        }

        // Sentiment/tone indicators (dims 24-31)
        let positive = [
            "good",
            "great",
            "excellent",
            "nice",
            "perfect",
            "thanks",
            "awesome",
            "love",
        ];
        let negative = [
            "bad", "wrong", "error", "fail", "broken", "issue", "problem", "bug",
        ];

        for (i, p) in positive.iter().take(4).enumerate() {
            if text.contains(p) {
                out[24 + i] = 1.0;
            }
        }
        for (i, n) in negative.iter().take(4).enumerate() {
            if text.contains(n) {
                out[28 + i] = 1.0;
            }
        }
    }

    /// Encode n-gram features (64 dims)
    fn encode_ngrams(&self, text: &str, out: &mut [f32]) {
        let chars: Vec<char> = text.chars().collect();

        // Bigram frequency encoding (dims 0-31)
        // Use hash-based encoding for common bigrams
        let common_bigrams = [
            "th", "he", "in", "er", "an", "re", "on", "at", "en", "nd", "ti", "es", "or", "te",
            "of", "ed", "is", "it", "al", "ar", "st", "to", "nt", "ng", "se", "ha", "as", "ou",
            "io", "le", "ve", "co",
        ];

        for (i, bg) in common_bigrams.iter().enumerate() {
            let count = text.matches(bg).count();
            out[i] = (count as f32 / 10.0).min(1.0);
        }

        // Trigram features (dims 32-47)
        let common_trigrams = [
            "the", "and", "ing", "ion", "tio", "ent", "ati", "for", "her", "ter", "hat", "tha",
            "ere", "ate", "his", "con",
        ];

        for (i, tg) in common_trigrams.iter().enumerate() {
            let count = text.matches(tg).count();
            out[32 + i] = (count as f32 / 5.0).min(1.0);
        }

        // Character transition features (dims 48-63)
        // vowel-consonant patterns
        let vowels: std::collections::HashSet<char> = "aeiou".chars().collect();
        let mut vc_transitions = 0;
        let mut cv_transitions = 0;
        let mut vv_transitions = 0;
        let mut cc_transitions = 0;

        for window in chars.windows(2) {
            if let [a, b] = window {
                let a_vowel = vowels.contains(&a.to_ascii_lowercase());
                let b_vowel = vowels.contains(&b.to_ascii_lowercase());
                let a_alpha = a.is_alphabetic();
                let b_alpha = b.is_alphabetic();

                if a_alpha && b_alpha {
                    match (a_vowel, b_vowel) {
                        (true, false) => vc_transitions += 1,
                        (false, true) => cv_transitions += 1,
                        (true, true) => vv_transitions += 1,
                        (false, false) => cc_transitions += 1,
                    }
                }
            }
        }

        let total_trans =
            (vc_transitions + cv_transitions + vv_transitions + cc_transitions).max(1) as f32;
        out[48] = vc_transitions as f32 / total_trans;
        out[49] = cv_transitions as f32 / total_trans;
        out[50] = vv_transitions as f32 / total_trans;
        out[51] = cc_transitions as f32 / total_trans;

        // Repetition detection (dims 52-55)
        let mut char_repeats = 0;
        for window in chars.windows(2) {
            if window[0] == window[1] {
                char_repeats += 1;
            }
        }
        out[52] = (char_repeats as f32 / chars.len().max(1) as f32).min(1.0);

        // Word-initial letter distribution (dims 56-63)
        // Hash words to 8 buckets based on first letter
        let words: Vec<&str> = text.split_whitespace().collect();
        for word in &words {
            if let Some(c) = word.chars().next() {
                let bucket = (c as usize) % 8;
                out[56 + bucket] += 1.0 / words.len().max(1) as f32;
            }
        }
    }

    /// Encode structural features (32 dims)
    fn encode_structure(&self, chars: &[char], words: &[&str], out: &mut [f32]) {
        let len = chars.len();

        // Length buckets (dims 0-7)
        let length_buckets = [10, 25, 50, 100, 200, 500, 1000, 2000];
        for (i, &bucket) in length_buckets.iter().enumerate() {
            if len <= bucket {
                out[i] = 1.0;
                break;
            }
        }

        // Sentence count estimate (dim 8)
        let sentence_enders = chars
            .iter()
            .filter(|c| **c == '.' || **c == '!' || **c == '?')
            .count();
        out[8] = (sentence_enders as f32 / 10.0).min(1.0);

        // Line break presence (dim 9)
        out[9] = if chars.contains(&'\n') { 1.0 } else { 0.0 };

        // Multiple lines (dim 10)
        let line_count = chars.iter().filter(|c| **c == '\n').count();
        out[10] = (line_count as f32 / 10.0).min(1.0);

        // Code-like structure indicators (dims 11-18)
        let code_indicators = ['{', '}', '(', ')', '[', ']', ';', '='];
        for (i, &ci) in code_indicators.iter().enumerate() {
            out[11 + i] = if chars.contains(&ci) { 1.0 } else { 0.0 };
        }

        // URL/path indicator (dim 19)
        let text: String = chars.iter().collect();
        out[19] = if text.contains("://") || text.contains("/") {
            1.0
        } else {
            0.0
        };

        // List indicator (dim 20)
        out[20] = if text.contains("- ") || text.contains("* ") || text.contains("1.") {
            1.0
        } else {
            0.0
        };

        // Quote presence (dim 21)
        out[21] = if chars.contains(&'"') || chars.contains(&'\'') {
            1.0
        } else {
            0.0
        };

        // Markdown indicators (dims 22-25)
        out[22] = if text.contains("```") { 1.0 } else { 0.0 }; // code block
        out[23] = if text.contains("**") || text.contains("__") {
            1.0
        } else {
            0.0
        }; // bold
        out[24] = if text.starts_with('#') { 1.0 } else { 0.0 }; // header
        out[25] = if text.contains('[') && text.contains(']') {
            1.0
        } else {
            0.0
        }; // link

        // Indentation (dim 26)
        out[26] = if text.starts_with(' ') || text.starts_with('\t') {
            1.0
        } else {
            0.0
        };

        // Density: chars per word (dim 27)
        out[27] = (len as f32 / words.len().max(1) as f32 / 10.0).min(1.0);

        // Remaining dims reserved
    }

    /// Encode question/intent markers (32 dims)
    fn encode_intent(&self, text: &str, chars: &[char], out: &mut [f32]) {
        // Question mark presence (dim 0)
        out[0] = if chars.contains(&'?') { 1.0 } else { 0.0 };

        // Multiple questions (dim 1)
        let q_count = chars.iter().filter(|c| **c == '?').count();
        out[1] = (q_count as f32 / 3.0).min(1.0);

        // Exclamation (dim 2)
        out[2] = if chars.contains(&'!') { 1.0 } else { 0.0 };

        // Request patterns (dims 3-10)
        let requests = [
            "can you",
            "could you",
            "would you",
            "please",
            "i need",
            "i want",
            "help me",
            "how do i",
        ];
        for (i, req) in requests.iter().enumerate() {
            if text.contains(req) {
                out[3 + i] = 1.0;
            }
        }

        // Explanation requests (dims 11-14)
        let explain = ["explain", "what is", "what are", "how does"];
        for (i, exp) in explain.iter().enumerate() {
            if text.contains(exp) {
                out[11 + i] = 1.0;
            }
        }

        // Action requests (dims 15-18)
        let actions = ["create", "make", "build", "generate"];
        for (i, act) in actions.iter().enumerate() {
            if text.contains(act) {
                out[15 + i] = 1.0;
            }
        }

        // Debug/fix requests (dims 19-22)
        let debug = ["fix", "debug", "error", "not working"];
        for (i, dbg) in debug.iter().enumerate() {
            if text.contains(dbg) {
                out[19 + i] = 1.0;
            }
        }

        // Comparison requests (dims 23-25)
        let compare = ["compare", "difference", "vs"];
        for (i, cmp) in compare.iter().enumerate() {
            if text.contains(cmp) {
                out[23 + i] = 1.0;
            }
        }

        // Opinion/recommendation (dims 26-28)
        let opinion = ["should i", "recommend", "best"];
        for (i, op) in opinion.iter().enumerate() {
            if text.contains(op) {
                out[26 + i] = 1.0;
            }
        }

        // Urgency indicators (dims 29-31)
        let urgent = ["urgent", "asap", "immediately"];
        for (i, urg) in urgent.iter().enumerate() {
            if text.contains(urg) {
                out[29 + i] = 1.0;
            }
        }
    }

    /// Normalize embedding to unit length
    fn normalize(&self, embedding: &mut [f32]) {
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 1e-8 {
            for x in embedding.iter_mut() {
                *x /= norm;
            }
        }
    }

    /// Get the embedding dimension
    pub fn dim(&self) -> usize {
        self.hidden_dim
    }
}

impl Default for TextEmbedder {
    fn default() -> Self {
        Self::default_dim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embed_dimension() {
        let embedder = TextEmbedder::new(256);
        let embedding = embedder.embed("Hello, world!");
        assert_eq!(embedding.len(), 256);
    }

    #[test]
    fn test_embed_normalized() {
        let embedder = TextEmbedder::default();
        let embedding = embedder.embed("This is a test sentence.");
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!(
            (norm - 1.0).abs() < 0.01,
            "Embedding should be normalized, got norm={}",
            norm
        );
    }

    #[test]
    fn test_embed_different_inputs() {
        let embedder = TextEmbedder::default();
        let e1 = embedder.embed("What is recursion?");
        let e2 = embedder.embed("Fix this bug in my code");

        // Embeddings should be different
        let diff: f32 = e1.iter().zip(e2.iter()).map(|(a, b)| (a - b).abs()).sum();
        assert!(
            diff > 0.1,
            "Different inputs should produce different embeddings"
        );
    }

    #[test]
    fn test_question_detection() {
        let embedder = TextEmbedder::default();
        let e_question = embedder.embed("What is the meaning of life?");
        let e_statement = embedder.embed("The sky is blue.");

        // Question embedding should have question marker active (dim 224)
        assert!(
            e_question[224] > e_statement[224],
            "Question should have higher question marker"
        );
    }

    #[test]
    fn test_code_detection() {
        let embedder = TextEmbedder::default();
        let e_code = embedder.embed("function foo() { return 42; }");
        let e_prose = embedder.embed("The quick brown fox jumps over the lazy dog.");

        // Code embedding should have code structure markers active
        // dims 203-210 are code indicators {, }, (, ), [, ], ;, =
        let code_score: f32 = e_code[203..211].iter().sum();
        let prose_score: f32 = e_prose[203..211].iter().sum();
        assert!(
            code_score > prose_score,
            "Code should have higher code structure score"
        );
    }
}
