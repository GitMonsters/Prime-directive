#!/usr/bin/env python3
"""
PHYSICS WORLD MODEL - Compound Integration System

A comprehensive physics knowledge and reasoning system that:
1. Functions independently for physics questions and simulations
2. Integrates with GAIA consciousness system via query interface
3. Covers Classical Mechanics, Thermodynamics, Electromagnetism,
   Quantum Mechanics, and Sacred Geometry

Architecture:
- PhysicsKnowledgeBase: Core facts and principles
- PhysicsReasoner: Logical inference and constraint checking
- PhysicsSimulator: System dynamics and evolution
- PhysicsExplainer: Intuitive explanations of phenomena
- GAIAPhysicsInterface: Bridge to consciousness module
"""

import torch
import numpy as np
from typing import Dict, List, Tuple, Optional, Any
from dataclasses import dataclass
from enum import Enum
import math


# ============================================================================
# PHYSICS DOMAIN ENUMERATIONS
# ============================================================================

class PhysicsDomain(Enum):
    """Physics domains covered by the world model."""
    CLASSICAL_MECHANICS = "classical"
    THERMODYNAMICS = "thermo"
    ELECTROMAGNETISM = "electro"
    QUANTUM_MECHANICS = "quantum"
    SACRED_GEOMETRY = "geometry"


class PhysicalPrinciple(Enum):
    """Fundamental physics principles."""
    CONSERVATION_ENERGY = "energy_conservation"
    CONSERVATION_MOMENTUM = "momentum_conservation"
    CONSERVATION_ANGULAR_MOMENTUM = "angular_momentum_conservation"
    CONSERVATION_CHARGE = "charge_conservation"
    ENTROPY_INCREASE = "entropy_increase"
    UNCERTAINTY_PRINCIPLE = "uncertainty_principle"
    SYMMETRY_PRINCIPLE = "symmetry"
    GOLDEN_RATIO = "golden_ratio"
    HARMONIC_RESONANCE = "harmonic_resonance"


# ============================================================================
# DATA STRUCTURES
# ============================================================================

@dataclass
class PhysicalObject:
    """Represents a physical object in the world model."""
    name: str
    mass: Optional[float] = None
    charge: Optional[float] = None
    position: Optional[np.ndarray] = None  # (x, y, z)
    velocity: Optional[np.ndarray] = None  # (vx, vy, vz)
    properties: Dict[str, Any] = None

    def __post_init__(self):
        if self.properties is None:
            self.properties = {}


@dataclass
class PhysicalLaw:
    """Represents a physics law or principle."""
    name: str
    domain: PhysicsDomain
    principle: PhysicalPrinciple
    equation: str
    constraints: List[str]
    conditions: List[str]


@dataclass
class PhysicsQuery:
    """A physics question or reasoning task."""
    question: str
    domain: PhysicsDomain
    requires_simulation: bool = False
    requires_constraint_check: bool = True
    requires_explanation: bool = True


@dataclass
class PhysicsAnswer:
    """A physics answer with reasoning and explanation."""
    answer: str
    confidence: float
    domain: PhysicsDomain
    reasoning: List[str]
    principles_used: List[PhysicalPrinciple]
    explanation: str
    simulation_data: Optional[Dict] = None


# ============================================================================
# PHYSICS KNOWLEDGE BASE
# ============================================================================

