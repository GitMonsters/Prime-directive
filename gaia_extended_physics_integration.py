#!/usr/bin/env python3
"""
GAIA Extended Physics Integration

Integrates the extended physics domains and advanced reasoning capabilities
with the GAIA consciousness system.

Features:
- Query routing to appropriate physics domain
- Cross-domain reasoning
- Hybrid physics-consciousness answers
- Confidence quantification
- Reasoning explanation
"""

from typing import Dict, List, Optional, Tuple
from physics_extended_domains import (
    ExtendedPhysicsKnowledgeBase,
    AdvancedPhysicsReasoner,
    ExtendedPhysicsDomain,
    ExtendedPhysicalPrinciple
)
import torch


class ExtendedPhysicsQueryRouter:
    """Routes queries to appropriate physics domain and reasoning method."""

    def __init__(self):
        self.kb = ExtendedPhysicsKnowledgeBase()
        self.reasoner = AdvancedPhysicsReasoner(self.kb)

        # Domain keywords for routing
        self.domain_keywords = {
            ExtendedPhysicsDomain.RELATIVITY: [
                "relativity", "spacetime", "gravity", "black hole", "light cone",
                "time dilation", "length contraction", "equivalence", "geodesic"
            ],
            ExtendedPhysicsDomain.FLUID_DYNAMICS: [
                "fluid", "flow", "viscosity", "turbulence", "bernoulli", "streamline",
                "vortex", "aerodynamics", "hydrodynamics", "lift"
            ],
            ExtendedPhysicsDomain.QUANTUM_FIELD_THEORY: [
                "quantum field", "qft", "virtual particle", "gauge", "lagrangian",
                "propagator", "renormalization", "yang-mills", "feynman"
            ],
            ExtendedPhysicsDomain.COSMOLOGY: [
                "universe", "cosmic", "expansion", "big bang", "inflation", "hubble",
                "dark energy", "dark matter", "nucleosynthesis", "cmb"
            ],
            ExtendedPhysicsDomain.PARTICLE_PHYSICS: [
                "particle", "standard model", "decay", "interaction", "quark", "lepton",
                "boson", "higgs", "electroweak", "symmetry breaking"
            ],
            ExtendedPhysicsDomain.OPTICS: [
                "light", "optical", "interference", "diffraction", "polarization",
                "refraction", "lens", "photon", "wavelength", "prism"
            ],
            ExtendedPhysicsDomain.ACOUSTICS: [
                "sound", "acoustic", "doppler", "resonance", "frequency", "wave",
                "decibel", "echo", "ultrasound", "vibration"
            ],
            ExtendedPhysicsDomain.STATISTICAL_MECHANICS: [
                "statistical", "entropy", "distribution", "phase transition",
                "boltzmann", "partition function", "ensemble", "critical phenomena"
            ],
            ExtendedPhysicsDomain.PLASMA_PHYSICS: [
                "plasma", "ionization", "magnetic confinement", "fusion", "discharge",
                "magnetosphere", "solar wind", "tokamak"
            ],
            ExtendedPhysicsDomain.ASTROPHYSICS: [
                "astrophysics", "star", "stellar evolution", "neutron star",
                "accretion", "supernova", "galaxy", "pulsar", "x-ray"
            ]
        }

    def detect_domain(self, query: str) -> Tuple[ExtendedPhysicsDomain, float]:
        """
        Detect which domain a query belongs to.
        Returns (domain, confidence).
        """
        query_lower = query.lower()
        scores = {}

        # Score each domain based on keyword matches
        for domain, keywords in self.domain_keywords.items():
            score = sum(1.0 for kw in keywords if kw in query_lower)
            if score > 0:
                scores[domain] = score / len(keywords)

        if not scores:
            # Default to relativity if no match
            return ExtendedPhysicsDomain.RELATIVITY, 0.5

        best_domain = max(scores, key=scores.get)
        confidence = scores[best_domain]
        return best_domain, confidence

    def detect_reasoning_type(self, query: str) -> str:
        """
        Detect what type of reasoning is needed.
        Returns: "explanation", "prediction", "causal", "cross_domain", "uncertainty"
        """
        query_lower = query.lower()

        if any(w in query_lower for w in ["why", "how", "explain", "understand"]):
            return "explanation"
        elif any(w in query_lower for w in ["predict", "what will", "happen", "future", "outcome"]):
            return "prediction"
        elif any(w in query_lower for w in ["cause", "caused", "because", "effect", "result"]):
            return "causal"
        elif any(w in query_lower for w in ["relate", "analogy", "similar", "between", "connection"]):
            return "cross_domain"
        elif any(w in query_lower for w in ["uncertainty", "error", "precision", "how accurate"]):
            return "uncertainty"

        return "explanation"  # Default

    def route_query(self, query: str) -> Dict:
        """
        Route a query to appropriate reasoning method.
        Returns routing information and handler function.
        """
        domain, domain_confidence = self.detect_domain(query)
        reasoning_type = self.detect_reasoning_type(query)

        routing = {
            'query': query,
            'domain': domain.value,
            'domain_confidence': domain_confidence,
            'reasoning_type': reasoning_type,
            'handler': self._get_handler(reasoning_type),
            'description': f"Route to {domain.value} via {reasoning_type} reasoning"
        }

        return routing

    def _get_handler(self, reasoning_type: str) -> str:
        """Get the appropriate handler function name."""
        handlers = {
            'explanation': 'explain_phenomenon',
            'prediction': 'predict_outcome',
            'causal': 'reasoning_causal',
            'cross_domain': 'cross_domain_inference',
            'uncertainty': 'uncertainty_quantification'
        }
        return handlers.get(reasoning_type, 'explain_phenomenon')

    def explain_phenomenon(self, query: str, domain: ExtendedPhysicsDomain) -> Dict:
        """Explain a physical phenomenon."""
        laws = self.kb.get_laws_by_domain(domain)

        if not laws:
            return {
                'type': 'explanation',
                'domain': domain.value,
                'answer': f"Limited knowledge about {domain.value} in knowledge base.",
                'confidence': 0.3,
                'principles': []
            }

        # Select relevant laws (top 2-3 most applicable)
        relevant_laws = laws[:min(3, len(laws))]

        explanation = f"In {domain.value}:\n"
        for law in relevant_laws:
            explanation += f"\n• {law.name}: {law.equation}\n  {law.description}"

        return {
            'type': 'explanation',
            'domain': domain.value,
            'answer': explanation,
            'confidence': 0.8,
            'principles': [law.principle.value for law in relevant_laws],
            'laws_cited': [law.name for law in relevant_laws]
        }

    def predict_outcome(self, query: str, domain: ExtendedPhysicsDomain) -> Dict:
        """Predict physical outcomes."""
        result = self.reasoner.predict_outcome(
            initial_conditions={},
            domain=domain,
            time_scale="medium"
        )
        return {
            'type': 'prediction',
            'domain': domain.value,
            'answer': "\n".join([f"• {p}" for p in result['predictions']]),
            'confidence': result['confidence'],
            'uncertainties': result['uncertainties']
        }

    def cross_domain_inference(self, query: str, domain: ExtendedPhysicsDomain) -> Dict:
        """Infer across domains."""
        # For simplicity, relate to particle physics
        target = ExtendedPhysicsDomain.PARTICLE_PHYSICS

        result = self.reasoner.cross_domain_inference(
            query=query,
            source_domain=domain,
            target_domain=target
        )

        return {
            'type': 'cross_domain',
            'source_domain': domain.value,
            'target_domain': target.value,
            'answer': result.get('inference', 'No direct analogy found'),
            'confidence': result['confidence'],
            'analogy': result.get('analogy')
        }

    def reasoning_causal(self, query: str, domain: ExtendedPhysicsDomain) -> Dict:
        """Reason about causality."""
        # Extract cause (simple approach)
        cause = "mass" if "mass" in query.lower() else "energy"

        result = self.reasoner.causal_reasoning(
            cause=cause,
            domain=domain
        )

        return {
            'type': 'causal',
            'domain': domain.value,
            'cause': result['cause'],
            'answer': "Effect chain:\n" + "\n".join([f"  {i}. {e}" for i, e in enumerate(result['effects'], 1)]),
            'confidence': result['confidence'],
            'effect_chain': result['effects']
        }

    def uncertainty_quantification(self, query: str, domain: ExtendedPhysicsDomain) -> Dict:
        """Quantify uncertainties."""
        # Extract measurement (simple approach)
        measurement = "hubble_constant" if "hubble" in query.lower() else "coupling_constant"

        result = self.reasoner.uncertainty_quantification(
            measurement=measurement,
            domain=domain
        )

        if 'uncertainty_percent' in result:
            answer = (f"Measurement: {result['measurement']}\n"
                     f"Uncertainty: ±{result['uncertainty_percent']}%\n"
                     f"Confidence: {result['confidence']:.1%}\n\n"
                     f"Sources:\n" +
                     "\n".join([f"  • {s}" for s in result['uncertainty_sources']]))
        else:
            answer = result['status']

        return {
            'type': 'uncertainty',
            'domain': domain.value,
            'measurement': result.get('measurement'),
            'answer': answer,
            'confidence': result.get('confidence', 0.5),
            'uncertainty_percent': result.get('uncertainty_percent')
        }


