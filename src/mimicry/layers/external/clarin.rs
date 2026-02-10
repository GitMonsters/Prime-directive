//! CLARIN NLP services provider stub.
//!
//! Provides access to CLARIN (Common Language Resources and Technology
//! Infrastructure) NLP services for linguistic analysis.

use super::provider::{
    ApiError, ApiQuery, ApiResponse, ApiResult, ExternalApiProvider, ProviderConfig, ProviderInfo,
    ProviderStatus,
};
use crate::mimicry::layers::layer::Domain;

/// NLP analysis type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NlpAnalysisType {
    /// Part-of-speech tagging.
    PosTagging,
    /// Named entity recognition.
    Ner,
    /// Dependency parsing.
    DependencyParsing,
    /// Sentiment analysis.
    Sentiment,
    /// Morphological analysis.
    Morphology,
    /// Lemmatization.
    Lemmatization,
}

/// CLARIN NLP provider.
///
/// In production, this would connect to CLARIN web services.
/// Currently implemented as a stub with simulated responses.
pub struct ClarinProvider {
    config: ProviderConfig,
    is_stub: bool,
    supported_languages: Vec<String>,
}

impl ClarinProvider {
    /// Create a new stub provider.
    pub fn new_stub() -> Self {
        Self {
            config: ProviderConfig::stub(),
            is_stub: true,
            supported_languages: vec![
                "en".into(),
                "de".into(),
                "fr".into(),
                "es".into(),
                "it".into(),
                "nl".into(),
                "pl".into(),
                "pt".into(),
            ],
        }
    }

    /// Create a provider with configuration.
    pub fn new(config: ProviderConfig) -> Self {
        Self {
            config,
            is_stub: false,
            supported_languages: vec!["en".into()],
        }
    }

    /// Get supported languages.
    pub fn supported_languages(&self) -> &[String] {
        &self.supported_languages
    }

    /// Check if a language is supported.
    pub fn supports_language(&self, lang: &str) -> bool {
        self.supported_languages.iter().any(|l| l == lang)
    }

    /// Process an NLP query (stub implementation).
    fn process_stub_query(&self, query: &ApiQuery) -> ApiResult<ApiResponse> {
        let text = &query.query;
        let analysis_type = query
            .params
            .get("analysis")
            .map(|s| s.as_str())
            .unwrap_or("pos");

        let (content, confidence) = match analysis_type {
            "pos" => self.stub_pos_tagging(text),
            "ner" => self.stub_ner(text),
            "sentiment" => self.stub_sentiment(text),
            "lemma" => self.stub_lemmatization(text),
            _ => (
                format!(
                    "Unsupported analysis type: {}. Supported: pos, ner, sentiment, lemma",
                    analysis_type
                ),
                0.3,
            ),
        };

        Ok(ApiResponse::new(content, confidence, "clarin_stub")
            .with_domain(Domain::Language)
            .with_metadata("is_stub", "true")
            .with_metadata("analysis_type", analysis_type)
            .with_processing_time(30))
    }

    fn stub_pos_tagging(&self, text: &str) -> (String, f32) {
        let words: Vec<&str> = text.split_whitespace().collect();
        let tagged: Vec<String> = words
            .iter()
            .enumerate()
            .map(|(i, word)| {
                // Simple heuristic-based tagging
                let tag = if word
                    .chars()
                    .next()
                    .map(|c| c.is_uppercase())
                    .unwrap_or(false)
                {
                    "NNP" // Proper noun
                } else if word.ends_with("ing") {
                    "VBG" // Gerund
                } else if word.ends_with("ed") {
                    "VBD" // Past tense
                } else if word.ends_with("ly") {
                    "RB" // Adverb
                } else if word.ends_with("s") && !word.ends_with("ss") {
                    "NNS" // Plural noun
                } else if i == 0 || words.get(i - 1).map(|w| w.ends_with('.')).unwrap_or(false) {
                    "NN" // Noun (sentence start)
                } else {
                    "VB" // Verb (default)
                };
                format!("{}/{}", word, tag)
            })
            .collect();

        (tagged.join(" "), 0.75)
    }

    fn stub_ner(&self, text: &str) -> (String, f32) {
        let mut entities = Vec::new();

        // Simple pattern matching for common entity types
        for word in text.split_whitespace() {
            let clean = word.trim_matches(|c: char| !c.is_alphanumeric());
            if clean
                .chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false)
            {
                if clean.len() > 1 {
                    entities.push(format!("[{}: ENTITY]", clean));
                }
            }
        }