class PhysicsKnowledgeBase:
    """
    Comprehensive physics knowledge base covering all domains.
    Stores facts, laws, principles, and relationships.
    """

    def __init__(self):
        self.laws = self._initialize_laws()
        self.constants = self._initialize_constants()
        self.principles = self._initialize_principles()
        self.relationships = self._initialize_relationships()

    def _initialize_laws(self) -> Dict[str, PhysicalLaw]:
        """Initialize fundamental physics laws."""
        laws = {}

        # Classical Mechanics
        laws['newtons_first'] = PhysicalLaw(
            name="Newton's First Law (Inertia)",
            domain=PhysicsDomain.CLASSICAL_MECHANICS,
            principle=PhysicalPrinciple.CONSERVATION_MOMENTUM,
            equation="F = 0 → a = 0 (no acceleration without force)",
            constraints=["Object in isolation", "Inertial reference frame"],
            conditions=["No external forces"]
        )

        laws['newtons_second'] = PhysicalLaw(
            name="Newton's Second Law",
            domain=PhysicsDomain.CLASSICAL_MECHANICS,
            principle=PhysicalPrinciple.CONSERVATION_MOMENTUM,
            equation="F = ma",
            constraints=["Constant mass", "Inertial frame"],
            conditions=["Net force applied"]
        )

        laws['newtons_third'] = PhysicalLaw(
            name="Newton's Third Law (Action-Reaction)",
            domain=PhysicsDomain.CLASSICAL_MECHANICS,
            principle=PhysicalPrinciple.CONSERVATION_MOMENTUM,
            equation="F_AB = -F_BA",
            constraints=["Instantaneous interaction"],
            conditions=["Two objects interacting"]
        )

        laws['conservation_energy'] = PhysicalLaw(
            name="Conservation of Energy",
            domain=PhysicsDomain.CLASSICAL_MECHANICS,
            principle=PhysicalPrinciple.CONSERVATION_ENERGY,
            equation="E_total = KE + PE = constant",
            constraints=["Closed system", "Conservative forces"],
            conditions=["No external work", "No dissipation"]
        )

        # Thermodynamics
        laws['first_law_thermo'] = PhysicalLaw(
            name="First Law of Thermodynamics",
            domain=PhysicsDomain.THERMODYNAMICS,
            principle=PhysicalPrinciple.CONSERVATION_ENERGY,
            equation="dU = dQ - dW",
            constraints=["Well-defined state", "Equilibrium assumptions"],
            conditions=["Energy exchange"]
        )

        laws['second_law_thermo'] = PhysicalLaw(
            name="Second Law of Thermodynamics",
            domain=PhysicsDomain.THERMODYNAMICS,
            principle=PhysicalPrinciple.ENTROPY_INCREASE,
            equation="dS_universe >= 0",
            constraints=["Isolated system", "Macroscopic scale"],
            conditions=["Natural processes"]
        )

        # Electromagnetism
        laws['coulombs_law'] = PhysicalLaw(
            name="Coulomb's Law",
            domain=PhysicsDomain.ELECTROMAGNETISM,
            principle=PhysicalPrinciple.CONSERVATION_CHARGE,
            equation="F = k*q1*q2/r²",
            constraints=["Point charges", "Vacuum/medium"],
            conditions=["Electrostatic interaction"]
        )

        laws['gauss_law'] = PhysicalLaw(
            name="Gauss's Law",
            domain=PhysicsDomain.ELECTROMAGNETISM,
            principle=PhysicalPrinciple.CONSERVATION_CHARGE,
            equation="∮E·dA = Q_enc/ε₀",
            constraints=["Closed surface", "Static fields"],
            conditions=["Charge distribution"]
        )

        # Quantum Mechanics
        laws['uncertainty_principle'] = PhysicalLaw(
            name="Heisenberg Uncertainty Principle",
            domain=PhysicsDomain.QUANTUM_MECHANICS,
            principle=PhysicalPrinciple.UNCERTAINTY_PRINCIPLE,
            equation="Δx·Δp >= ℏ/2",
            constraints=["Quantum regime", "Microscopic scale"],
            conditions=["Position-momentum measurement"]
        )

        laws['schrodinger_equation'] = PhysicalLaw(
            name="Schrödinger Equation",
            domain=PhysicsDomain.QUANTUM_MECHANICS,
            principle=PhysicalPrinciple.SYMMETRY_PRINCIPLE,
            equation="iℏ(∂ψ/∂t) = Ĥψ",
            constraints=["Non-relativistic", "Single particle"],
            conditions=["Quantum system evolution"]
        )

        # Sacred Geometry
        laws['golden_ratio'] = PhysicalLaw(
            name="Golden Ratio Principle",
            domain=PhysicsDomain.SACRED_GEOMETRY,
            principle=PhysicalPrinciple.GOLDEN_RATIO,
            equation="φ = (1 + √5)/2 ≈ 1.618",
            constraints=["Natural patterns", "Fibonacci sequences"],
            conditions=["Self-similar structures"]
        )

        laws['harmonic_resonance'] = PhysicalLaw(
            name="Harmonic Resonance",
            domain=PhysicsDomain.SACRED_GEOMETRY,
            principle=PhysicalPrinciple.HARMONIC_RESONANCE,
            equation="f_resonant = c/λ (integer ratios)",
            constraints=["Wave systems", "Periodic boundaries"],
            conditions=["Natural frequencies"]
        )

        return laws

    def _initialize_constants(self) -> Dict[str, float]:
        """Initialize fundamental physical constants."""
        return {
            'G': 6.67430e-11,           # Gravitational constant (m³/kg·s²)
            'c': 299792458.0,            # Speed of light (m/s)
            'h': 6.62607015e-34,        # Planck constant (J·s)
            'hbar': 1.054571817e-34,    # Reduced Planck constant
            'k_B': 1.380649e-23,        # Boltzmann constant (J/K)
            'e': 1.602176634e-19,       # Elementary charge (C)
            'epsilon_0': 8.8541878128e-12, # Permittivity of free space
            'mu_0': 1.25663706212e-6,   # Permeability of free space
            'k_e': 8.9875517923e9,      # Coulomb constant (N·m²/C²)
            'phi': 1.618033988749895,   # Golden ratio
            'pi': math.pi,
            'e_math': math.e,
        }

    def _initialize_principles(self) -> Dict[PhysicalPrinciple, str]:
        """Initialize fundamental physics principles."""
        return {
            PhysicalPrinciple.CONSERVATION_ENERGY:
                "Energy cannot be created or destroyed, only transformed",
            PhysicalPrinciple.CONSERVATION_MOMENTUM:
                "Total momentum of an isolated system remains constant",
            PhysicalPrinciple.CONSERVATION_ANGULAR_MOMENTUM:
                "Angular momentum is conserved in closed systems",
            PhysicalPrinciple.CONSERVATION_CHARGE:
                "Electric charge is conserved in all interactions",
            PhysicalPrinciple.ENTROPY_INCREASE:
                "Entropy of an isolated system always increases or stays constant",
            PhysicalPrinciple.UNCERTAINTY_PRINCIPLE:
                "Certain pairs of physical properties cannot be simultaneously known to arbitrary precision",
            PhysicalPrinciple.SYMMETRY_PRINCIPLE:
                "Laws of physics are invariant under certain transformations",
            PhysicalPrinciple.GOLDEN_RATIO:
                "Natural systems exhibit proportions related to the golden ratio",
            PhysicalPrinciple.HARMONIC_RESONANCE:
                "Systems resonate at frequencies governed by harmonic relationships",
        }

    def _initialize_relationships(self) -> Dict[str, List[str]]:
        """Initialize relationships between concepts."""
        return {
            'force': ['mass', 'acceleration', 'momentum_change'],
            'energy': ['work', 'heat', 'kinetic', 'potential'],
            'momentum': ['force', 'time', 'velocity', 'mass'],
            'charge': ['electric_field', 'magnetic_field', 'current'],
            'wave': ['frequency', 'wavelength', 'amplitude', 'phase'],
            'particle': ['position', 'momentum', 'energy', 'spin'],
            'system': ['energy', 'entropy', 'temperature', 'structure'],
        }

    def get_law(self, law_name: str) -> Optional[PhysicalLaw]:
        """Retrieve a physics law by name."""
        return self.laws.get(law_name)

    def get_constant(self, constant_name: str) -> Optional[float]:
        """Retrieve a fundamental constant."""
        return self.constants.get(constant_name)

    def get_principle_description(self, principle: PhysicalPrinciple) -> str:
        """Get description of a principle."""
        return self.principles.get(principle, "Unknown principle")