class GAIAExtendedPhysicsInterface:
    """Bridge between GAIA consciousness and extended physics systems."""

    def __init__(self):
        self.router = ExtendedPhysicsQueryRouter()
        self.query_history = []

    def process_physics_query(self, query: str, gaia_context: Optional[Dict] = None) -> Dict:
        """
        Process a physics query with extended domains.

        Args:
            query: The physics question
            gaia_context: Optional context from GAIA (empathy scores, agent states, etc.)

        Returns:
            Comprehensive physics answer with confidence and reasoning
        """
        # Route the query
        routing = self.router.route_query(query)

        # Get domain and reasoning type
        domain = ExtendedPhysicsDomain(routing['domain'])
        reasoning_type = routing['reasoning_type']

        # Execute appropriate handler
        handler_name = routing['handler']
        handler = getattr(self.router, handler_name, self.router.explain_phenomenon)
        result = handler(query, domain)

        # Store in history
        self.query_history.append({
            'query': query,
            'routing': routing,
            'result': result
        })

        # Enhance with GAIA context if available
        if gaia_context:
            result['gaia_context'] = {
                'empathy_score': gaia_context.get('empathy_score'),
                'agent_coherence': gaia_context.get('coherence'),
                'multi_agent_agreement': gaia_context.get('agreement')
            }

        return {
            'query': query,
            'type': 'extended_physics',
            'routing': routing,
            'physics_answer': result,
            'query_timestamp': len(self.query_history)
        }

    def batch_process_queries(self, queries: List[str]) -> List[Dict]:
        """Process multiple queries efficiently."""
        return [self.process_physics_query(q) for q in queries]

    def get_domain_capabilities(self, domain: str) -> Dict:
        """Get capabilities for a specific domain."""
        try:
            domain_enum = ExtendedPhysicsDomain(domain)
        except ValueError:
            return {'error': f'Unknown domain: {domain}'}

        laws = self.router.kb.get_laws_by_domain(domain_enum)
        related = self.router.kb.get_related_domains(domain_enum)

        return {
            'domain': domain,
            'laws_count': len(laws),
            'laws': [law.name for law in laws],
            'principles': list(set(law.principle.value for law in laws)),
            'related_domains': [d.value for d in related],
            'reasoning_capabilities': [
                'explanation',
                'prediction',
                'causal_analysis',
                'cross_domain_inference',
                'uncertainty_quantification'
            ]
        }

    def explain_analogy(self, domain1: str, domain2: str) -> Dict:
        """Explain analogy between two domains."""
        analogy = self.router.kb.get_analogy(domain1, domain2)

        return {
            'domain1': domain1,
            'domain2': domain2,
            'analogy': analogy if analogy else 'No direct analogy found',
            'confidence': 0.75 if analogy else 0.0
        }


