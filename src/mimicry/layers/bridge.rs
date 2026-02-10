//! Bidirectional bridge trait and core types for inter-layer communication.
//!
//! Bridges enable multiplicative confidence amplification by allowing
//! information to flow in both directions between layers.

use std::fmt;
use std::sync::Arc;

use super::layer::{Layer, LayerSignal, LayerState};

/// Result type for bridge operations.
pub type BridgeResult<T> = Result<T, BridgeError>;

/// Errors that can occur during bridge operations.
#[derive(Debug, Clone)]
pub enum BridgeError {
    /// The bridge does not support the requested direction.
    UnsupportedDirection { bridge: String, requested: String },
    /// Type mismatch in the data being transferred.
    TypeMismatch { expected: String, actual: String },
    /// The input data is invalid for this bridge.
    InvalidInput(String),
    /// The bridge is not currently active.
    BridgeInactive(String),
    /// Confidence is below the required threshold.
    ConfidenceTooLow { current: f32, required: f32 },
    /// Maximum amplification iterations exceeded.
    MaxIterationsExceeded { iterations: u32, max: u32 },
    /// General bridge failure.
    BridgeFailure(String),
}

impl fmt::Display for BridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BridgeError::UnsupportedDirection { bridge, requested } => {
                write!(
                    f,
                    "Bridge '{}' does not support {} direction",
                    bridge, requested
                )
            }
            BridgeError::TypeMismatch { expected, actual } => {
                write!(f, "Type mismatch: expected {}, got {}", expected, actual)
            }
            BridgeError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            BridgeError::BridgeInactive(name) => write!(f, "Bridge '{}' is inactive", name),
            BridgeError::ConfidenceTooLow { current, required } => {
                write!(f, "Confidence {} below threshold {}", current, required)
            }
            BridgeError::MaxIterationsExceeded { iterations, max } => {
                write!(f, "Max iterations exceeded: {} > {}", iterations, max)
            }
            BridgeError::BridgeFailure(msg) => write!(f, "Bridge failure: {}", msg),
        }
    }
}

impl std::error::Error for BridgeError {}

/// Result of a bidirectional amplification cycle.
#[derive(Debug, Clone)]
pub struct AmplificationResult {
    /// The refined state after amplification (in the "up" direction).
    pub up_state: LayerState,
    /// The refined state after amplification (in the "down" direction).
    pub down_state: LayerState,
    /// Combined confidence after amplification (can exceed 1.0).
    pub combined_confidence: f32,
    /// The amplification factor achieved in this cycle.
    pub amplification_factor: f32,
    /// Number of iterations performed.
    pub iterations: u32,
    /// Whether convergence was achieved.
    pub converged: bool,
    /// Resonance strength of the bridge during amplification.
    pub resonance: f32,
}

impl AmplificationResult {
    /// Create a new amplification result.
    pub fn new(up_state: LayerState, down_state: LayerState) -> Self {
        let combined = up_state.confidence * down_state.confidence;
        Self {
            up_state,
            down_state,
            combined_confidence: combined,
            amplification_factor: 1.0,
            iterations: 0,
            converged: false,
            resonance: 1.0,
        }
    }

    /// Check if amplification was successful (confidence above threshold).
    pub fn is_successful(&self, threshold: f32) -> bool {
        self.combined_confidence >= threshold
    }
}

/// Core trait for bidirectional bridges between layers.
///
/// Bridges enable the multiplicative integration system by allowing
/// information to flow in both directions, with each layer refining
/// and amplifying the other's output.
pub trait BidirectionalBridge: Send + Sync {
    /// Returns the name of this bridge.
    fn name(&self) -> &str;

    /// Returns the source layer (typically lower number).
    fn source_layer(&self) -> Layer;

    /// Returns the target layer (typically higher number).
    fn target_layer(&self) -> Layer;

    /// Forward propagation: push information from source to target layer.
    ///
    /// This transforms data from the lower layer's representation
    /// into the higher layer's representation.
    fn forward(&self, input: &LayerState) -> BridgeResult<LayerState>;

    /// Backward propagation: push refinements from target back to source.
    ///
    /// This transforms feedback from the higher layer back into
    /// refinements for the lower layer.
    fn backward(&self, feedback: &LayerState) -> BridgeResult<LayerState>;