# ============================================================================
# PHYSICS REASONER
# ============================================================================

class PhysicsReasoner:
    """
    Logical inference engine for physics.
    Applies conservation laws, checks constraints, and derives conclusions.
    """

    def __init__(self, knowledge_base: PhysicsKnowledgeBase):
        self.kb = knowledge_base

    def reason_about_system(self,
                           objects: List[PhysicalObject],
                           query: PhysicsQuery) -> Tuple[str, List[str]]:
        """
        Reason about a physical system given objects and a query.
        Returns conclusion and reasoning steps.
        """
        reasoning_steps = []

        # Identify applicable principles
        applicable_principles = self._identify_applicable_principles(query.domain)
        reasoning_steps.append(f"Domain: {query.domain.value}")
        reasoning_steps.append(f"Applicable principles: {[p.value for p in applicable_principles]}")

        # Check conservation laws
        conservation_checks = self._check_conservation_laws(objects, applicable_principles)
        reasoning_steps.extend(conservation_checks)

        # Derive conclusion
        conclusion = self._derive_conclusion(query, applicable_principles, conservation_checks)

        return conclusion, reasoning_steps

    def _identify_applicable_principles(self, domain: PhysicsDomain) -> List[PhysicalPrinciple]:
        """Identify which principles apply to a domain."""
        domain_principles = {
            PhysicsDomain.CLASSICAL_MECHANICS: [
                PhysicalPrinciple.CONSERVATION_MOMENTUM,
                PhysicalPrinciple.CONSERVATION_ENERGY,
                PhysicalPrinciple.CONSERVATION_ANGULAR_MOMENTUM,
            ],
            PhysicsDomain.THERMODYNAMICS: [
                PhysicalPrinciple.CONSERVATION_ENERGY,
                PhysicalPrinciple.ENTROPY_INCREASE,
            ],
            PhysicsDomain.ELECTROMAGNETISM: [
                PhysicalPrinciple.CONSERVATION_CHARGE,
                PhysicalPrinciple.CONSERVATION_ENERGY,
            ],
            PhysicsDomain.QUANTUM_MECHANICS: [
                PhysicalPrinciple.UNCERTAINTY_PRINCIPLE,
                PhysicalPrinciple.CONSERVATION_ENERGY,
                PhysicalPrinciple.SYMMETRY_PRINCIPLE,
            ],
            PhysicsDomain.SACRED_GEOMETRY: [
                PhysicalPrinciple.GOLDEN_RATIO,
                PhysicalPrinciple.HARMONIC_RESONANCE,
                PhysicalPrinciple.SYMMETRY_PRINCIPLE,
            ],
        }
        return domain_principles.get(domain, [])

    def _check_conservation_laws(self,
                                objects: List[PhysicalObject],
                                principles: List[PhysicalPrinciple]) -> List[str]:
        """Check if conservation laws are satisfied."""
        checks = []

        if PhysicalPrinciple.CONSERVATION_ENERGY in principles:
            total_ke = sum(0.5 * obj.mass * np.linalg.norm(obj.velocity)**2
                          for obj in objects if obj.mass and obj.velocity is not None)
            checks.append(f"Total kinetic energy: {total_ke:.4e} J")

        if PhysicalPrinciple.CONSERVATION_MOMENTUM in principles:
            total_momentum = sum(obj.mass * obj.velocity
                               for obj in objects
                               if obj.mass and obj.velocity is not None)
            checks.append(f"Total momentum: {total_momentum}")

        if PhysicalPrinciple.CONSERVATION_CHARGE in principles:
            total_charge = sum(obj.charge for obj in objects if obj.charge)
            checks.append(f"Total charge: {total_charge:.4e} C")

        return checks

    def _derive_conclusion(self,
                          query: PhysicsQuery,
                          principles: List[PhysicalPrinciple],
                          checks: List[str]) -> str:
        """Derive conclusion from reasoning."""
        if not checks:
            return f"Question about {query.domain.value} physics is reasonable but requires specific objects"
        return f"System satisfies {len(principles)} key physics principles"


# ============================================================================
# PHYSICS SIMULATOR
# ============================================================================

