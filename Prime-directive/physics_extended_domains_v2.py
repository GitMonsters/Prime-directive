#!/usr/bin/env python3
"""
EXTENDED PHYSICS DOMAINS V2

Extends physics system to 20+ domains by adding 5 new domains:
- Nuclear Physics
- Materials Science
- Biophysics
- Geophysics
- Environmental Physics

Plus potential additional specialized domains.
"""

from enum import Enum
from typing import Dict, List, Optional, Set
from dataclasses import dataclass


# ============================================================================
# EXTENDED DOMAIN DEFINITIONS
# ============================================================================

class ExtendedDomainV2(Enum):
    """All 20+ physics domains including new extensions."""

    # Original 15 domains
    CLASSICAL_MECHANICS = "classical_mechanics"
    THERMODYNAMICS = "thermodynamics"
    ELECTROMAGNETISM = "electromagnetism"
    QUANTUM_MECHANICS = "quantum_mechanics"
    SACRED_GEOMETRY = "sacred_geometry"
    RELATIVITY = "relativity"
    FLUID_DYNAMICS = "fluid_dynamics"
    QUANTUM_FIELD_THEORY = "quantum_field_theory"
    COSMOLOGY = "cosmology"
    PARTICLE_PHYSICS = "particle_physics"
    OPTICS = "optics"
    ACOUSTICS = "acoustics"
    STATISTICAL_MECHANICS = "statistical_mechanics"
    PLASMA_PHYSICS = "plasma_physics"
    ASTROPHYSICS = "astrophysics"

    # New 5+ domains
    NUCLEAR_PHYSICS = "nuclear_physics"
    MATERIALS_SCIENCE = "materials_science"
    BIOPHYSICS = "biophysics"
    GEOPHYSICS = "geophysics"
    ENVIRONMENTAL_PHYSICS = "environmental_physics"

    # Additional optional domains
    SOLID_STATE_PHYSICS = "solid_state_physics"
    CONDENSED_MATTER_PHYSICS = "condensed_matter_physics"
    HIGH_ENERGY_PHYSICS = "high_energy_physics"
    LASER_PHYSICS = "laser_physics"


@dataclass
class ExtendedPhysicalLaw:
    """Definition of physics law with metadata."""
    name: str
    description: str
    equation: Optional[str] = None
    variables: List[str] = None
    domain: Optional[str] = None
    related_domains: List[str] = None
    applications: List[str] = None
    complexity: str = "intermediate"  # "simple", "intermediate", "advanced"

    def __post_init__(self):
        if self.variables is None:
            self.variables = []
        if self.related_domains is None:
            self.related_domains = []
        if self.applications is None:
            self.applications = []


@dataclass
class ExtendedPhysicalPrinciple:
    """Definition of physics principle."""
    name: str
    description: str
    domain: str
    related_laws: List[str] = None
    examples: List[str] = None

    def __post_init__(self):
        if self.related_laws is None:
            self.related_laws = []
        if self.examples is None:
            self.examples = []


# ============================================================================
# NUCLEAR PHYSICS
# ============================================================================