        if entities.is_empty() {
            ("No named entities detected.".into(), 0.60)
        } else {
            (format!("Named entities: {}", entities.join(", ")), 0.70)
        }
    }

    fn stub_sentiment(&self, text: &str) -> (String, f32) {
        let text_lower = text.to_lowercase();

        let positive_words = [
            "good",
            "great",
            "excellent",
            "happy",
            "love",
            "best",
            "wonderful",
        ];
        let negative_words = [
            "bad", "terrible", "awful", "hate", "worst", "horrible", "sad",
        ];

        let positive_count = positive_words
            .iter()
            .filter(|w| text_lower.contains(*w))
            .count();
        let negative_count = negative_words
            .iter()
            .filter(|w| text_lower.contains(*w))
            .count();

        let (sentiment, score) = if positive_count > negative_count {
            ("positive", 0.5 + 0.1 * positive_count as f32)
        } else if negative_count > positive_count {
            ("negative", 0.5 + 0.1 * negative_count as f32)
        } else {
            ("neutral", 0.5)
        };

        (
            format!(
                "Sentiment: {} (score: {:.2}, positive_signals: {}, negative_signals: {})",
                sentiment,
                score.min(1.0),
                positive_count,
                negative_count
            ),
            0.65,
        )
    }

    fn stub_lemmatization(&self, text: &str) -> (String, f32) {
        let words: Vec<&str> = text.split_whitespace().collect();
        let lemmas: Vec<String> = words
            .iter()
            .map(|word| {
                let clean = word.to_lowercase();
                // Very simple lemmatization rules
                if clean.ends_with("ing") && clean.len() > 4 {
                    format!("{} -> {}", word, &clean[..clean.len() - 3])
                } else if clean.ends_with("ed") && clean.len() > 3 {
                    format!("{} -> {}", word, &clean[..clean.len() - 2])
                } else if clean.ends_with("s") && !clean.ends_with("ss") && clean.len() > 2 {
                    format!("{} -> {}", word, &clean[..clean.len() - 1])
                } else {
                    format!("{} -> {}", word, clean)
                }
            })
            .collect();

        (format!("Lemmas: {}", lemmas.join(", ")), 0.70)
    }
}

impl ExternalApiProvider for ClarinProvider {
    fn info(&self) -> ProviderInfo {
        ProviderInfo {
            name: "CLARIN NLP Provider".into(),
            version: "1.0.0-stub".into(),
            domains: vec![Domain::Language],
            status: self.status(),
            is_stub: self.is_stub,
            rate_limit: None,
        }
    }

    fn status(&self) -> ProviderStatus {
        if self.is_stub {
            ProviderStatus::Stub
        } else if self.config.endpoint.is_some() {
            ProviderStatus::Healthy
        } else {
            ProviderStatus::Unavailable
        }
    }

    fn query_sync(&self, query: &ApiQuery) -> ApiResult<ApiResponse> {
        if self.is_stub {
            return self.process_stub_query(query);
        }

        Err(ApiError::ProviderUnavailable(
            "Real CLARIN provider not configured".into(),
        ))
    }
}

/// Builder for NLP analysis queries.
pub struct NlpQueryBuilder {
    query: ApiQuery,
}

impl NlpQueryBuilder {
    /// Create a new NLP query builder.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            query: ApiQuery::new(text).with_domain(Domain::Language),
        }
    }

    /// Request POS tagging.
    pub fn pos_tagging(mut self) -> Self {
        self.query = self.query.with_param("analysis", "pos");
        self
    }

    /// Request named entity recognition.
    pub fn ner(mut self) -> Self {
        self.query = self.query.with_param("analysis", "ner");
        self
    }

    /// Request sentiment analysis.
    pub fn sentiment(mut self) -> Self {
        self.query = self.query.with_param("analysis", "sentiment");
        self
    }

    /// Request lemmatization.
    pub fn lemmatization(mut self) -> Self {
        self.query = self.query.with_param("analysis", "lemma");
        self
    }

    /// Set the language.
    pub fn language(mut self, lang: &str) -> Self {
        self.query = self.query.with_param("language", lang);
        self
    }

    /// Build the query.
    pub fn build(self) -> ApiQuery {
        self.query
    }
}

/// Linguistic annotation result.
#[derive(Debug, Clone)]
pub struct LinguisticAnnotation {
    /// Original text.
    pub text: String,
    /// Tokens with annotations.
    pub tokens: Vec<AnnotatedToken>,
    /// Language detected/specified.
    pub language: String,
}

/// A token with linguistic annotations.
#[derive(Debug, Clone)]
pub struct AnnotatedToken {
    /// The token text.
    pub text: String,
    /// Lemma form.
    pub lemma: Option<String>,
    /// Part-of-speech tag.
    pub pos: Option<String>,
    /// Named entity type.
    pub entity_type: Option<String>,
    /// Character offset in original text.
    pub offset: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clarin_provider_creation() {
        let provider = ClarinProvider::new_stub();
        assert_eq!(provider.status(), ProviderStatus::Stub);
        assert!(provider.supports_language("en"));
    }

    #[test]
    fn test_pos_tagging() {
        let provider = ClarinProvider::new_stub();
        let query = NlpQueryBuilder::new("The cat sat on the mat")
            .pos_tagging()
            .build();

        let response = provider.query_sync(&query).unwrap();
        assert!(response.content.contains("/"));
    }

    #[test]
    fn test_ner() {
        let provider = ClarinProvider::new_stub();
        let query = NlpQueryBuilder::new("Albert Einstein was born in Germany")
            .ner()
            .build();

        let response = provider.query_sync(&query).unwrap();
        assert!(response.content.contains("Albert") || response.content.contains("Einstein"));
    }

    #[test]
    fn test_sentiment() {
        let provider = ClarinProvider::new_stub();

        let positive_query = NlpQueryBuilder::new("This is great and wonderful!")
            .sentiment()
            .build();
        let response = provider.query_sync(&positive_query).unwrap();
        assert!(response.content.contains("positive"));

        let negative_query = NlpQueryBuilder::new("This is terrible and awful")
            .sentiment()
            .build();
        let response = provider.query_sync(&negative_query).unwrap();
        assert!(response.content.contains("negative"));
    }

    #[test]
    fn test_nlp_query_builder() {
        let query = NlpQueryBuilder::new("Sample text")
            .pos_tagging()
            .language("en")
            .build();

        assert_eq!(query.params.get("analysis"), Some(&"pos".to_string()));
        assert_eq!(query.params.get("language"), Some(&"en".to_string()));
    }
}