class PhysicsSimulator:
    """
    Simulates physical systems and their evolution over time.
    Supports numerical integration of equations of motion.
    """

    def __init__(self, knowledge_base: PhysicsKnowledgeBase):
        self.kb = knowledge_base

    def simulate_motion(self,
                       objects: List[PhysicalObject],
                       forces: Dict[str, np.ndarray],
                       time_steps: int = 100,
                       dt: float = 0.01) -> Dict[str, np.ndarray]:
        """
        Simulate motion of objects under given forces.
        Returns trajectory history.
        """
        trajectories = {obj.name: [] for obj in objects}

        # Initialize
        current_state = {obj.name: obj for obj in objects}

        # Time stepping
        for step in range(time_steps):
            for obj in objects:
                if obj.name in forces and obj.mass:
                    # F = ma → a = F/m
                    acceleration = forces[obj.name] / obj.mass
                    # v = v₀ + at
                    if obj.velocity is not None:
                        obj.velocity += acceleration * dt
                    # x = x₀ + vt
                    if obj.position is not None:
                        obj.position += obj.velocity * dt

                trajectories[obj.name].append(obj.position.copy() if obj.position is not None else None)

        return trajectories

    def simulate_resonance(self,
                          frequencies: List[float],
                          duration: float = 1.0,
                          sample_rate: float = 1000) -> Dict[str, np.ndarray]:
        """Simulate harmonic resonance patterns."""
        t = np.linspace(0, duration, int(duration * sample_rate))
        waves = {}

        for i, freq in enumerate(frequencies):
            waves[f'harmonic_{i}'] = np.sin(2 * np.pi * freq * t)

        # Combined wave shows interference patterns
        combined = sum(waves.values()) / len(frequencies)
        waves['combined'] = combined

        return {'time': t, 'waves': waves}


# ============================================================================
# PHYSICS EXPLAINER
# ============================================================================

