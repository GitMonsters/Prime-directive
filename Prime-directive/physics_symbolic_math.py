#!/usr/bin/env python3
"""
SYMBOLIC MATHEMATICS FOR PHYSICS

Integrates SymPy for symbolic computation across all physics domains.
Enables equation solving, differentiation, integration, and manipulation.
"""

from typing import Dict, List, Optional, Tuple, Any
from sympy import *
from sympy.physics.mechanics import *
from sympy.physics.quantum import *
import sympy as sp


# ============================================================================
# SYMBOLIC PHYSICS EQUATIONS
# ============================================================================

class SymbolicPhysicsEquations:
    """Collection of symbolic physics equations across domains."""

    def __init__(self):
        self.equations = {}
        self.solutions = {}
        self._initialize_equations()

    def _initialize_equations(self):
        """Initialize symbolic equations for all domains."""

        # Define symbols
        t, x, y, z = symbols('t x y z', real=True)
        m, q, E, B, v, c = symbols('m q E B v c', positive=True, real=True)
        T, P, V, n, k_B = symbols('T P V n k_B', positive=True, real=True)
        hbar, h = symbols('hbar h', positive=True, real=True)
        G, M = symbols('G M', positive=True, real=True)
        rho, mu = symbols('rho mu', positive=True, real=True)

        # ─── CLASSICAL MECHANICS ──────────────────────────────────
        self.equations['newtons_second_law'] = {
            'equation': Eq(m * symbols('a'), symbols('F')),
            'description': 'F = ma',
            'domain': 'classical_mechanics',
            'solve_for': ['a', 'F', 'm']
        }

        self.equations['kinetic_energy'] = {
            'equation': Eq(symbols('K'), Rational(1, 2) * m * v**2),
            'description': 'Kinetic energy',
            'domain': 'classical_mechanics',
            'solve_for': ['K', 'v', 'm']
        }

        # ─── RELATIVITY ───────────────────────────────────────────
        self.equations['mass_energy_equivalence'] = {
            'equation': Eq(symbols('E'), m * c**2),
            'description': 'E = mc²',
            'domain': 'relativity',
            'solve_for': ['E', 'm', 'c']
        }

        self.equations['lorentz_factor'] = {
            'equation': Eq(symbols('gamma'), 1 / sqrt(1 - v**2/c**2)),
            'description': 'γ = 1/√(1 - v²/c²)',
            'domain': 'relativity',
            'solve_for': ['gamma', 'v', 'c']
        }

        # ─── THERMODYNAMICS ───────────────────────────────────────
        self.equations['ideal_gas_law'] = {
            'equation': Eq(P * V, n * k_B * T),
            'description': 'PV = nk_BT',
            'domain': 'thermodynamics',
            'solve_for': ['P', 'V', 'T', 'n']
        }

        # ─── ELECTROMAGNETISM ─────────────────────────────────────
        self.equations['lorentz_force'] = {
            'equation': Eq(symbols('F'), q * (E + v * B)),
            'description': 'F = q(E + v×B)',
            'domain': 'electromagnetism',
            'solve_for': ['F', 'q', 'E', 'v', 'B']
        }

        self.equations['coulombs_law'] = {
            'equation': Eq(symbols('F'), (q**2) / (4 * pi * symbols('epsilon_0') * x**2)),
            'description': 'Coulomb force',
            'domain': 'electromagnetism',
            'solve_for': ['F', 'q', 'x']
        }

        # ─── QUANTUM MECHANICS ────────────────────────────────────
        self.equations['schrodinger_equation_1d'] = {
            'equation': Eq(-hbar**2/(2*m) * symbols('psi').diff(x, 2) + symbols('V')*symbols('psi'), symbols('E')*symbols('psi')),
            'description': 'Time-independent Schrödinger equation (1D)',
            'domain': 'quantum_mechanics',
            'solve_for': ['E', 'psi', 'V']
        }

        self.equations['de_broglie_wavelength'] = {
            'equation': Eq(symbols('lambda'), h / (m * v)),
            'description': 'λ = h/(mv)',
            'domain': 'quantum_mechanics',
            'solve_for': ['lambda', 'h', 'm', 'v']
        }

        # ─── COSMOLOGY ────────────────────────────────────────────
        self.equations['hubble_law'] = {
            'equation': Eq(symbols('v_recession'), symbols('H_0') * symbols('distance')),
            'description': 'v = H₀d',
            'domain': 'cosmology',
            'solve_for': ['v_recession', 'H_0', 'distance']
        }

        # ─── ASTROPHYSICS ─────────────────────────────────────────
        self.equations['schwarzschild_radius'] = {
            'equation': Eq(symbols('r_s'), 2 * G * M / c**2),
            'description': 'Schwarzschild radius',
            'domain': 'astrophysics',
            'solve_for': ['r_s', 'G', 'M', 'c']
        }

    def solve_for(self, equation_name: str, variable: str) -> Optional[List]:
        """Solve equation for a specific variable."""
        if equation_name not in self.equations:
            return None

        eq_info = self.equations[equation_name]
        equation = eq_info['equation']

        try:
            # Get all symbols in equation
            symbols_in_eq = list(equation.free_symbols)

            # Find which symbol to solve for
            target_symbol = None
            for sym in symbols_in_eq:
                if str(sym) == variable or variable in str(sym):
                    target_symbol = sym
                    break

            if not target_symbol:
                return None

            solutions = solve(equation, target_symbol)
            return solutions
        except:
            return None

    def differentiate(self, equation_name: str, with_respect_to: str) -> Optional[Expr]:
        """Differentiate equation with respect to variable."""
        if equation_name not in self.equations:
            return None

        eq_info = self.equations[equation_name]
        equation = eq_info['equation']

        try:
            # Get RHS of equation
            rhs = equation.rhs

            # Find symbol to differentiate with respect to
            var_symbol = None
            for sym in equation.free_symbols:
                if str(sym) == with_respect_to:
                    var_symbol = sym
                    break

            if not var_symbol:
                return None

            derivative = diff(rhs, var_symbol)
            return derivative
        except:
            return None

    def simplify_equation(self, equation_name: str) -> Optional[Eq]:
        """Simplify equation."""
        if equation_name not in self.equations:
            return None

        eq_info = self.equations[equation_name]
        equation = eq_info['equation']

        try:
            simplified = Eq(simplify(equation.lhs), simplify(equation.rhs))
            return simplified
        except:
            return None

    def get_equation(self, equation_name: str) -> Optional[Dict]:
        """Get equation details."""
        return self.equations.get(equation_name)

    def list_equations(self, domain: Optional[str] = None) -> List[str]:
        """List available equations, optionally filtered by domain."""
        if not domain:
            return list(self.equations.keys())

        return [name for name, info in self.equations.items()
                if info.get('domain') == domain]


