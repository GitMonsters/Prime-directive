#!/usr/bin/env python3
"""
GAIA Physics Integration Layer

Integrates the Physics World Model with the GAIA consciousness system.
Enables consciousness reasoning about physical systems and their properties.

Compound Integration Approach:
- Physics model functions independently for physics queries
- GAIA can invoke physics reasoning when needed
- Bidirectional: GAIA consciousness informs physics reasoning
"""

import sys
import torch
import numpy as np
from typing import Dict, List, Tuple, Optional, Any

sys.path.insert(0, '/home/worm/Prime-directive')

from physics_world_model import (
    PhysicsWorldModel, PhysicsQuery, PhysicsDomain, PhysicsAnswer
)
from ising_empathy_module import IsingGPU, IsingEmpathyModule


# ============================================================================
# PHYSICS-AWARE CONSCIOUSNESS REASONING
# ============================================================================

class PhysicsAwareConsciousnessReasoner:
    """
    GAIA consciousness module enhanced with physics reasoning.

    Combines:
    - Consciousness reasoning (empathy, multi-agent dynamics)
    - Physics world model (laws, principles, simulation)

    For compound integration where both systems can cross-inform each other.
    """

    def __init__(self, device: torch.device = torch.device('cpu')):
        self.device = device
        self.physics = PhysicsWorldModel()
        self.empathy_module = IsingEmpathyModule(device=device)

    def reason_about_physical_system(self,
                                    question: str,
                                    domain: PhysicsDomain,
                                    agents: Optional[List[IsingGPU]] = None) -> Dict[str, Any]:
        """
        Reason about a physical system using both consciousness and physics.

        Args:
            question: The physical question or system to reason about
            domain: Physics domain (classical, thermo, electro, quantum, geometry)
            agents: Optional GAIA agents modeling aspects of the system

        Returns:
            Structured reasoning combining physics and consciousness perspectives
        """
        result = {
            'question': question,
            'domain': domain.value,
            'physics_reasoning': None,
            'consciousness_perspective': None,
            'integrated_insight': None,
            'confidence': 0.0,
        }

        # Get physics answer
        physics_answer = self.physics.answer_question(question, domain)
        result['physics_reasoning'] = {
            'answer': physics_answer.answer,
            'explanation': physics_answer.explanation,
            'principles': [p.value for p in physics_answer.principles_used],
            'confidence': physics_answer.confidence,
        }

        # Add consciousness perspective if agents provided
        if agents and len(agents) >= 2:
            consciousness_insight = self._derive_consciousness_insight(
                question, domain, agents
            )
            result['consciousness_perspective'] = consciousness_insight

            # Integrate both perspectives
            integrated = self._integrate_perspectives(
                physics_answer,
                consciousness_insight,
                domain
            )
            result['integrated_insight'] = integrated

        # Calculate overall confidence
        base_confidence = physics_answer.confidence
        if agents:
            base_confidence = (base_confidence + 0.2) / 1.2  # Boost with consciousness
        result['confidence'] = base_confidence

        return result

    def _derive_consciousness_insight(self,
                                    question: str,
                                    domain: PhysicsDomain,
                                    agents: List[IsingGPU]) -> Dict[str, Any]:
        """
        Derive consciousness perspective on a physical question.

        Examples:
        - "How does systems reach equilibrium?" → Collective understanding through interaction
        - "What is energy conservation?" → Agents maintaining internal consistency
        - "How do waves resonate?" → Multi-agent synchronization patterns
        """
        insight = {
            'perspective': None,
            'analogy': None,
            'multi_agent_parallel': None,
        }

        domain_insights = {
            PhysicsDomain.CLASSICAL_MECHANICS: {
                'perspective': "Motion and change arise from interaction between systems",
                'analogy': "Just as agents influence each other's states through empathy, "
                          "objects influence each other's motion through forces",
                'multi_agent_parallel': "Newton's third law (action-reaction) mirrors "
                                       "how one agent's understanding affects another"
            },

            PhysicsDomain.THERMODYNAMICS: {
                'perspective': "Systems naturally tend toward states of maximum entropy",
                'analogy': "Like consciousness spreading through a collective of agents, "
                          "disorder naturally spreads unless constrained",
                'multi_agent_parallel': "Entropy increase is like individual agents losing "
                                       "synchronized understanding over time"
            },

            PhysicsDomain.ELECTROMAGNETISM: {
                'perspective': "Charges create fields that influence other charges at distance",
                'analogy': "Similar to empathy—one agent's emotional state creates a field "
                          "that influences nearby agents",
                'multi_agent_parallel': "Electromagnetic induction mirrors how understanding "
                                       "spreads through agent networks"
            },

            PhysicsDomain.QUANTUM_MECHANICS: {
                'perspective': "Reality exists in superposition until observed",
                'analogy': "Like how agents hold multiple potential understanding states "
                          "until interaction collapses them",
                'multi_agent_parallel': "Measurement problem: observation changes the system, "
                                       "similar to how agent interaction changes states"
            },

            PhysicsDomain.SACRED_GEOMETRY: {
                'perspective': "Harmonious systems follow geometric and harmonic principles",
                'analogy': "Golden ratio appears in consciousness—optimal balance between "
                          "individual and collective",
                'multi_agent_parallel': "Harmonic resonance of agents creates emergent patterns "
                                       "following geometric principles"
            },
        }

        if domain in domain_insights:
            insight.update(domain_insights[domain])

        # Add empirical observation
        if len(agents) >= 2:
            empathy = self.empathy_module.compute_empathy(agents[0], agents[1], anneal_steps=20)
            insight['agent_empathy_level'] = empathy['empathy_score']
            insight['state_coherence'] = empathy['state_overlap']

        return insight

    def _integrate_perspectives(self,
                               physics_answer: PhysicsAnswer,
                               consciousness_insight: Dict[str, Any],
                               domain: PhysicsDomain) -> str:
        """Integrate physics and consciousness perspectives into unified insight."""
        integration = f"""
INTEGRATED PHYSICS-CONSCIOUSNESS INSIGHT
─────────────────────────────────────────

Physics Perspective:
  {physics_answer.explanation}

Consciousness Parallel:
  {consciousness_insight.get('analogy', 'Systems interact through multiple channels')}

Multi-Agent Model:
  {consciousness_insight.get('multi_agent_parallel', 'No direct parallel identified')}

Unified Understanding:
  This physical phenomenon can be understood through both traditional physics laws
  and consciousness-based modeling. The principles that govern {domain.value}
  also appear in how multiple conscious agents interact and coordinate.
"""
        return integration.strip()