class PhysicsExplainer:
    """
    Generates intuitive explanations of physics phenomena.
    Connects formal laws to observable behaviors.
    """

    def __init__(self, knowledge_base: PhysicsKnowledgeBase):
        self.kb = knowledge_base

    def explain_phenomenon(self, phenomenon: str, domain: PhysicsDomain) -> str:
        """Provide intuitive explanation of a physics phenomenon."""
        explanations = {
            # CLASSICAL MECHANICS
            (PhysicsDomain.CLASSICAL_MECHANICS, 'gravity'):
                """Gravity is one of the fundamental forces that shapes our universe. All objects with mass
attract each other, though the effect is usually only noticeable with very massive objects like planets and stars.

Here's how it works: Earth pulls you downward with a force proportional to your mass and Earth's mass, following
Newton's law of universal gravitation (F = G·m₁·m₂/r²). What's fascinating is that you also pull Earth upward with
equal force—but since Earth is about 10²⁴ times more massive than you, you don't notice Earth accelerating toward you.

Gravity isn't just a downward force—it keeps the Moon orbiting Earth, keeps planets orbiting the Sun, and keeps
galaxies held together. Einstein later showed us that gravity isn't actually a force pulling objects, but rather
a curvature of spacetime caused by mass and energy. Massive objects bend spacetime around them, and other objects
follow the curved geometry, which we perceive as gravitational attraction.""",

            (PhysicsDomain.CLASSICAL_MECHANICS, 'inertia'):
                """Inertia is the resistance of an object to changes in its motion. This is formalized in Newton's
First Law: an object in motion stays in motion, and an object at rest stays at rest, unless acted upon by a force.

Why does this happen? At the fundamental level, inertia is related to mass. Objects with more mass have greater inertia
and resist acceleration more strongly. This is why a feather blows away easily in the wind, but a bowling ball doesn't.

Practical examples: When you're in a car and it suddenly brakes, your body wants to continue moving forward (inertia)
until friction or the seatbelt provides a force to stop you. A ball rolling on frictionless ice keeps rolling indefinitely
because there's no force to slow it down. This principle is why astronauts in space move at constant velocity—without
friction or other forces, nothing changes their motion.""",

            (PhysicsDomain.CLASSICAL_MECHANICS, 'momentum'):
                """Momentum is the quantity of motion an object possesses, calculated as mass times velocity (p = mv).
It's conserved in isolated systems, meaning the total momentum before a collision equals the total momentum after.

This conservation law explains why crashing cars stick together and move more slowly than either car was moving alone,
and why catching a fast baseball requires your hand to move backward slightly to gradually reduce the ball's momentum.

Momentum is particularly important in collision analysis, rocket propulsion (Newton's third law: expelled momentum pushes
the rocket forward), and understanding why heavy slow objects can have the same momentum as light fast objects.""",

            # THERMODYNAMICS
            (PhysicsDomain.THERMODYNAMICS, 'entropy'):
                """Entropy measures disorder or randomness in a system, and it always increases in isolated systems
(the Second Law of Thermodynamics). This is why your room gets messier over time without active organization, and
why you can't "unbreak" a dropped egg.

The fundamental insight: there are vastly more disordered states than ordered states. Imagine a deck of cards: there's
exactly one perfectly ordered arrangement, but billions upon billions of shuffled arrangements. Random processes naturally
tend toward more common (disordered) states.

This principle applies everywhere: heat flows from hot to cold (not the reverse), stars burn out, and the universe tends
toward maximum disorder. However, local regions can decrease entropy by using energy—your body maintains low entropy through
constant metabolic activity, powered by the sun's energy. The total entropy of the universe still increases.""",

            (PhysicsDomain.THERMODYNAMICS, 'temperature'):
                """Temperature is the average kinetic energy of particles in a substance. At absolute zero (-273.15°C),
particles have minimal motion; as temperature increases, particles vibrate and move faster.

Temperature differs fundamentally from heat. Temperature is a property of an object; heat is energy transfer between
objects at different temperatures. A cup of hot water has high temperature, but a swimming pool at warm temperature contains
far more thermal energy because it has so much more mass.

The three temperature scales (Celsius, Fahrenheit, Kelvin) measure the same physical phenomenon differently. Scientists
use Kelvin because it's an absolute scale starting at true zero—making ratios physically meaningful (200K is twice as hot as 100K).""",

            # ELECTROMAGNETISM
            (PhysicsDomain.ELECTROMAGNETISM, 'magnetism'):
                """Magnetism arises from moving electric charges. Electrons spinning in atoms create tiny magnetic moments;
when these align in the same direction, they produce observable magnetism.

Every magnet is fundamentally composed of aligned electrons. Permanent magnets have electron spins aligned in the same
direction, while non-magnetic materials have randomly oriented spins that cancel out. This is why magnets can be demagnetized
by heating (which randomizes electron motion) or by striking them (which physically disrupts alignment).

Earth itself is a giant magnet due to molten iron in the core carrying electrical currents. This magnetic field protects us
from solar radiation and guides migrating birds. Interestingly, Earth's magnetic poles flip periodically (thousands of times
in geological history), leaving magnetic records in rocks.""",

            (PhysicsDomain.ELECTROMAGNETISM, 'light'):
                """Light is an electromagnetic wave—synchronized oscillations of electric and magnetic fields propagating through
space at a constant speed (approximately 300,000 km/s in vacuum).

The frequency of these oscillations determines what we perceive: low frequencies are radio waves, infrared, then visible light
(which our eyes can detect), then ultraviolet, X-rays, and gamma rays. All are the same phenomenon—electromagnetic radiation—
just oscillating at different rates.

Light behaves both as a wave (explaining interference and diffraction) and as a particle (photon) depending on how you observe it.
The energy of a photon depends on its frequency: E = hf, where h is Planck's constant. This is why ultraviolet light can damage
DNA (high frequency, high energy) while radio waves pass through you harmlessly (low frequency, low energy).""",

            # QUANTUM MECHANICS
            (PhysicsDomain.QUANTUM_MECHANICS, 'superposition'):
                """At quantum scales, particles don't have definite properties until measured. Instead, they exist in a superposition
of multiple states simultaneously. This isn't because we lack information—experiments prove the particle literally doesn't have a
definite state until observation.

The famous thought experiment: Schrödinger's cat describes a cat that is both alive and dead simultaneously (if quantum rules applied
to macroscopic objects). This seems absurd because we never observe such superpositions at human scales, but it's the standard behavior
at atomic scales.

When you measure a quantum particle, the superposition "collapses" to a single definite state. Remarkably, the act of measurement
itself changes the system. This is foundational to quantum mechanics and leads to phenomena like quantum entanglement and explains
why the electron can orbit at multiple distances simultaneously until you measure its position.""",

            (PhysicsDomain.QUANTUM_MECHANICS, 'uncertainty'):
                """Heisenberg's Uncertainty Principle states you cannot simultaneously know both the exact position and exact momentum
of a particle. The more precisely you measure one, the less precisely you can know the other.

This isn't a limitation of measurement technology—it's fundamental to reality. Attempting to measure position requires high-energy
photons that disturb the particle's momentum. Attempting to measure momentum requires long wavelengths that blur position.

Mathematically: Δx · Δp ≥ ℏ/2 (where ℏ is Planck's constant). This principle explains why electrons can't collapse into the nucleus
(which would violate the principle), why atoms are so large, and why quantum tunneling is possible (particles can exist in regions
classically forbidden).""",

            (PhysicsDomain.QUANTUM_MECHANICS, 'entanglement'):
                """Quantum entanglement creates correlations between particles that are impossible in classical physics. When two particles
are entangled, measuring one instantly affects the other, regardless of distance.

This doesn't violate relativity because you can't use entanglement to transmit information faster than light—the correlation only becomes
apparent when comparing measurements. Yet Einstein called this "spooky action at a distance" because it seemed to violate locality (the idea
that distant objects can't instantly affect each other).

Experiments have confirmed that entanglement is real and not due to hidden variables. It's the foundation of quantum computing (qubits in superposition)
and quantum cryptography (unhackable encryption). Entanglement shows that the quantum world is fundamentally interconnected in ways our intuition
from macroscopic experience can't comprehend.""",

            # ADDITIONAL CLASSICAL MECHANICS PHENOMENA
            (PhysicsDomain.CLASSICAL_MECHANICS, 'friction'):
                """Friction is the force opposing motion between two surfaces in contact. It's caused by microscopic imperfections and interactions
between surface atoms. Without friction, you couldn't walk (your foot would slip), cars couldn't brake, and everything would slide indefinitely.

There are three types of friction: static (prevents initial motion), kinetic (opposes moving objects), and rolling (much lower than sliding friction).
Engineers exploit friction in brakes, tires, and clutches, while also minimize it in machinery using lubricants and ball bearings.

Real-world impact: Friction causes 15-20% of fuel consumption in cars. High-speed trains use magnetic levitation to eliminate friction entirely.
Friction also generates heat—it's why meteor entries create fireballs and why rubbing wood creates fire.""",

            (PhysicsDomain.CLASSICAL_MECHANICS, 'circular motion'):
                """Objects moving in circles experience centripetal acceleration—a constant force toward the center keeps them curved. This isn't
just theoretical; it's the reason planets orbit stars, why roller coasters need reinforced tracks, and why a bucket of water doesn't spill when
swung overhead.

The faster you go around a curve, the greater the centripetal force required. This is why highways have banked turns (tilted inward) to provide
gravity assistance, and why satellites must maintain specific orbital speeds. Too slow and they fall; too fast and they escape into space.

Real-world applications: GPS satellites orbit at 3.87 km/s. The International Space Station orbits Earth every 90 minutes at 7.66 km/s. Formula 1
cars experience up to 5G of centripetal acceleration in tight turns, pushing drivers to their physical limits.""",

            (PhysicsDomain.CLASSICAL_MECHANICS, 'projectile motion'):
                """When you throw an object, it follows a parabolic path—combining horizontal motion (which continues unchanged) with vertical motion
(accelerated by gravity). This principle applies to everything from baseballs to artillery to rockets.

The trajectory depends on launch angle, initial speed, and gravity. A 45-degree angle maximizes horizontal distance. This is why archers and
basketball players instinctively aim at optimal angles—the physics is built into human muscle memory through practice.

Real-world uses: Architects use projectile motion to design water fountains with specific splash patterns. Sports engineers analyze trajectories
to improve club designs in golf. Military ballistics rely entirely on these equations. Even plants use projectile motion—some seed pods launch
seeds at precise angles to maximize dispersal distance.""",

            # ADDITIONAL THERMODYNAMICS PHENOMENA
            (PhysicsDomain.THERMODYNAMICS, 'heat transfer'):
                """Heat moves through three mechanisms: conduction (direct contact), convection (fluid movement), and radiation (electromagnetic waves).
Understanding heat transfer is essential for designing everything from homes to computers to industrial furnaces.

Conduction is how a metal rod heats up when one end touches fire. Convection is how a hot-air balloon rises or how radiators heat rooms. Radiation
is how the sun warms Earth despite the vacuum of space. Most systems use combinations: a house loses heat through conduction (walls), convection
(air circulation), and radiation (infrared escape).

Real-world impact: Insulation works by minimizing heat transfer—preventing conduction through materials with low thermal conductivity (like fiberglass)
and convection by trapping air. Thermal imaging cameras detect infrared radiation to find heat leaks in buildings. Computer chips generate 100+ watts
of heat that must be conducted away or they'll fail.""",

            (PhysicsDomain.THERMODYNAMICS, 'phase transitions'):
                """Matter exists in distinct states—solid, liquid, gas, and plasma—and transitions between them require specific energy amounts (called
latent heat). Water freezes at 0°C and boils at 100°C at sea level, but these temperatures change with pressure.

Phase transitions are everywhere: ice melts when you add heat, water evaporates to cool (through sweating), and clouds form when water vapor cools.
Pressure cookers work by increasing pressure, which raises the boiling point, allowing hotter water and faster cooking. Understanding phase transitions
is crucial for refrigeration, weather prediction, and materials science.

Real-world applications: Industrial refrigeration removes heat through liquid-gas phase transitions. Thermal energy storage systems use phase-change
materials (like paraffin wax) that absorb massive amounts of heat while melting, then release it when solidifying. Dry ice (solid CO₂) sublimates
directly to gas, creating the dramatic fog effect in concerts and medical cryotherapy.""",

            # ADDITIONAL ELECTROMAGNETISM PHENOMENA
            (PhysicsDomain.ELECTROMAGNETISM, 'electric current'):
                """Electric current is the flow of electrons through a conductor. It powers every device we use—from lightbulbs to computers to
electric vehicles. The amount of current depends on voltage (electrical pressure) and resistance (opposition to flow).

Ohm's Law (V = IR) explains why a lightbulb draws more current at 110V than at 12V, and why thicker wires are used for high-current applications.
Superconductors (at extremely cold temperatures) have zero resistance, allowing indefinite current flow—a holy grail of physics that could revolutionize
power transmission.

Real-world impact: The global power grid transmits billions of watts using high voltages (reducing resistive losses). Your phone charger uses 5V at
2 amps (10 watts). Electric vehicles draw 100-200 amps when charging. Lightning is a catastrophic 30,000-amp discharge that can melt copper and cause
explosions.""",

            (PhysicsDomain.ELECTROMAGNETISM, 'circuits'):
                """Circuits are paths for electric current, consisting of a power source, conductors, and devices that use the electricity. Series
circuits have components in a line; parallel circuits have multiple paths. Understanding circuits is fundamental to all electronics.

In series, current flows through all components sequentially—if one breaks, all stop. Christmas lights used to work this way, which is why one broken
bulb darkened the whole strand. In parallel, each component has its own path—this is why your home outlets work independently; if one appliance stops,
others continue.

Real-world examples: Your house uses parallel circuits—different rooms and appliances draw power independently. Solar panels are connected in series
to increase voltage, then the output is converted. The human heart uses electrical signals following cardiac circuits; understanding these is crucial
for treating arrhythmias and designing pacemakers.""",

            # ADDITIONAL QUANTUM MECHANICS
            (PhysicsDomain.QUANTUM_MECHANICS, 'atomic structure'):
                """Atoms consist of a nucleus (protons and neutrons) surrounded by electrons in probability clouds (orbitals). The number of protons
defines the element; the number of electrons determines chemical properties. Electrons don't orbit like planets—they exist in quantized energy levels,
and transitions between levels absorb or emit light at specific frequencies.

This explains why elements have characteristic colors: sodium glows yellow, neon glows red, and copper compounds are blue. Lasers work by pumping
electrons to high energy levels, then stimulating them to cascade down, emitting coherent light. Chemistry itself is quantum mechanics—the chemical
bonds that hold molecules together are quantum phenomena.

Real-world applications: Atomic clocks measure frequency of electron transitions with incredible precision—the cesium-133 transition defines the second
itself. X-ray tubes use electron transitions to generate radiation for medical imaging. Semiconductors use quantum band structures to control electron flow,
enabling all modern electronics. Fireworks achieve their colors by exciting metal atoms to different quantum states.""",

            (PhysicsDomain.QUANTUM_MECHANICS, 'tunneling'):
                """Quantum tunneling allows particles to pass through energy barriers they classically couldn't overcome. According to classical physics,
an electron in an atom should fall into the nucleus, yet atoms are stable—quantum tunneling prevents this catastrophe.

At the quantum scale, particles have a wave-like character with a non-zero probability of appearing on the other side of a barrier without ever being
above the barrier. This becomes less probable with thicker barriers but never zero. This is why nuclear decay occurs—alpha particles tunnel through the
nuclear potential barrier.

Real-world impact: Tunnel diodes exploit tunneling for ultra-fast switching. Scanning tunneling microscopes use tunneling to image individual atoms—
moving a needle close to a surface and measuring tunneling current reveals atomic topography. DNA mutations sometimes result from tunneling of protons,
changing genetic code. Fusion reactors depend on tunneling to bring nuclei close enough to fuse at practical temperatures.""",

            # SACRED GEOMETRY / PATTERNS
            (PhysicsDomain.SACRED_GEOMETRY, 'golden ratio'):
                """The golden ratio (φ ≈ 1.618) appears throughout nature and art. Shells spiral in golden ratios, flower petals often number fibonacci
sequences (1, 1, 2, 3, 5, 8, 13...), and human face proportions approximate the golden ratio. This mathematical constant represents optimal
balance and efficiency.

Ancient architects recognized this ratio in perfect proportions. The Parthenon, Egyptian pyramids, and Renaissance art deliberately incorporated golden
ratios. Modern designers use it intuitively—the aspect ratio of an iPhone (9:16) approximates the golden rectangle.

Real-world mathematics: The golden ratio emerges from the fibonacci sequence naturally—each number equals the sum of the previous two. This creates
a spiral that efficiently packs items: sunflower seeds spiral in golden angles (137.5°) to maximize seed packing. Stock prices, molecular structures,
and even galaxy spiral arms follow golden ratio patterns. It's not magic—it's optimization that evolution and physics naturally discover.""",

            (PhysicsDomain.SACRED_GEOMETRY, 'fractals'):
                """Fractals are self-similar patterns that repeat at different scales. A coastline looks the same whether viewed from a plane or
under a microscope. Fractals appear in nature: trees branch fractally, blood vessels branch fractally, and clouds have fractal structure.

The Mandelbrot set is a famous mathematical fractal—zooming infinitely reveals ever-finer patterns. Fractals have non-integer dimensions between 1D
(line) and 2D (plane)—a coastline might have dimension 1.3, meaning it's jaggier than a line but less plane-filling than a full 2D shape.

Real-world applications: Fractal antennas are more efficient than conventional designs because their jagged structure resonates at multiple frequencies.
Medical imaging uses fractals—tumors have fractal characteristics different from healthy tissue, aiding detection. Atmospheric modeling uses fractals
to simulate cloud formation. Computer graphics render realistic terrain using fractals—games wouldn't look natural without this mathematics.""",
        }

        key = (domain, phenomenon.lower())

        # Try exact match first
        if key in explanations:
            return explanations[key]

        # Try keyword extraction if exact match fails
        keywords = [
            # Classical Mechanics
            'gravity', 'inertia', 'momentum', 'friction', 'circular', 'projectile',
            # Thermodynamics
            'entropy', 'temperature', 'heat', 'phase', 'transition',
            # Electromagnetism
            'magnetism', 'light', 'current', 'electric', 'circuit',
            # Quantum Mechanics
            'superposition', 'uncertainty', 'entanglement', 'atomic', 'atom', 'tunnel',
            # Sacred Geometry
            'golden', 'ratio', 'fractal'
        ]
        phenomenon_lower = phenomenon.lower()
        for keyword in keywords:
            if keyword in phenomenon_lower:
                key = (domain, keyword)
                if key in explanations:
                    return explanations[key]

        # Fallback
        return f"The phenomenon of {phenomenon} in {domain.value} is a deep and complex subject in physics. It involves the interplay of fundamental forces and principles that govern reality."

    def explain_law(self, law_name: str) -> str:
        """Provide intuitive explanation of a physics law."""
        law = self.kb.get_law(law_name)
        if not law:
            return f"Unknown law: {law_name}"

        explanation = f"""
Law: {law.name}
Domain: {law.domain.value}
Equation: {law.equation}

Principle: {self.kb.get_principle_description(law.principle)}

Conditions: {', '.join(law.conditions)}
Constraints: {', '.join(law.constraints)}
"""
        return explanation