class NuclearPhysicsDomain:
    """Nuclear Physics domain: radioactivity, reactions, binding energy."""

    @staticmethod
    def get_laws() -> Dict[str, ExtendedPhysicalLaw]:
        return {
            'radioactive_decay': ExtendedPhysicalLaw(
                name="Radioactive Decay Law",
                description="Exponential decay of radioactive nuclei: N(t) = N₀ exp(-λt)",
                equation="N(t) = N₀ * exp(-λ*t)",
                variables=['N', 'N₀', 'λ', 't'],
                domain='nuclear_physics',
                applications=['radiocarbon dating', 'half-life calculation', 'radiation safety']
            ),
            'binding_energy': ExtendedPhysicalLaw(
                name="Nuclear Binding Energy",
                description="Energy holding nucleus together: BE = (Zm_p + Nm_n - M_nucleus)c²",
                equation="BE = (Z*m_p + N*m_n - M)*c²",
                variables=['Z', 'N', 'm_p', 'm_n', 'M', 'c'],
                domain='nuclear_physics',
                applications=['mass defect', 'stability analysis', 'reaction energy']
            ),
            'nuclear_reaction': ExtendedPhysicalLaw(
                name="Nuclear Reaction Cross Section",
                description="Probability of nuclear reaction between particles",
                equation="σ = π*b²*(angular_dependence)",
                variables=['σ', 'b', 'impact_parameter'],
                domain='nuclear_physics',
                applications=['fission', 'fusion', 'transmutation']
            ),
        }

    @staticmethod
    def get_principles() -> List[ExtendedPhysicalPrinciple]:
        return [
            ExtendedPhysicalPrinciple(
                name="Mass Defect",
                description="Loss of mass during nucleus formation converted to binding energy",
                domain='nuclear_physics',
                related_laws=['binding_energy'],
                examples=['Helium-4 nucleus', 'Iron-56 peak stability']
            ),
            ExtendedPhysicalPrinciple(
                name="Stability Valley",
                description="Nuclei with specific neutron-proton ratios are more stable",
                domain='nuclear_physics',
                related_laws=['radioactive_decay'],
                examples=['Stability curve', 'Beta decay modes']
            ),
            ExtendedPhysicalPrinciple(
                name="Fission and Fusion",
                description="Heavy nuclei split or light nuclei combine to release energy",
                domain='nuclear_physics',
                related_laws=['binding_energy', 'nuclear_reaction'],
                examples=['Uranium-235 fission', 'Deuterium-Tritium fusion']
            ),
        ]


# ============================================================================
# MATERIALS SCIENCE
# ============================================================================

class MaterialsScienceDomain:
    """Materials Science: crystal structures, mechanical properties, phase diagrams."""

    @staticmethod
    def get_laws() -> Dict[str, ExtendedPhysicalLaw]:
        return {
            'bragg_law': ExtendedPhysicalLaw(
                name="Bragg's Law",
                description="X-ray diffraction from crystal planes: nλ = 2d*sin(θ)",
                equation="n*λ = 2*d*sin(θ)",
                variables=['n', 'λ', 'd', 'θ'],
                domain='materials_science',
                applications=['crystal identification', 'structure determination', 'X-ray diffraction']
            ),
            'youngs_modulus': ExtendedPhysicalLaw(
                name="Young's Modulus",
                description="Elasticity measure: E = (stress)/(strain) = (F/A)/(ΔL/L)",
                equation="E = (F*L) / (A*ΔL)",
                variables=['E', 'F', 'A', 'L', 'ΔL'],
                domain='materials_science',
                applications=['material stiffness', 'elastic deformation', 'structural design']
            ),
            'phase_diagram': ExtendedPhysicalLaw(
                name="Phase Diagram Relations",
                description="Gibbs phase rule: F = C - P + 2 (degrees of freedom)",
                equation="F = C - P + 2",
                variables=['F', 'C', 'P'],
                domain='materials_science',
                applications=['alloy design', 'phase transitions', 'equilibrium prediction']
            ),
        }

    @staticmethod
    def get_principles() -> List[ExtendedPhysicalPrinciple]:
        return [
            ExtendedPhysicalPrinciple(
                name="Crystal Structure",
                description="Periodic arrangement of atoms in lattices determines material properties",
                domain='materials_science',
                related_laws=['bragg_law'],
                examples=['FCC, BCC, HCP lattices', 'Miller indices']
            ),
            ExtendedPhysicalPrinciple(
                name="Mechanical Properties",
                description="Strength, ductility, hardness depend on crystal structure and defects",
                domain='materials_science',
                related_laws=['youngs_modulus'],
                examples=['Stress-strain curves', 'Fracture mechanics']
            ),
            ExtendedPhysicalPrinciple(
                name="Defects and Dislocations",
                description="Crystal imperfections affect mechanical and thermal properties",
                domain='materials_science',
                related_laws=['youngs_modulus'],
                examples=['Point defects', 'Edge and screw dislocations']
            ),
        ]


# ============================================================================
# BIOPHYSICS
# ============================================================================