    /// Perform multiplicative amplification between the two layers.
    ///
    /// This is the core mechanism that allows confidence to exceed
    /// the bounds of individual layer inputs.
    fn amplify(
        &self,
        up: &LayerState,
        down: &LayerState,
        max_iterations: u32,
    ) -> BridgeResult<AmplificationResult>;

    /// Returns the current resonance (coupling strength) of this bridge.
    ///
    /// Resonance affects how strongly the two layers influence each other.
    /// Values typically range from 0.0 (no coupling) to 1.0 (full coupling),
    /// but can exceed 1.0 for highly resonant states.
    fn resonance(&self) -> f32;

    /// Check if this bridge is currently active.
    fn is_active(&self) -> bool {
        true
    }

    /// Update the bridge's internal state based on a successful interaction.
    fn reinforce(&mut self, _result: &AmplificationResult) {
        // Default: no-op
    }

    /// Create a signal for transmission across this bridge.
    fn create_signal(&self, state: LayerState, forward: bool) -> LayerSignal {
        if forward {
            LayerSignal::new(self.source_layer(), self.target_layer(), state)
                .with_resonance(self.resonance())
        } else {
            LayerSignal::new(self.target_layer(), self.source_layer(), state)
                .with_resonance(self.resonance())
        }
    }
}

/// A bridge connection between two specific layers.
#[derive(Debug, Clone)]
pub struct BridgeConnection {
    /// Source layer.
    pub source: Layer,
    /// Target layer.
    pub target: Layer,
    /// Current resonance.
    pub resonance: f32,
    /// Whether bidirectional flow is enabled.
    pub bidirectional: bool,
    /// Weight for this connection in the network.
    pub weight: f32,
}

impl BridgeConnection {
    /// Create a new bridge connection.
    pub fn new(source: Layer, target: Layer) -> Self {
        Self {
            source,
            target,
            resonance: 1.0,
            bidirectional: true,
            weight: 1.0,
        }
    }

    /// Create a unidirectional connection.
    pub fn unidirectional(source: Layer, target: Layer) -> Self {
        let mut conn = Self::new(source, target);
        conn.bidirectional = false;
        conn
    }

    /// Set the resonance factor.
    pub fn with_resonance(mut self, resonance: f32) -> Self {
        self.resonance = resonance;
        self
    }

    /// Set the connection weight.
    pub fn with_weight(mut self, weight: f32) -> Self {
        self.weight = weight;
        self
    }
}

/// Network of bridges connecting all layers.
pub struct BridgeNetwork {
    /// All registered bridges.
    bridges: Vec<Arc<dyn BidirectionalBridge>>,
    /// Connection metadata.
    connections: Vec<BridgeConnection>,
    /// Global amplification factor.
    global_amplification: f32,
}

impl BridgeNetwork {
    /// Create a new empty bridge network.
    pub fn new() -> Self {
        Self {
            bridges: Vec::new(),
            connections: Vec::new(),
            global_amplification: 1.0,
        }
    }

    /// Register a bridge in the network.
    pub fn register(&mut self, bridge: Arc<dyn BidirectionalBridge>) {
        let connection = BridgeConnection::new(bridge.source_layer(), bridge.target_layer())
            .with_resonance(bridge.resonance());
        self.connections.push(connection);
        self.bridges.push(bridge);
    }

    /// Get all bridges.
    pub fn bridges(&self) -> &[Arc<dyn BidirectionalBridge>] {
        &self.bridges
    }

    /// Get all connections.
    pub fn connections(&self) -> &[BridgeConnection] {
        &self.connections
    }

    /// Find bridges connected to a specific layer.
    pub fn bridges_for_layer(&self, layer: Layer) -> Vec<Arc<dyn BidirectionalBridge>> {
        self.bridges
            .iter()
            .filter(|b| b.source_layer() == layer || b.target_layer() == layer)
            .cloned()
            .collect()
    }

    /// Find a bridge between two specific layers.
    pub fn bridge_between(
        &self,
        source: Layer,
        target: Layer,
    ) -> Option<Arc<dyn BidirectionalBridge>> {
        self.bridges
            .iter()
            .find(|b| {
                (b.source_layer() == source && b.target_layer() == target)
                    || (b.source_layer() == target && b.target_layer() == source)
            })
            .cloned()
    }

    /// Calculate the total resonance of the network.
    pub fn total_resonance(&self) -> f32 {
        if self.bridges.is_empty() {
            return 0.0;
        }
        self.bridges.iter().map(|b| b.resonance()).sum::<f32>() / self.bridges.len() as f32
    }