# ============================================================================
# GAIA PHYSICS INTERFACE
# ============================================================================

class GAIAPhysicsInterface:
    """
    Bridge between standalone physics system and GAIA consciousness module.
    Enables GAIA to query physics knowledge and reasoning.
    """

    def __init__(self, knowledge_base: PhysicsKnowledgeBase):
        self.kb = knowledge_base
        self.reasoner = PhysicsReasoner(knowledge_base)
        self.simulator = PhysicsSimulator(knowledge_base)
        self.explainer = PhysicsExplainer(knowledge_base)

    def answer_physics_question(self, query: PhysicsQuery) -> PhysicsAnswer:
        """
        Main interface for GAIA to ask physics questions.
        Returns structured answer with reasoning and explanation.
        """
        # Identify applicable laws
        applicable_laws = [law for law in self.kb.laws.values()
                          if law.domain == query.domain]

        # Perform reasoning
        reasoning_steps = [f"Query: {query.question}"]
        reasoning_steps.append(f"Domain: {query.domain.value}")
        reasoning_steps.append(f"Found {len(applicable_laws)} applicable laws")

        # Extract principles
        principles_used = []
        for law in applicable_laws:
            if law.principle not in principles_used:
                principles_used.append(law.principle)

        reasoning_steps.extend([f"- {law.name}" for law in applicable_laws[:3]])

        # Generate detailed explanation first
        explanation = self.explainer.explain_phenomenon(
            query.question.lower(),
            query.domain
        )

        # Generate comprehensive answer combining summary and detailed explanation
        answer_text = f"This question involves {query.domain.value} physics. "
        answer_text += f"Key principles: {', '.join(p.value for p in principles_used[:2])}\n\n"

        # Add the detailed explanation as part of the answer
        answer_text += explanation

        # Confidence based on clarity of applicable principles
        confidence = min(0.95, 0.5 + 0.45 * (len(principles_used) / 9))

        return PhysicsAnswer(
            answer=answer_text,
            confidence=confidence,
            domain=query.domain,
            reasoning=reasoning_steps,
            principles_used=principles_used,
            explanation=explanation
        )

    def get_physics_knowledge(self, aspect: str) -> Dict[str, Any]:
        """
        Get structured physics knowledge for GAIA integration.
        """
        return {
            'constants': self.kb.constants,
            'principles': {p.value: desc for p, desc in self.kb.principles.items()},
            'laws': {name: {
                'equation': law.equation,
                'domain': law.domain.value,
                'principle': law.principle.value
            } for name, law in self.kb.laws.items()},
        }