class BiophysicsDomain:
    """Biophysics: protein folding, DNA dynamics, cellular mechanics."""

    @staticmethod
    def get_laws() -> Dict[str, ExtendedPhysicalLaw]:
        return {
            'protein_folding': ExtendedPhysicalLaw(
                name="Protein Folding Energy",
                description="Proteins fold to minimize free energy: ΔG = ΔH - TΔS",
                equation="ΔG = ΔH - T*ΔS",
                variables=['ΔG', 'ΔH', 'T', 'ΔS'],
                domain='biophysics',
                applications=['protein structure prediction', 'drug design', 'enzyme kinetics']
            ),
            'dna_melting': ExtendedPhysicalLaw(
                name="DNA Melting Temperature",
                description="Temperature at which DNA strands separate: T_m = 4(G+C) + 2(A+T)",
                equation="T_m = 4*(G+C) + 2*(A+T)",
                variables=['T_m', 'G', 'C', 'A', 'T'],
                domain='biophysics',
                applications=['PCR design', 'DNA stability', 'sequencing']
            ),
            'osmotic_pressure': ExtendedPhysicalLaw(
                name="Osmotic Pressure",
                description="Pressure from dissolved solutes: π = i*M*R*T",
                equation="π = i*M*R*T",
                variables=['π', 'i', 'M', 'R', 'T'],
                domain='biophysics',
                applications=['cell equilibrium', 'osmosis', 'membrane transport']
            ),
        }

    @staticmethod
    def get_principles() -> List[ExtendedPhysicalPrinciple]:
        return [
            ExtendedPhysicalPrinciple(
                name="Protein Structure Levels",
                description="Primary, secondary, tertiary, quaternary structures build complexity",
                domain='biophysics',
                related_laws=['protein_folding'],
                examples=['α-helix, β-sheet', 'domain organization']
            ),
            ExtendedPhysicalPrinciple(
                name="DNA as Information Storage",
                description="DNA structure encodes genetic information through base pairing",
                domain='biophysics',
                related_laws=['dna_melting'],
                examples=['Double helix', 'Watson-Crick base pairing']
            ),
            ExtendedPhysicalPrinciple(
                name="Cellular Mechanics",
                description="Cells respond to mechanical forces like tensegrity structures",
                domain='biophysics',
                related_laws=['osmotic_pressure'],
                examples=['Cytoskeleton', 'cell migration']
            ),
        ]


# ============================================================================
# GEOPHYSICS
# ============================================================================

class GeophysicsDomain:
    """Geophysics: Earth structure, seismology, plate tectonics."""

    @staticmethod
    def get_laws() -> Dict[str, ExtendedPhysicalLaw]:
        return {
            'seismic_waves': ExtendedPhysicalLaw(
                name="Seismic Wave Equations",
                description="P-waves (compression) travel faster than S-waves (shear)",
                equation="v_p > v_s; v ∝ √(modulus/density)",
                variables=['v_p', 'v_s', 'modulus', 'density'],
                domain='geophysics',
                applications=['earthquake location', 'interior structure', 'resource exploration']
            ),
            'lithostatic_pressure': ExtendedPhysicalLaw(
                name="Lithostatic Pressure",
                description="Pressure from overlying rock: P = ρ*g*h",
                equation="P = ρ*g*h",
                variables=['P', 'ρ', 'g', 'h'],
                domain='geophysics',
                applications=['depth estimation', 'mineral stability', 'crustal structure']
            ),
            'geothermal_gradient': ExtendedPhysicalLaw(
                name="Geothermal Gradient",
                description="Temperature increases with depth: dT/dz ≈ 25-30 K/km",
                equation="T(z) = T₀ + (dT/dz)*z",
                variables=['T', 'z', 'dT/dz', 'T₀'],
                domain='geophysics',
                applications=['heat flow', 'magma generation', 'metamorphism']
            ),
        }

    @staticmethod
    def get_principles() -> List[ExtendedPhysicalPrinciple]:
        return [
            ExtendedPhysicalPrinciple(
                name="Plate Tectonics",
                description="Earth's crust divided into moving plates with interactions at boundaries",
                domain='geophysics',
                related_laws=['seismic_waves', 'lithostatic_pressure'],
                examples=['Convergent, divergent, transform boundaries']
            ),
            ExtendedPhysicalPrinciple(
                name="Isostasy",
                description="Lithosphere floats on denser asthenosphere in gravitational equilibrium",
                domain='geophysics',
                related_laws=['lithostatic_pressure'],
                examples=['Mountain roots', 'sedimentary basin loading']
            ),
            ExtendedPhysicalPrinciple(
                name="Mantle Dynamics",
                description="Convection in Earth's mantle drives plate motion and surface features",
                domain='geophysics',
                related_laws=['geothermal_gradient'],
                examples=['Hot spots', 'ridge push-slab pull']
            ),
        ]