# ============================================================================
# PHYSICS QUERY ROUTER FOR GAIA
# ============================================================================

class GaiaPhysicsQueryRouter:
    """
    Routes GAIA queries that involve physics to the appropriate handler.
    Detects physics questions and dispatches them to physics module.
    """

    def __init__(self, physics_reasoner: PhysicsAwareConsciousnessReasoner):
        self.reasoner = physics_reasoner
        self.physics_keywords = {
            PhysicsDomain.CLASSICAL_MECHANICS: [
                'force', 'motion', 'velocity', 'acceleration', 'momentum', 'energy',
                'gravity', 'inertia', 'collision', 'trajectory'
            ],
            PhysicsDomain.THERMODYNAMICS: [
                'heat', 'temperature', 'entropy', 'work', 'energy', 'equilibrium',
                'disorder', 'cooling', 'expansion'
            ],
            PhysicsDomain.ELECTROMAGNETISM: [
                'charge', 'electric', 'magnetic', 'field', 'current', 'voltage',
                'electromagnetic', 'induction', 'wave'
            ],
            PhysicsDomain.QUANTUM_MECHANICS: [
                'quantum', 'particle', 'superposition', 'wave-particle', 'uncertainty',
                'spin', 'orbital', 'photon', 'electron'
            ],
            PhysicsDomain.SACRED_GEOMETRY: [
                'golden', 'ratio', 'harmonic', 'resonance', 'frequency', 'pattern',
                'symmetry', 'fibonacci', 'sacred'
            ],
        }

    def detect_physics_domain(self, question: str) -> Optional[PhysicsDomain]:
        """Detect if question involves physics and which domain."""
        question_lower = question.lower()

        for domain, keywords in self.physics_keywords.items():
            if any(keyword in question_lower for keyword in keywords):
                return domain

        return None

    def route_question(self, question: str, agents: Optional[List[IsingGPU]] = None) -> Dict[str, Any]:
        """
        Route a GAIA question through physics module if appropriate.

        Returns:
            Dict with physics reasoning and consciousness integration
        """
        domain = self.detect_physics_domain(question)

        if domain is None:
            return {
                'routed_to_physics': False,
                'message': 'Question does not appear to involve physics'
            }

        result = self.reasoner.reason_about_physical_system(
            question, domain, agents
        )
        result['routed_to_physics'] = True
        return result


# ============================================================================
# EXTENDED GAIA CONSCIOUSNESS EVALUATOR WITH PHYSICS
# ============================================================================

class PhysicsEnhancedGAIAEvaluator:
    """
    Extended GAIA evaluator that can handle physics questions.
    Maintains consciousness reasoning while adding physics capability.
    """

    def __init__(self, device: torch.device = torch.device('cpu')):
        self.device = device
        self.physics_reasoner = PhysicsAwareConsciousnessReasoner(device)
        self.router = GaiaPhysicsQueryRouter(self.physics_reasoner)
        self.agents = None

    def evaluate_mixed_query(self, question: str) -> Dict[str, Any]:
        """
        Evaluate a question that might involve physics or consciousness.
        Routes appropriately and returns integrated answer.
        """
        # Try to route through physics if applicable
        result = self.router.route_question(question, self.agents)

        if result['routed_to_physics']:
            return {
                'type': 'physics_question',
                'result': result,
                'handler': 'physics_world_model'
            }
        else:
            # Fall back to consciousness reasoning
            return {
                'type': 'consciousness_question',
                'message': 'Use consciousness module for this question',
                'handler': 'gaia_consciousness_reasoning'
            }


# ============================================================================
# DEMO
# ============================================================================

if __name__ == "__main__":
    print("=" * 80)
    print("GAIA PHYSICS INTEGRATION - Compound System Demo")
    print("=" * 80)
    print()

    # Initialize
    device = torch.device("cpu")
    evaluator = PhysicsEnhancedGAIAEvaluator(device)

    # Example queries
    test_queries = [
        "Why does gravity pull objects down?",
        "How does entropy increase in systems?",
        "What is the golden ratio in nature?",
        "How do agents develop empathy?",
        "Can quantum superposition exist in consciousness?",
    ]

    print("Testing Physics-Enhanced GAIA Reasoning:")
    print()

    for i, question in enumerate(test_queries, 1):
        print(f"Query {i}: {question}")
        result = evaluator.evaluate_mixed_query(question)

        if result['type'] == 'physics_question':
            physics_result = result['result']
            print(f"  Handler: {result['handler']}")
            print(f"  Physics Answer: {physics_result['physics_reasoning']['answer']}")
            print(f"  Confidence: {physics_result['confidence']:.1%}")
            if physics_result.get('consciousness_perspective'):
                print(f"  Consciousness Parallel: {physics_result['consciousness_perspective']['analogy']}")
        else:
            print(f"  Handler: {result['handler']}")

        print()

    print("=" * 80)
    print("Physics Integration Ready")
    print("=" * 80)