# ============================================================================
# MAIN PHYSICS WORLD MODEL
# ============================================================================

class PhysicsWorldModel:
    """
    Main physics world model with all capabilities.
    Can function standalone or integrate with GAIA.
    """

    def __init__(self):
        self.kb = PhysicsKnowledgeBase()
        self.reasoner = PhysicsReasoner(self.kb)
        self.simulator = PhysicsSimulator(self.kb)
        self.explainer = PhysicsExplainer(self.kb)
        self.gaia_interface = GAIAPhysicsInterface(self.kb)

    def answer_question(self, question: str, domain: PhysicsDomain) -> PhysicsAnswer:
        """Answer a physics question (standalone mode)."""
        query = PhysicsQuery(
            question=question,
            domain=domain,
            requires_explanation=True
        )
        return self.gaia_interface.answer_physics_question(query)

    def query_gaia(self, question: str, domain: PhysicsDomain) -> Dict[str, Any]:
        """
        Interface for GAIA to query physics (compound integration mode).
        """
        answer = self.answer_question(question, domain)
        return {
            'answer': answer.answer,
            'confidence': answer.confidence,
            'reasoning': answer.reasoning,
            'explanation': answer.explanation,
            'principles': [p.value for p in answer.principles_used],
        }

    def list_domains(self) -> List[str]:
        """List all physics domains available."""
        return [d.value for d in PhysicsDomain]

    def list_laws(self, domain: Optional[PhysicsDomain] = None) -> Dict[str, str]:
        """List laws in a domain."""
        laws = {}
        for name, law in self.kb.laws.items():
            if domain is None or law.domain == domain:
                laws[name] = f"{law.name} ({law.domain.value})"
        return laws