# ============================================================================
# ENVIRONMENTAL PHYSICS
# ============================================================================

class EnvironmentalPhysicsDomain:
    """Environmental Physics: atmospheric dynamics, climate, pollution modeling."""

    @staticmethod
    def get_laws() -> Dict[str, ExtendedPhysicalLaw]:
        return {
            'ideal_gas_law_atm': ExtendedPhysicalLaw(
                name="Atmospheric Ideal Gas Law",
                description="Relates pressure, density, temperature in atmosphere: P = ρ*R*T",
                equation="P = ρ*R_specific*T",
                variables=['P', 'ρ', 'R', 'T'],
                domain='environmental_physics',
                applications=['weather prediction', 'altitude effects', 'atmospheric stability']
            ),
            'radiative_balance': ExtendedPhysicalLaw(
                name="Earth's Radiative Balance",
                description="Energy balance: Solar input = Reflected + Emitted",
                equation="Q_in*(1-α) = σ*T⁴",
                variables=['Q_in', 'α', 'σ', 'T'],
                domain='environmental_physics',
                applications=['climate models', 'greenhouse effect', 'temperature prediction']
            ),
            'diffusion_pollution': ExtendedPhysicalLaw(
                name="Pollution Diffusion",
                description="Pollutants spread via diffusion and advection: ∂c/∂t = -v·∇c + D∇²c",
                equation="∂c/∂t = -v·∇c + D*∇²c",
                variables=['c', 't', 'v', 'D'],
                domain='environmental_physics',
                applications=['air quality', 'water contamination', 'dispersion modeling']
            ),
        }

    @staticmethod
    def get_principles() -> List[ExtendedPhysicalPrinciple]:
        return [
            ExtendedPhysicalPrinciple(
                name="Atmospheric Thermodynamics",
                description="Temperature and pressure structure determined by solar heating and rotation",
                domain='environmental_physics',
                related_laws=['ideal_gas_law_atm'],
                examples=['Troposphere, stratosphere', 'adiabatic processes']
            ),
            ExtendedPhysicalPrinciple(
                name="Climate System Forcing",
                description="Climate responds to radiative forcing from greenhouse gases and solar variation",
                domain='environmental_physics',
                related_laws=['radiative_balance'],
                examples=['CO₂ forcing', 'feedback mechanisms']
            ),
            ExtendedPhysicalPrinciple(
                name="Transport and Mixing",
                description="Wind, ocean currents, and turbulence transport heat, moisture, and pollutants",
                domain='environmental_physics',
                related_laws=['diffusion_pollution'],
                examples=['Meridional heat transport', 'turbulent mixing']
            ),
        ]


# ============================================================================
# UNIFIED EXTENDED DOMAINS SYSTEM V2
# ============================================================================