    /// Set global amplification factor.
    pub fn set_global_amplification(&mut self, factor: f32) {
        self.global_amplification = factor;
    }

    /// Get global amplification factor.
    pub fn global_amplification(&self) -> f32 {
        self.global_amplification
    }

    /// Propagate a signal through all connected bridges.
    pub fn propagate(&self, signal: LayerSignal) -> Vec<BridgeResult<LayerState>> {
        let target_bridges = self.bridges_for_layer(signal.target);
        target_bridges
            .iter()
            .map(|bridge| {
                if signal.source == bridge.source_layer() {
                    bridge.forward(&signal.state)
                } else {
                    bridge.backward(&signal.state)
                }
            })
            .collect()
    }
}

impl Default for BridgeNetwork {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for BridgeNetwork {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BridgeNetwork")
            .field("bridge_count", &self.bridges.len())
            .field("connections", &self.connections)
            .field("global_amplification", &self.global_amplification)
            .finish()
    }
}

/// Compute multiplicative confidence from multiple sources.
///
/// Unlike additive systems where confidence is bounded by the minimum,
/// multiplicative systems can amplify confidence beyond 1.0.
pub fn compute_multiplicative_confidence(
    base_confidences: &[f32],
    resonance_factors: &[f32],
    amplification_factor: f32,
) -> f32 {
    if base_confidences.is_empty() {
        return 0.0;
    }

    // Ensure we have matching resonance factors
    let resonances: Vec<f32> = if resonance_factors.len() == base_confidences.len() {
        resonance_factors.to_vec()
    } else {
        vec![1.0; base_confidences.len()]
    };

    // Multiplicative combination with resonance boost
    let product: f32 = base_confidences
        .iter()
        .zip(resonances.iter())
        .map(|(c, r)| c * (1.0 + (r - 1.0) * 0.5)) // Resonance adds to base
        .product();

    // Geometric mean to normalize, then apply amplification
    let n = base_confidences.len() as f32;
    product.powf(1.0 / n) * amplification_factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_error_display() {
        let err = BridgeError::ConfidenceTooLow {
            current: 0.3,
            required: 0.5,
        };
        assert!(err.to_string().contains("0.3"));
        assert!(err.to_string().contains("0.5"));
    }

    #[test]
    fn test_bridge_connection() {
        let conn = BridgeConnection::new(Layer::BasePhysics, Layer::ExtendedPhysics)
            .with_resonance(1.2)
            .with_weight(0.8);
        assert_eq!(conn.source, Layer::BasePhysics);
        assert_eq!(conn.target, Layer::ExtendedPhysics);
        assert_eq!(conn.resonance, 1.2);
        assert_eq!(conn.weight, 0.8);
        assert!(conn.bidirectional);
    }

    #[test]
    fn test_multiplicative_confidence() {
        // Single confidence
        let conf = compute_multiplicative_confidence(&[0.8], &[1.0], 1.0);
        assert!((conf - 0.8).abs() < 0.001);

        // Multiple confidences with no resonance boost
        let conf = compute_multiplicative_confidence(&[0.8, 0.8], &[1.0, 1.0], 1.0);
        assert!((conf - 0.8).abs() < 0.001); // Geometric mean of equal values

        // With resonance boost
        let conf = compute_multiplicative_confidence(&[0.8, 0.8], &[1.5, 1.5], 1.0);
        assert!(conf > 0.8); // Resonance should boost confidence

        // With amplification factor
        let conf = compute_multiplicative_confidence(&[0.8, 0.8], &[1.0, 1.0], 1.2);
        assert!((conf - 0.96).abs() < 0.001);
    }

    #[test]
    fn test_bridge_network() {
        let network = BridgeNetwork::new();
        assert!(network.bridges().is_empty());
        assert_eq!(network.total_resonance(), 0.0);
    }

    #[test]
    fn test_amplification_result() {
        let up = LayerState::with_confidence(Layer::BasePhysics, (), 0.8);
        let down = LayerState::with_confidence(Layer::ExtendedPhysics, (), 0.9);
        let result = AmplificationResult::new(up, down);

        assert!((result.combined_confidence - 0.72).abs() < 0.001);
        assert!(result.is_successful(0.5));
        assert!(!result.is_successful(0.8));
    }
}