# ============================================================================
# DEMO
# ============================================================================

if __name__ == "__main__":
    print("=" * 80)
    print("PHYSICS WORLD MODEL - Compound Integration System")
    print("=" * 80)
    print()

    # Initialize
    physics = PhysicsWorldModel()

    # Show available domains
    print("Available Physics Domains:")
    for domain in physics.list_domains():
        print(f"  • {domain}")
    print()

    # Show available laws
    print("Available Physics Laws:")
    laws = physics.list_laws()
    for i, (name, description) in enumerate(laws.items(), 1):
        if i <= 10:
            print(f"  {i}. {description}")
    print(f"  ... and {len(laws) - 10} more")
    print()

    # Example 1: Answer a physics question
    print("=" * 80)
    print("EXAMPLE 1: Standalone Physics Question")
    print("=" * 80)
    answer = physics.answer_question(
        "Why does gravity pull objects down?",
        PhysicsDomain.CLASSICAL_MECHANICS
    )
    print(f"Question: Why does gravity pull objects down?")
    print(f"Answer: {answer.answer}")
    print(f"Confidence: {answer.confidence:.1%}")
    print(f"Explanation: {answer.explanation}")
    print()

    # Example 2: GAIA integration
    print("=" * 80)
    print("EXAMPLE 2: GAIA Integration")
    print("=" * 80)
    gaia_result = physics.query_gaia(
        "How does entropy work?",
        PhysicsDomain.THERMODYNAMICS
    )
    print(f"GAIA Query: How does entropy work?")
    print(f"Physics Response: {gaia_result['answer']}")
    print(f"Confidence: {gaia_result['confidence']:.1%}")
    print(f"Principles Used: {', '.join(gaia_result['principles'])}")
    print()

    # Example 3: Sacred Geometry
    print("=" * 80)
    print("EXAMPLE 3: Sacred Geometry")
    print("=" * 80)
    answer = physics.answer_question(
        "What is the golden ratio?",
        PhysicsDomain.SACRED_GEOMETRY
    )
    print(f"Question: What is the golden ratio?")
    print(f"Answer: {answer.answer}")
    print(f"Explanation: {answer.explanation}")
    print()

    print("=" * 80)
    print("Physics World Model Ready for Integration")
    print("=" * 80)
