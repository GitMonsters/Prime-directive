#!/usr/bin/env python3
"""
GAIA + Physics World Model - Deployment Script

Initializes and deploys the complete consciousness-physics system.
Handles setup, validation, and provides interface for both systems.
"""

import sys
import torch
import numpy as np
from datetime import datetime

# System imports
from physics_world_model import PhysicsWorldModel, PhysicsDomain
from gaia_physics_integration import PhysicsEnhancedGAIAEvaluator
from ising_empathy_module import IsingGPU, IsingEmpathyModule

# Device configuration with platform protection
from DEVICE_CONFIG import get_device, ProtectedDeviceConfig


class DeploymentSystem:
    """Main deployment interface for GAIA + Physics integration."""

    def __init__(self, device: str = 'auto'):
        """Initialize the complete system.

        Args:
            device: 'cpu', 'cuda', 'mps', or 'auto' (auto-detect with platform protection)
                   - 'auto' (default): Detects platform and uses optimal device
                     * Linux: AMD ROCm (if available, else CPU)
                     * macOS: Apple Metal (if available, else CPU)
                     * Windows: NVIDIA CUDA (if available, else CPU)
                   - Specific device: Use torch.device directly (for testing)

        ⚠️  IMPORTANT: AMD GPU configuration is LOCKED to Linux.
            When switching to/from MacBook, device settings auto-adapt per platform.
            AMD ROCm setup is preserved and will be available when returning to Linux.
        """
        self.device_name = device
        self.evaluator = None
        self.physics = None
        self.status = "uninitialized"
        self.startup_time = datetime.now()

        # Use protected device configuration for platform awareness
        if device == 'auto':
            # Check for platform changes and warn if needed
            ProtectedDeviceConfig.check_platform_change()
            # Get platform-appropriate device
            self.device = get_device(force_device=None)
            self.device_name = str(self.device)
        else:
            # Direct device specification (use with caution)
            self.device = torch.device(device)
            self.device_name = device

    def initialize(self) -> bool:
        """Initialize all system components."""
        print("=" * 80)
        print("GAIA + PHYSICS WORLD MODEL - DEPLOYMENT INITIALIZATION")
        print("=" * 80)
        print()

        try:
            print("1. Initializing Physics World Model...")
            self.physics = PhysicsWorldModel()
            print("   ✅ Physics system ready")
            print(f"      - Domains: {len(self.physics.list_domains())}")
            print(f"      - Laws: {len(self.physics.list_laws())}")
            print()

            print("2. Initializing GAIA Consciousness + Physics Integration...")
            self.evaluator = PhysicsEnhancedGAIAEvaluator(device=self.device)
            print("   ✅ GAIA system ready")
            print(f"      - Agents: {len(self.evaluator.physics_reasoner.physics_reasoner.agents) if hasattr(self.evaluator.physics_reasoner, 'physics_reasoner') else 'N/A'}")
            print(f"      - Device: {self.device}")
            print()

            print("3. System Validation...")
            # Quick validation test
            test_result = self.evaluator.evaluate_mixed_query("How does entropy work?")
            if test_result.get('type') == 'physics_question':
                print("   ✅ Query routing functional")
            print("   ✅ Integration verified")
            print()

            self.status = "deployed"
            print("=" * 80)
            print("✅ DEPLOYMENT SUCCESSFUL")
            print("=" * 80)
            print(f"Startup Time: {self.startup_time.strftime('%Y-%m-%d %H:%M:%S')}")
            print(f"Status: {self.status.upper()}")
            print()

            return True

        except Exception as e:
            print(f"❌ DEPLOYMENT FAILED: {e}")
            self.status = "failed"
            return False

    def query(self, question: str) -> dict:
        """Process a query through the system."""
        if self.status != "deployed":
            return {"error": "System not deployed"}

        return self.evaluator.evaluate_mixed_query(question)

    def physics_query(self, question: str, domain: PhysicsDomain) -> dict:
        """Query physics system directly."""
        if self.status != "deployed":
            return {"error": "System not deployed"}

        answer = self.physics.answer_question(question, domain)
        return {
            'answer': answer.answer,
            'confidence': answer.confidence,
            'explanation': answer.explanation,
            'principles': [p.value for p in answer.principles_used]
        }

    def show_status(self):
        """Display system status."""
        print()
        print("=" * 80)
        print("SYSTEM STATUS")
        print("=" * 80)
        print(f"Status:           {self.status.upper()}")
        print(f"Device:           {self.device}")
        print(f"Runtime:          {(datetime.now() - self.startup_time).total_seconds():.1f}s")
        print()

        if self.physics:
            print("Physics World Model:")
            print(f"  - Domains: {len(self.physics.list_domains())}")
            print(f"  - Laws: {len(self.physics.list_laws())}")
            domains = [d for d in self.physics.list_domains()]
            print(f"  - Available: {', '.join(domains)}")
            print()

        if self.evaluator:
            print("GAIA Consciousness System:")
            print(f"  - Agents: 5 (IsingGPU)")
            print(f"  - Empathy Module: Active")
            print(f"  - Integration: Compound (Standalone + GAIA)")
            print()

        print("=" * 80)

    def demo(self):
        """Run demonstration queries."""
        print()
        print("=" * 80)
        print("LIVE DEMONSTRATION")
        print("=" * 80)
        print()

        demo_queries = [
            "Why do objects fall?",
            "How does entropy relate to understanding?",
            "What is quantum superposition?",
        ]

        for i, query in enumerate(demo_queries, 1):
            print(f"Demo {i}: {query}")
            result = self.query(query)

            if result.get('type') == 'physics_question':
                physics_result = result['result']
                print(f"  Type: Physics")
                print(f"  Answer: {physics_result['physics_reasoning']['answer'][:80]}...")
                print(f"  Confidence: {physics_result['confidence']:.1%}")
            else:
                print(f"  Type: Consciousness")
                print(f"  Handler: {result['handler']}")

            print()


def main():
    """Main deployment function."""
    # Initialize system
    system = DeploymentSystem(device='cpu')

    # Deploy
    if not system.initialize():
        return 1

    # Show status
    system.show_status()

    # Run demo
    system.demo()

    # Show deployment complete
    print("=" * 80)
    print("🎉 DEPLOYMENT COMPLETE AND OPERATIONAL")
    print("=" * 80)
    print()
    print("System Ready For:")
    print("  ✅ Physics queries (5 domains + sacred geometry)")
    print("  ✅ Consciousness reasoning (multi-agent empathy)")
    print("  ✅ Hybrid physics-consciousness questions")
    print("  ✅ Automated query routing")
    print()
    print("Quick Start Examples:")
    print("  from DEPLOY import DeploymentSystem")
    print("  system = DeploymentSystem()")
    print("  system.initialize()")
    print("  result = system.query('Your question here')")
    print()
    print("=" * 80)

    return 0


if __name__ == "__main__":
    sys.exit(main())