class ExtendedDomainsSystemV2:
    """Unified system for 20+ physics domains with new extensions."""

    def __init__(self):
        """Initialize all domains."""
        self.domains = list(ExtendedDomainV2)
        self.domain_objects = {
            'nuclear_physics': NuclearPhysicsDomain(),
            'materials_science': MaterialsScienceDomain(),
            'biophysics': BiophysicsDomain(),
            'geophysics': GeophysicsDomain(),
            'environmental_physics': EnvironmentalPhysicsDomain(),
        }

    def get_all_domains(self) -> List[str]:
        """Get list of all domain names."""
        return [d.value for d in self.domains]

    def get_domain_count(self) -> int:
        """Get total number of domains."""
        return len(self.domains)

    def get_new_domains(self) -> List[str]:
        """Get only the 5 new domains."""
        new_domain_names = [
            'nuclear_physics', 'materials_science', 'biophysics',
            'geophysics', 'environmental_physics'
        ]
        return new_domain_names

    def get_domain_laws(self, domain: str) -> Optional[Dict[str, ExtendedPhysicalLaw]]:
        """Get laws for a specific domain."""
        if domain == 'nuclear_physics':
            return NuclearPhysicsDomain.get_laws()
        elif domain == 'materials_science':
            return MaterialsScienceDomain.get_laws()
        elif domain == 'biophysics':
            return BiophysicsDomain.get_laws()
        elif domain == 'geophysics':
            return GeophysicsDomain.get_laws()
        elif domain == 'environmental_physics':
            return EnvironmentalPhysicsDomain.get_laws()
        return None

    def get_domain_principles(self, domain: str) -> Optional[List[ExtendedPhysicalPrinciple]]:
        """Get principles for a specific domain."""
        if domain == 'nuclear_physics':
            return NuclearPhysicsDomain.get_principles()
        elif domain == 'materials_science':
            return MaterialsScienceDomain.get_principles()
        elif domain == 'biophysics':
            return BiophysicsDomain.get_principles()
        elif domain == 'geophysics':
            return GeophysicsDomain.get_principles()
        elif domain == 'environmental_physics':
            return EnvironmentalPhysicsDomain.get_principles()
        return None

    def get_domain_info(self, domain: str) -> Optional[Dict]:
        """Get comprehensive info about domain."""
        laws = self.get_domain_laws(domain)
        principles = self.get_domain_principles(domain)

        if laws is None or principles is None:
            return None

        return {
            'name': domain,
            'laws': {name: {
                'description': law.description,
                'equation': law.equation,
                'complexity': law.complexity
            } for name, law in laws.items()},
            'principles': [
                {
                    'name': p.name,
                    'description': p.description,
                    'examples': p.examples
                }
                for p in principles
            ],
            'law_count': len(laws),
            'principle_count': len(principles)
        }

    def get_statistics(self) -> Dict:
        """Get statistics about extended domains."""
        total_laws = 0
        total_principles = 0

        for domain in self.get_new_domains():
            laws = self.get_domain_laws(domain)
            principles = self.get_domain_principles(domain)
            if laws:
                total_laws += len(laws)
            if principles:
                total_principles += len(principles)

        return {
            'total_domains': self.get_domain_count(),
            'new_domains': len(self.get_new_domains()),
            'original_domains': self.get_domain_count() - len(self.get_new_domains()),
            'new_domain_laws': total_laws,
            'new_domain_principles': total_principles,
            'domains': self.get_all_domains()
        }


# ============================================================================
# DEMO
# ============================================================================

if __name__ == '__main__':
    print("=" * 80)
    print("EXTENDED PHYSICS DOMAINS V2 - 20+ DOMAINS")
    print("=" * 80)
    print()

    system = ExtendedDomainsSystemV2()

    # Statistics
    print("SYSTEM STATISTICS:")
    stats = system.get_statistics()
    print(f"  Total domains: {stats['total_domains']}")
    print(f"  Original domains: {stats['original_domains']}")
    print(f"  New domains: {stats['new_domains']}")
    print(f"  New laws: {stats['new_domain_laws']}")
    print(f"  New principles: {stats['new_domain_principles']}")

    print("\n" + "=" * 80)
    print("NEW DOMAINS:")
    print("=" * 80)
    for domain in system.get_new_domains():
        info = system.get_domain_info(domain)
        if info:
            print(f"\n{domain.upper()}")
            print(f"  Laws: {info['law_count']}")
            print(f"  Principles: {info['principle_count']}")
            for law_name in list(info['laws'].keys())[:2]:
                print(f"    • {law_name}")

    print("\n" + "=" * 80)
    print("DETAILED EXAMPLE: NUCLEAR PHYSICS")
    print("=" * 80)
    nuclear_info = system.get_domain_info('nuclear_physics')
    if nuclear_info:
        print(f"\nLaws:")
        for law_name, law_info in nuclear_info['laws'].items():
            print(f"  • {law_name}: {law_info['description'][:60]}...")
            if law_info['equation']:
                print(f"    Equation: {law_info['equation']}")

        print(f"\nPrinciples:")
        for principle in nuclear_info['principles']:
            print(f"  • {principle['name']}: {principle['description'][:50]}...")

    print("\n" + "=" * 80)
    print("ALL DOMAINS (20+):")
    print("=" * 80)
    domains = system.get_all_domains()
    for i, domain in enumerate(domains, 1):
        domain_type = "NEW" if domain in system.get_new_domains() else "ORIGINAL"
        print(f"  {i:2d}. {domain:30s} [{domain_type}]")

    print(f"\n✅ Extended Domains V2 System with {system.get_domain_count()} total domains")
