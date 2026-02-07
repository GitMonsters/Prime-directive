#!/usr/bin/env python3
"""
PHYSICS SIMULATION ENGINE

Numerical simulation of physical systems across multiple domains.
Supports time-stepping, numerical integration, and system evolution.
"""

import numpy as np
from typing import Dict, List, Tuple, Optional, Callable, Any
from dataclasses import dataclass
import matplotlib.pyplot as plt


# ============================================================================
# PHYSICS SIMULATION FRAMEWORK
# ============================================================================

@dataclass
class SimulationState:
    """State of a physics simulation."""
    time: float
    position: np.ndarray
    velocity: np.ndarray
    acceleration: np.ndarray
    energy: float
    properties: Dict[str, float]


@dataclass
class SimulationConfig:
    """Configuration for simulation."""
    domain: str
    dt: float  # Time step
    t_end: float  # End time
    method: str  # Integration method: 'euler', 'rk4', 'verlet'
    initial_conditions: Dict
    parameters: Dict


class PhysicsSimulator:
    """Universal physics simulator for all domains."""

    def __init__(self):
        self.simulations = {}
        self.results = {}
        self._register_domains()

    def _register_domains(self):
        """Register simulation methods for each domain."""
        self.domain_simulators = {
            'classical_mechanics': self._simulate_classical,
            'fluid_dynamics': self._simulate_fluid,
            'quantum_mechanics': self._simulate_quantum,
            'thermodynamics': self._simulate_thermal,
            'electromagnetism': self._simulate_em,
            'relativity': self._simulate_relativity,
            'astrophysics': self._simulate_astrophysics,
        }

    # ─────────────────────────────────────────────────────────────
    # CLASSICAL MECHANICS SIMULATION
    # ─────────────────────────────────────────────────────────────

    def _simulate_classical(self, config: SimulationConfig) -> Dict:
        """Simulate classical mechanics (N-body, projectile, etc)."""
        initial = config.initial_conditions
        params = config.parameters

        # Extract initial state
        pos = np.array(initial.get('position', [0.0, 0.0]))
        vel = np.array(initial.get('velocity', [0.0, 0.0]))
        m = params.get('mass', 1.0)
        g = params.get('gravity', 9.81)

        # Time array
        time = np.arange(0, config.t_end, config.dt)
        positions = [pos.copy()]
        velocities = [vel.copy()]
        energies = []

        # Integrate
        for t in time[1:]:
            # Force (gravity)
            F = np.array([0, -m * g])
            a = F / m

            # Update velocity and position
            vel = vel + a * config.dt
            pos = pos + vel * config.dt

            # Calculate energy
            KE = 0.5 * m * np.sum(vel**2)
            PE = m * g * pos[1]
            E = KE + PE

            positions.append(pos.copy())
            velocities.append(vel.copy())
            energies.append(E)

        return {
            'domain': 'classical_mechanics',
            'time': time,
            'positions': np.array(positions),
            'velocities': np.array(velocities),
            'energies': np.array(energies),
            'trajectory': positions
        }

    # ─────────────────────────────────────────────────────────────
    # FLUID DYNAMICS SIMULATION
    # ─────────────────────────────────────────────────────────────

    def _simulate_fluid(self, config: SimulationConfig) -> Dict:
        """Simulate fluid dynamics (simplified grid-based)."""
        initial = config.initial_conditions
        params = config.parameters

        # Grid
        nx, ny = initial.get('grid_size', (50, 50))
        rho = np.ones((nx, ny)) * params.get('density', 1.0)
        u = np.zeros((nx, ny))
        v = np.zeros((nx, ny))

        # Initial conditions
        u_initial = initial.get('velocity_x', 1.0)
        u[:, :] = u_initial

        time = np.arange(0, config.t_end, config.dt)
        rho_history = [rho.copy()]

        # Simple advection
        for t in time[1:]:
            # Advect density
            rho = rho - u * np.gradient(rho, axis=0) * config.dt

            rho_history.append(rho.copy())

        return {
            'domain': 'fluid_dynamics',
            'time': time,
            'density': np.array(rho_history),
            'velocity_x': u,
            'velocity_y': v,
            'grid_size': (nx, ny)
        }

    # ─────────────────────────────────────────────────────────────
    # QUANTUM MECHANICS SIMULATION
    # ─────────────────────────────────────────────────────────────

    def _simulate_quantum(self, config: SimulationConfig) -> Dict:
        """Simulate quantum mechanics (wavefunction evolution)."""
        initial = config.initial_conditions
        params = config.parameters

        # Spatial grid
        x_min, x_max = initial.get('x_range', (-5, 5))
        nx = initial.get('points', 256)
        x = np.linspace(x_min, x_max, nx)

        # Gaussian wavepacket
        x0 = initial.get('center', 0.0)
        sigma = initial.get('width', 1.0)
        k0 = initial.get('momentum', 0.0)

        psi = np.exp(-((x - x0)**2) / (2 * sigma**2)) * np.exp(1j * k0 * x)
        psi = psi / np.sqrt(np.sum(np.abs(psi)**2))

        time = np.arange(0, config.t_end, config.dt)
        hbar = params.get('hbar', 1.0)
        m = params.get('mass', 1.0)

        psi_history = [psi.copy()]
        probability = [np.abs(psi)**2]

        # Simple evolution (kinetic energy operator)
        for t in time[1:]:
            # FFT-based evolution
            psi_k = np.fft.fft(psi)
            k = 2 * np.pi * np.fft.fftfreq(nx, x[1] - x[0])
            phase = np.exp(-1j * hbar * k**2 / (2 * m) * config.dt)
            psi_k = psi_k * phase
            psi = np.fft.ifft(psi_k)

            psi_history.append(psi.copy())
            probability.append(np.abs(psi)**2)

        return {
            'domain': 'quantum_mechanics',
            'time': time,
            'x': x,
            'wavefunction': np.array(psi_history),
            'probability': np.array(probability),
            'normalization': np.sum(np.abs(psi)**2) * (x[1] - x[0])
        }

    # ─────────────────────────────────────────────────────────────
    # THERMODYNAMICS SIMULATION
    # ─────────────────────────────────────────────────────────────

    def _simulate_thermal(self, config: SimulationConfig) -> Dict:
        """Simulate thermodynamic system (heat diffusion)."""
        initial = config.initial_conditions
        params = config.parameters

        # 1D heat diffusion
        nx = initial.get('points', 100)
        x = np.linspace(0, 1, nx)
        T = np.ones(nx) * initial.get('ambient_temp', 300.0)
        T[nx//2] = initial.get('source_temp', 500.0)

        alpha = params.get('thermal_diffusivity', 0.1)

        time = np.arange(0, config.t_end, config.dt)
        T_history = [T.copy()]

        # Forward Euler (explicit)
        dx = 1.0 / (nx - 1)
        for t in time[1:]:
            dT_dx2 = np.gradient(np.gradient(T, dx), dx)
            T = T + alpha * dT_dx2 * config.dt
            T_history.append(T.copy())

        return {
            'domain': 'thermodynamics',
            'time': time,
            'x': x,
            'temperature': np.array(T_history),
            'final_state': T,
            'equilibrium_reached': np.std(T) < 1.0
        }

    # ─────────────────────────────────────────────────────────────
    # ELECTROMAGNETISM SIMULATION
    # ─────────────────────────────────────────────────────────────

    def _simulate_em(self, config: SimulationConfig) -> Dict:
        """Simulate electromagnetic field propagation."""
        initial = config.initial_conditions
        params = config.parameters

        # 1D wave equation (electromagnetic wave)
        nx = initial.get('points', 100)
        x = np.linspace(0, 1, nx)
        E = np.sin(2 * np.pi * x) * initial.get('amplitude', 1.0)
        dE_dt = np.zeros_like(E)

        c = params.get('speed_of_light', 1.0)
        time = np.arange(0, config.t_end, config.dt)
        E_history = [E.copy()]

        dx = 1.0 / (nx - 1)
        for t in time[1:]:
            d2E_dx2 = np.gradient(np.gradient(E, dx), dx)
            dE_dt_new = c**2 * d2E_dx2
            E = E + dE_dt * config.dt
            dE_dt = dE_dt_new
            E_history.append(E.copy())

        return {
            'domain': 'electromagnetism',
            'time': time,
            'x': x,
            'electric_field': np.array(E_history),
            'wavelength': 2 * np.pi,
            'frequency': c / (2 * np.pi)
        }

    # ─────────────────────────────────────────────────────────────
    # RELATIVITY SIMULATION
    # ─────────────────────────────────────────────────────────────

    def _simulate_relativity(self, config: SimulationConfig) -> Dict:
        """Simulate relativistic particle motion."""
        initial = config.initial_conditions
        params = config.parameters

        pos = np.array(initial.get('position', [0.0, 0.0]))
        vel = np.array(initial.get('velocity', [0.5, 0.0]))  # As fraction of c
        c = params.get('speed_of_light', 1.0)
        m = params.get('mass', 1.0)

        time = np.arange(0, config.t_end, config.dt)
        positions = [pos.copy()]
        velocities = [vel.copy()]

        # Constant velocity (no forces in flat spacetime)
        for t in time[1:]:
            # Apply time dilation
            gamma = 1.0 / np.sqrt(1 - np.sum(vel**2) / c**2)
            # Proper time dt_proper = dt / gamma
            pos = pos + vel * config.dt / gamma
            positions.append(pos.copy())
            velocities.append(vel.copy())

        return {
            'domain': 'relativity',
            'time': time,
            'positions': np.array(positions),
            'velocities': np.array(velocities),
            'speed_of_light': c,
            'lorentz_factors': 1.0 / np.sqrt(1 - np.sum(velocities, axis=1)**2 / c**2)
        }

    # ─────────────────────────────────────────────────────────────
    # ASTROPHYSICS SIMULATION
    # ─────────────────────────────────────────────────────────────

    def _simulate_astrophysics(self, config: SimulationConfig) -> Dict:
        """Simulate astrophysical system (orbits, etc)."""
        initial = config.initial_conditions
        params = config.parameters

        # Two-body gravitational problem
        m1 = params.get('mass1', 1.0)
        m2 = params.get('mass2', 0.1)
        G = params.get('gravitational_constant', 1.0)

        r1 = np.array(initial.get('position1', [1.0, 0.0]))
        r2 = np.array(initial.get('position2', [-0.1, 0.0]))
        v1 = np.array(initial.get('velocity1', [0.0, 0.5]))
        v2 = np.array(initial.get('velocity2', [0.0, -5.0]))

        time = np.arange(0, config.t_end, config.dt)
        r1_history, r2_history = [r1.copy()], [r2.copy()]

        # Integration
        for t in time[1:]:
            dr = r2 - r1
            r = np.linalg.norm(dr)

            # Gravitational forces
            F = G * m1 * m2 / (r**3 + 1e-10) * dr
            a1 = F / m1
            a2 = -F / m2

            v1 = v1 + a1 * config.dt
            v2 = v2 + a2 * config.dt
            r1 = r1 + v1 * config.dt
            r2 = r2 + v2 * config.dt

            r1_history.append(r1.copy())
            r2_history.append(r2.copy())

        return {
            'domain': 'astrophysics',
            'time': time,
            'position1': np.array(r1_history),
            'position2': np.array(r2_history),
            'masses': (m1, m2),
            'separation': np.linalg.norm(np.array(r1_history) - np.array(r2_history), axis=1)
        }

    # ─────────────────────────────────────────────────────────────
    # PUBLIC INTERFACE
    # ─────────────────────────────────────────────────────────────

    def run_simulation(self, config: SimulationConfig) -> Dict:
        """Run simulation for specified domain."""
        simulator = self.domain_simulators.get(config.domain)
        if not simulator:
            return {'error': f'No simulator for domain: {config.domain}'}

        result = simulator(config)
        self.results[config.domain] = result
        return result

    def get_result(self, domain: str) -> Optional[Dict]:
        """Retrieve simulation result."""
        return self.results.get(domain)


# ============================================================================
# DEMO
# ============================================================================

if __name__ == '__main__':
    print("=" * 80)
    print("PHYSICS SIMULATOR")
    print("=" * 80)
    print()

    simulator = PhysicsSimulator()

    # Example 1: Projectile motion
    print("Example 1: Projectile Motion (Classical Mechanics)")
    print("=" * 80)

    config = SimulationConfig(
        domain='classical_mechanics',
        dt=0.01,
        t_end=3.0,
        method='euler',
        initial_conditions={'position': [0.0, 0.0], 'velocity': [10.0, 20.0]},
        parameters={'mass': 1.0, 'gravity': 9.81}
    )

    result = simulator.run_simulation(config)
    print(f"Simulated {len(result['time'])} time steps")
    print(f"Max height: {np.max(result['positions'][:, 1]):.2f} m")
    print(f"Total energy: {result['energies'][-1]:.2f} J")
    print()

    # Example 2: Quantum wavefunction
    print("Example 2: Quantum Wavefunction Evolution")
    print("=" * 80)

    config = SimulationConfig(
        domain='quantum_mechanics',
        dt=0.01,
        t_end=1.0,
        method='fft',
        initial_conditions={
            'x_range': (-10, 10),
            'center': 0.0,
            'width': 1.0,
            'momentum': 1.0,
            'points': 256
        },
        parameters={'hbar': 1.0, 'mass': 1.0}
    )

    result = simulator.run_simulation(config)
    print(f"Wavefunction evolved over {result['time'][-1]:.2f} time units")
    print(f"Normalization: {result['normalization']:.4f}")
    print()

    # Example 3: Orbital mechanics
    print("Example 3: Orbital Mechanics (Astrophysics)")
    print("=" * 80)

    config = SimulationConfig(
        domain='astrophysics',
        dt=0.001,
        t_end=10.0,
        method='rk4',
        initial_conditions={
            'position1': [1.0, 0.0],
            'position2': [-0.1, 0.0],
            'velocity1': [0.0, 0.5],
            'velocity2': [0.0, -5.0]
        },
        parameters={
            'mass1': 1.0,
            'mass2': 0.1,
            'gravitational_constant': 1.0
        }
    )

    result = simulator.run_simulation(config)
    print(f"Orbital simulation complete")
    print(f"Mean separation: {np.mean(result['separation']):.3f}")
    print(f"Min separation: {np.min(result['separation']):.3f}")
    print(f"Max separation: {np.max(result['separation']):.3f}")