# ============================================================================
# SYMBOLIC REASONING ENGINE
# ============================================================================

class SymbolicPhysicsReasoner:
    """Symbolic reasoning for physics problems."""

    def __init__(self):
        self.equations = SymbolicPhysicsEquations()
        self.reasoning_cache = {}

    def symbolic_solve_problem(self, problem: str, equation_name: str,
                             solve_for_variable: str) -> Dict:
        """
        Symbolically solve a physics problem.

        Example:
            problem = "A 2kg mass falls. What is its kinetic energy at v=10m/s?"
            equation_name = "kinetic_energy"
            solve_for_variable = "K"
        """
        solution = self.equations.solve_for(equation_name, solve_for_variable)

        return {
            'problem': problem,
            'equation': equation_name,
            'solving_for': solve_for_variable,
            'solution': solution,
            'steps': self._generate_solution_steps(equation_name, solve_for_variable)
        }

    def _generate_solution_steps(self, equation_name: str,
                                variable: str) -> List[str]:
        """Generate step-by-step solution process."""
        eq_info = self.equations.get_equation(equation_name)
        if not eq_info:
            return []

        return [
            f"Starting equation: {eq_info['description']}",
            f"Identify variable to solve for: {variable}",
            f"Rearrange equation algebraically",
            f"Substitute known values",
            f"Simplify result"
        ]

    def derive_formula(self, equation_name: str, with_respect_to: str) -> Dict:
        """Derive new formula by differentiation."""
        derivative = self.equations.differentiate(equation_name, with_respect_to)

        return {
            'original_equation': equation_name,
            'differentiated_with_respect_to': with_respect_to,
            'result': derivative,
            'interpretation': self._interpret_derivative(equation_name, derivative)
        }

    def _interpret_derivative(self, equation_name: str, derivative: Any) -> str:
        """Interpret physical meaning of derivative."""
        interpretations = {
            'kinetic_energy': "Rate of change of kinetic energy with respect to velocity",
            'mass_energy_equivalence': "Rate of change of energy with respect to mass",
        }
        return interpretations.get(equation_name, "Rate of change of the equation")


# ============================================================================
# DEMO
# ============================================================================

if __name__ == '__main__':
    print("=" * 80)
    print("SYMBOLIC MATHEMATICS FOR PHYSICS")
    print("=" * 80)
    print()

    equations = SymbolicPhysicsEquations()
    reasoner = SymbolicPhysicsReasoner()

    # List equations by domain
    print("Available Equations by Domain:")
    domains = set(info.get('domain') for info in equations.equations.values())
    for domain in sorted(domains):
        eqs = equations.list_equations(domain)
        print(f"\n{domain.upper()}:")
        for eq in eqs:
            info = equations.get_equation(eq)
            print(f"  • {info['description']}")

    # Example: Solve kinetic energy for velocity
    print("\n" + "=" * 80)
    print("EXAMPLE 1: Solve Kinetic Energy for Velocity")
    print("=" * 80)

    result = reasoner.symbolic_solve_problem(
        "Given K=100J and m=2kg, find velocity",
        "kinetic_energy",
        "v"
    )
    print(f"Equation: {result['equation']}")
    print(f"Solving for: {result['solving_for']}")
    print(f"Solution: {result['solution']}")

    # Example: E=mc² solving for mass
    print("\n" + "=" * 80)
    print("EXAMPLE 2: Solve E=mc² for Mass")
    print("=" * 80)

    result = reasoner.symbolic_solve_problem(
        "Given E=1MeV, find mass using E=mc²",
        "mass_energy_equivalence",
        "m"
    )
    print(f"Solution: {result['solution']}")

    # Example: Differentiation
    print("\n" + "=" * 80)
    print("EXAMPLE 3: Derive Rate of Change of Kinetic Energy")
    print("=" * 80)

    result = reasoner.derive_formula("kinetic_energy", "v")
    print(f"Original: K = ½mv²")
    print(f"Derivative: dK/dv = {result['result']}")

    # Ideal gas law solving
    print("\n" + "=" * 80)
    print("EXAMPLE 4: Ideal Gas Law - Solve for Temperature")
    print("=" * 80)

    result = reasoner.symbolic_solve_problem(
        "Given P, V, n, find T from PV=nk_BT",
        "ideal_gas_law",
        "T"
    )
    print(f"Solution: {result['solution']}")