# ============================================================================
# DEMO
# ============================================================================

if __name__ == '__main__':
    print("=" * 80)
    print("GAIA EXTENDED PHYSICS INTEGRATION - Demo")
    print("=" * 80)
    print()

    interface = GAIAExtendedPhysicsInterface()

    # Example queries
    queries = [
        "Why does mass curve spacetime?",
        "What is the fate of an expanding universe?",
        "How do particles interact in the Standard Model?",
        "Explain the relationship between relativity and particle physics",
        "What is the uncertainty in the Hubble constant measurement?"
    ]

    print("Processing Example Queries:")
    print("=" * 80)

    for i, query in enumerate(queries, 1):
        print(f"\n{i}. Query: {query}")
        result = interface.process_physics_query(query)

        print(f"   Domain: {result['routing']['domain']}")
        print(f"   Type: {result['routing']['reasoning_type']}")
        print(f"   Confidence: {result['physics_answer']['confidence']:.1%}")
        answer = result['physics_answer']['answer']
        # Print first 200 chars of answer
        print(f"   Answer: {answer[:200]}..." if len(answer) > 200 else f"   Answer: {answer}")

    print("\n" + "=" * 80)
    print(f"Processed {len(interface.query_history)} queries")
    print("=" * 80)

    # Show domain capabilities
    print("\nDomain Capabilities (Relativity):")
    print("=" * 80)
    caps = interface.get_domain_capabilities('relativity')
    print(f"Laws: {len(caps['laws'])}")
    for law in caps['laws'][:3]:
        print(f"  • {law}")
    print(f"\nRelated domains: {', '.join(caps['related_domains'])}")
    print(f"Reasoning capabilities: {', '.join(caps['reasoning_capabilities'])}")
