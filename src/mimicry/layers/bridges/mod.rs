//! Inter-layer bridge implementations.
//!
//! This module contains concrete implementations of bidirectional bridges
//! that connect the 7 layers of the multiplicative integration system.

pub mod base_extended;
pub mod cross_domain;
pub mod individual_collective;
pub mod internal_external;
pub mod physics_consciousness;
pub mod physics_language;

// Re-export bridge implementations
pub use base_extended::BaseExtendedBridge;
pub use cross_domain::CrossDomainBridge;
pub use individual_collective::IndividualCollectiveBridge;
pub use internal_external::InternalExternalBridge;
pub use physics_consciousness::PhysicsConsciousnessBridge;
pub use physics_language::PhysicsLanguageBridge;

use std::sync::Arc;
use super::{BidirectionalBridge, BridgeNetwork};

/// Create a fully connected bridge network with all standard bridges.
pub fn create_default_bridge_network() -> BridgeNetwork {
    let mut network = BridgeNetwork::new();

    // Register all bridges
    network.register(Arc::new(BaseExtendedBridge::new()));
    network.register(Arc::new(CrossDomainBridge::new()));
    network.register(Arc::new(PhysicsConsciousnessBridge::new()));
    network.register(Arc::new(PhysicsLanguageBridge::new()));
    network.register(Arc::new(IndividualCollectiveBridge::new()));
    network.register(Arc::new(InternalExternalBridge::new()));

    network
}

/// Bridge builder for creating custom bridge configurations.
pub struct BridgeBuilder {
    bridges: Vec<Arc<dyn BidirectionalBridge>>,
    global_amplification: f32,
}

impl BridgeBuilder {
    /// Create a new bridge builder.
    pub fn new() -> Self {
        Self {
            bridges: Vec::new(),
            global_amplification: 1.1,
        }
    }

    /// Add the base-extended physics bridge.
    pub fn with_base_extended(mut self) -> Self {
        self.bridges.push(Arc::new(BaseExtendedBridge::new()));
        self
    }

    /// Add the cross-domain bridge.
    pub fn with_cross_domain(mut self) -> Self {
        self.bridges.push(Arc::new(CrossDomainBridge::new()));
        self
    }

    /// Add the physics-consciousness bridge.
    pub fn with_physics_consciousness(mut self) -> Self {
        self.bridges.push(Arc::new(PhysicsConsciousnessBridge::new()));
        self
    }

    /// Add the physics-language bridge.
    pub fn with_physics_language(mut self) -> Self {
        self.bridges.push(Arc::new(PhysicsLanguageBridge::new()));
        self
    }

    /// Add the individual-collective bridge.
    pub fn with_individual_collective(mut self) -> Self {
        self.bridges.push(Arc::new(IndividualCollectiveBridge::new()));
        self
    }

    /// Add the internal-external bridge.
    pub fn with_internal_external(mut self) -> Self {
        self.bridges.push(Arc::new(InternalExternalBridge::new()));
        self
    }

    /// Add all standard bridges.
    pub fn with_all_bridges(self) -> Self {
        self.with_base_extended()
            .with_cross_domain()
            .with_physics_consciousness()
            .with_physics_language()
            .with_individual_collective()
            .with_internal_external()
    }

    /// Set global amplification factor.
    pub fn with_global_amplification(mut self, factor: f32) -> Self {
        self.global_amplification = factor;
        self
    }

    /// Add a custom bridge.
    pub fn with_custom_bridge(mut self, bridge: Arc<dyn BidirectionalBridge>) -> Self {
        self.bridges.push(bridge);
        self
    }

    /// Build the bridge network.
    pub fn build(self) -> BridgeNetwork {
        let mut network = BridgeNetwork::new();
        network.set_global_amplification(self.global_amplification);
        
        for bridge in self.bridges {
            network.register(bridge);
        }

        network
    }
}

impl Default for BridgeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_bridge_network() {
        let network = create_default_bridge_network();
        assert_eq!(network.bridges().len(), 6);
    }

    #[test]
    fn test_bridge_builder() {
        let network = BridgeBuilder::new()
            .with_base_extended()
            .with_physics_consciousness()
            .with_global_amplification(1.5)
            .build();

        assert_eq!(network.bridges().len(), 2);
        assert_eq!(network.global_amplification(), 1.5);
    }

    #[test]
    fn test_builder_all_bridges() {
        let network = BridgeBuilder::new().with_all_bridges().build();
        assert_eq!(network.bridges().len(), 6);
    }
}
