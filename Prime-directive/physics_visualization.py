#!/usr/bin/env python3
"""
PHYSICS VISUALIZATION SYSTEM

Creates 2D, 3D, and animated visualizations for physics simulations.
Supports trajectory plots, wavefunction visualization, phase space diagrams,
field plots, and real-time animations.
"""

import numpy as np
import matplotlib.pyplot as plt
from matplotlib.animation import FuncAnimation
from mpl_toolkits.mplot3d import Axes3D
from typing import Dict, Optional, Tuple, List
import io
import base64


# ============================================================================
# PHYSICS VISUALIZER
# ============================================================================

class PhysicsVisualizer:
    """Universal visualization system for physics simulation results."""

    def __init__(self, figsize: Tuple[int, int] = (10, 6), style: str = 'seaborn-v0_8-darkgrid'):
        """Initialize visualizer with configuration."""
        self.figsize = figsize
        self.style = style
        try:
            plt.style.use(style)
        except:
            plt.style.use('default')
        self.figures = {}

    # ─────────────────────────────────────────────────────────────
    # 2D PLOTTING
    # ─────────────────────────────────────────────────────────────

    def plot_trajectory(self, result: Dict, filename: Optional[str] = None) -> np.ndarray:
        """Plot 2D trajectory for classical mechanics simulation."""
        if result.get('domain') != 'classical_mechanics':
            return None

        positions = result['positions']
        fig, ax = plt.subplots(figsize=self.figsize)

        # Plot trajectory
        ax.plot(positions[:, 0], positions[:, 1], 'b-', linewidth=2, label='Trajectory')
        ax.scatter([positions[0, 0]], [positions[0, 1]], color='green', s=100, label='Start', zorder=5)
        ax.scatter([positions[-1, 0]], [positions[-1, 1]], color='red', s=100, label='End', zorder=5)

        ax.set_xlabel('X Position (m)')
        ax.set_ylabel('Y Position (m)')
        ax.set_title('Classical Mechanics: Projectile Trajectory')
        ax.legend()
        ax.grid(True, alpha=0.3)

        if filename:
            fig.savefig(filename, dpi=150, bbox_inches='tight')

        return self._fig_to_array(fig)

    def plot_wavefunction(self, result: Dict, time_indices: Optional[List[int]] = None,
                         filename: Optional[str] = None) -> np.ndarray:
        """Plot quantum wavefunction probability evolution."""
        if result.get('domain') != 'quantum_mechanics':
            return None

        x = result['x']
        probability = result['probability']

        if time_indices is None:
            # Select evenly spaced time steps
            n_frames = min(5, len(probability))
            time_indices = np.linspace(0, len(probability) - 1, n_frames, dtype=int)

        fig, axes = plt.subplots(len(time_indices), 1, figsize=(self.figsize[0], 2 * len(time_indices)))
        if len(time_indices) == 1:
            axes = [axes]

        time = result['time']
        for idx, t_idx in enumerate(time_indices):
            ax = axes[idx]
            prob = probability[t_idx]
            ax.plot(x, prob, 'b-', linewidth=2)
            ax.fill_between(x, prob, alpha=0.3)
            ax.set_ylabel('Probability Density')
            ax.set_title(f'Wavefunction at t={time[t_idx]:.3f}')
            ax.grid(True, alpha=0.3)

        axes[-1].set_xlabel('Position (x)')
        fig.suptitle('Quantum Mechanics: Wavefunction Evolution', fontsize=14)
        plt.tight_layout()

        if filename:
            fig.savefig(filename, dpi=150, bbox_inches='tight')

        return self._fig_to_array(fig)

    def plot_phase_space(self, result: Dict, filename: Optional[str] = None) -> np.ndarray:
        """Plot phase space diagram (position vs velocity)."""
        if 'positions' not in result or 'velocities' not in result:
            return None

        positions = result['positions']
        velocities = result['velocities']

        fig, ax = plt.subplots(figsize=self.figsize)

        # Plot trajectory in phase space
        if positions.ndim == 2:
            pos = positions[:, 0]
        else:
            pos = positions
        if velocities.ndim == 2:
            vel = velocities[:, 0]
        else:
            vel = velocities

        scatter = ax.scatter(pos, vel, c=range(len(pos)), cmap='viridis', s=20, alpha=0.6)
        ax.plot(pos, vel, 'k-', alpha=0.3, linewidth=0.5)

        ax.set_xlabel('Position (x)')
        ax.set_ylabel('Velocity (v)')
        ax.set_title('Phase Space Diagram (Position vs Velocity)')
        cbar = plt.colorbar(scatter, ax=ax, label='Time Evolution')
        ax.grid(True, alpha=0.3)

        if filename:
            fig.savefig(filename, dpi=150, bbox_inches='tight')

        return self._fig_to_array(fig)

    def plot_energy(self, result: Dict, filename: Optional[str] = None) -> np.ndarray:
        """Plot energy evolution over time."""
        if 'energies' not in result or 'time' not in result:
            return None

        time = result['time']
        energies = result['energies']

        fig, ax = plt.subplots(figsize=self.figsize)

        ax.plot(time[1:len(energies)+1], energies, 'r-', linewidth=2, label='Total Energy')
        ax.set_xlabel('Time (s)')
        ax.set_ylabel('Energy (J)')
        ax.set_title('Energy Evolution')
        ax.legend()
        ax.grid(True, alpha=0.3)

        if filename:
            fig.savefig(filename, dpi=150, bbox_inches='tight')

        return self._fig_to_array(fig)

    def plot_field_2d(self, result: Dict, time_index: int = -1,
                     filename: Optional[str] = None) -> np.ndarray:
        """Plot 2D field (density, temperature, etc) at specific time."""
        if 'density' in result:
            field_data = result['density']
            field_name = 'Density'
        elif 'temperature' in result:
            field_data = result['temperature']
            field_name = 'Temperature'
        elif 'electric_field' in result:
            field_data = result['electric_field']
            field_name = 'Electric Field'
        else:
            return None

        # Get field at specified time
        if field_data.ndim == 3:
            field = field_data[time_index]
        else:
            field = field_data

        fig, ax = plt.subplots(figsize=self.figsize)

        im = ax.contourf(field, levels=20, cmap='viridis')
        cbar = plt.colorbar(im, ax=ax, label=field_name)
        ax.set_xlabel('X Grid Point')
        ax.set_ylabel('Y Grid Point')
        ax.set_title(f'{field_name} Distribution at t={time_index}')

        if filename:
            fig.savefig(filename, dpi=150, bbox_inches='tight')

        return self._fig_to_array(fig)

    # ─────────────────────────────────────────────────────────────
    # 3D PLOTTING
    # ─────────────────────────────────────────────────────────────

    def plot_3d_trajectory(self, result: Dict, filename: Optional[str] = None) -> np.ndarray:
        """Plot 3D trajectory for astrophysics simulations."""
        if 'position1' not in result and 'position2' not in result:
            if result.get('domain') != 'astrophysics':
                return None

        fig = plt.figure(figsize=self.figsize)
        ax = fig.add_subplot(111, projection='3d')

        # Plot primary body
        if 'position1' in result:
            pos1 = result['position1']
            # Project to 2D if needed (add z=0)
            if pos1.shape[1] == 2:
                z1 = np.zeros(len(pos1))
                ax.plot(pos1[:, 0], pos1[:, 1], z1, 'b-', linewidth=2, label='Body 1')
            else:
                ax.plot(pos1[:, 0], pos1[:, 1], pos1[:, 2], 'b-', linewidth=2, label='Body 1')
            ax.scatter([pos1[0, 0]], [pos1[0, 1]], [0 if pos1.shape[1] == 2 else pos1[0, 2]],
                      color='blue', s=100)

        # Plot secondary body
        if 'position2' in result:
            pos2 = result['position2']
            if pos2.shape[1] == 2:
                z2 = np.zeros(len(pos2))
                ax.plot(pos2[:, 0], pos2[:, 1], z2, 'r-', linewidth=2, label='Body 2')
            else:
                ax.plot(pos2[:, 0], pos2[:, 1], pos2[:, 2], 'r-', linewidth=2, label='Body 2')
            ax.scatter([pos2[0, 0]], [pos2[0, 1]], [0 if pos2.shape[1] == 2 else pos2[0, 2]],
                      color='red', s=100)

        ax.set_xlabel('X Position')
        ax.set_ylabel('Y Position')
        if 'position1' in result and result['position1'].shape[1] > 2:
            ax.set_zlabel('Z Position')
        ax.set_title('3D Orbital Trajectories')
        ax.legend()
        ax.grid(True, alpha=0.3)

        if filename:
            fig.savefig(filename, dpi=150, bbox_inches='tight')

        return self._fig_to_array(fig)

    # ─────────────────────────────────────────────────────────────
    # ANIMATIONS
    # ─────────────────────────────────────────────────────────────

    def animate_trajectory(self, result: Dict, filename: Optional[str] = None,
                          fps: int = 30) -> Optional[str]:
        """Create animation of classical trajectory."""
        if result.get('domain') != 'classical_mechanics':
            return None

        positions = result['positions']
        time = result['time']

        fig, ax = plt.subplots(figsize=self.figsize)
        ax.set_xlim(positions[:, 0].min() - 1, positions[:, 0].max() + 1)
        ax.set_ylim(positions[:, 1].min() - 1, positions[:, 1].max() + 1)
        line, = ax.plot([], [], 'b-', linewidth=2)
        point, = ax.plot([], [], 'ro', markersize=8)
        ax.set_xlabel('X Position (m)')
        ax.set_ylabel('Y Position (m)')
        ax.set_title('Classical Mechanics: Trajectory Animation')
        ax.grid(True, alpha=0.3)

        def animate(frame):
            end_idx = min(frame + 1, len(positions))
            line.set_data(positions[:end_idx, 0], positions[:end_idx, 1])
            point.set_data([positions[frame, 0]], [positions[frame, 1]])
            return line, point

        anim = FuncAnimation(fig, animate, frames=len(positions), interval=1000/fps, blit=True)

        if filename:
            try:
                anim.save(filename, writer='ffmpeg', fps=fps)
            except:
                # Fallback if ffmpeg not available
                pass

        return "Animation created successfully"

    def animate_wavefunction(self, result: Dict, filename: Optional[str] = None,
                            fps: int = 30) -> Optional[str]:
        """Create animation of quantum wavefunction evolution."""
        if result.get('domain') != 'quantum_mechanics':
            return None

        x = result['x']
        probability = result['probability']
        time = result['time']

        fig, ax = plt.subplots(figsize=self.figsize)
        ax.set_xlim(x.min(), x.max())
        ax.set_ylim(0, probability.max() * 1.1)
        line, = ax.plot([], [], 'b-', linewidth=2)
        time_text = ax.text(0.02, 0.95, '', transform=ax.transAxes)
        ax.set_xlabel('Position (x)')
        ax.set_ylabel('Probability Density')
        ax.set_title('Quantum Mechanics: Wavefunction Evolution')
        ax.grid(True, alpha=0.3)

        def animate(frame):
            line.set_data(x, probability[frame])
            time_text.set_text(f't = {time[frame]:.3f}')
            return line, time_text

        anim = FuncAnimation(fig, animate, frames=len(probability), interval=1000/fps, blit=True)

        if filename:
            try:
                anim.save(filename, writer='ffmpeg', fps=fps)
            except:
                pass

        return "Animation created successfully"

    # ─────────────────────────────────────────────────────────────
    # MULTI-PANEL PLOTS
    # ─────────────────────────────────────────────────────────────

    def plot_simulation_summary(self, result: Dict, filename: Optional[str] = None) -> np.ndarray:
        """Create comprehensive multi-panel visualization of simulation."""
        domain = result.get('domain', 'unknown')

        if domain == 'classical_mechanics':
            fig, axes = plt.subplots(2, 2, figsize=(14, 10))

            # Trajectory
            positions = result['positions']
            axes[0, 0].plot(positions[:, 0], positions[:, 1], 'b-', linewidth=2)
            axes[0, 0].scatter([positions[0, 0]], [positions[0, 1]], color='green', s=100)
            axes[0, 0].scatter([positions[-1, 0]], [positions[-1, 1]], color='red', s=100)
            axes[0, 0].set_xlabel('X Position')
            axes[0, 0].set_ylabel('Y Position')
            axes[0, 0].set_title('Trajectory')
            axes[0, 0].grid(True, alpha=0.3)

            # Phase space
            velocities = result['velocities']
            axes[0, 1].scatter(positions[:, 0], velocities[:, 0], c=range(len(positions)),
                             cmap='viridis', s=20, alpha=0.6)
            axes[0, 1].set_xlabel('Position')
            axes[0, 1].set_ylabel('Velocity')
            axes[0, 1].set_title('Phase Space')
            axes[0, 1].grid(True, alpha=0.3)

            # Energy
            time = result['time']
            energies = result['energies']
            axes[1, 0].plot(time[1:len(energies)+1], energies, 'r-', linewidth=2)
            axes[1, 0].set_xlabel('Time')
            axes[1, 0].set_ylabel('Energy')
            axes[1, 0].set_title('Energy Evolution')
            axes[1, 0].grid(True, alpha=0.3)

            # Speed
            speeds = np.linalg.norm(velocities, axis=1)
            axes[1, 1].plot(time, speeds, 'g-', linewidth=2)
            axes[1, 1].set_xlabel('Time')
            axes[1, 1].set_ylabel('Speed')
            axes[1, 1].set_title('Speed Evolution')
            axes[1, 1].grid(True, alpha=0.3)

            fig.suptitle(f'Classical Mechanics Simulation Summary', fontsize=14)

        elif domain == 'quantum_mechanics':
            fig, axes = plt.subplots(2, 2, figsize=(14, 10))

            x = result['x']
            probability = result['probability']
            time = result['time']

            # Wavefunction at different times
            time_indices = [0, len(probability)//3, 2*len(probability)//3, -1]
            for idx, t_idx in enumerate(time_indices):
                ax_idx = divmod(idx, 2)
                ax = axes[ax_idx]
                ax.plot(x, probability[t_idx], 'b-', linewidth=2)
                ax.fill_between(x, probability[t_idx], alpha=0.3)
                ax.set_ylabel('Probability')
                ax.set_title(f't = {time[t_idx]:.3f}')
                ax.grid(True, alpha=0.3)

            axes[1, 0].set_xlabel('Position')
            axes[1, 1].set_xlabel('Position')
            fig.suptitle('Quantum Mechanics Simulation Summary', fontsize=14)

        else:
            # Generic summary for other domains
            fig, ax = plt.subplots(figsize=self.figsize)
            ax.text(0.5, 0.5, f'Simulation Summary\nDomain: {domain}',
                   ha='center', va='center', fontsize=12, transform=ax.transAxes)
            ax.axis('off')

        plt.tight_layout()

        if filename:
            fig.savefig(filename, dpi=150, bbox_inches='tight')

        return self._fig_to_array(fig)

    # ─────────────────────────────────────────────────────────────
    # UTILITY METHODS
    # ─────────────────────────────────────────────────────────────

    def _fig_to_array(self, fig) -> np.ndarray:
        """Convert matplotlib figure to numpy array."""
        fig.canvas.draw()
        image_data = fig.canvas.tostring_rgb()
        width, height = fig.canvas.get_width_height()
        image_array = np.frombuffer(image_data, dtype=np.uint8).reshape(height, width, 3)
        plt.close(fig)
        return image_array

    def save_figure(self, figure_name: str, filename: str) -> bool:
        """Save stored figure to file."""
        if figure_name in self.figures:
            array = self.figures[figure_name]
            fig, ax = plt.subplots(figsize=self.figsize)
            ax.imshow(array)
            ax.axis('off')
            fig.savefig(filename, dpi=150, bbox_inches='tight')
            plt.close(fig)
            return True
        return False

    def get_figure_base64(self, figure_name: str) -> Optional[str]:
        """Get figure as base64 string for embedding in web."""
        if figure_name in self.figures:
            array = self.figures[figure_name]
            fig, ax = plt.subplots(figsize=self.figsize)
            ax.imshow(array)
            ax.axis('off')

            buffer = io.BytesIO()
            fig.savefig(buffer, format='png', dpi=100, bbox_inches='tight')
            buffer.seek(0)
            image_base64 = base64.b64encode(buffer.read()).decode()
            plt.close(fig)
            return image_base64

        return None


# ============================================================================
# DEMO
# ============================================================================

if __name__ == '__main__':
    print("=" * 80)
    print("PHYSICS VISUALIZATION SYSTEM")
    print("=" * 80)
    print()

    # Example with classical mechanics simulation
    from physics_simulator import PhysicsSimulator, SimulationConfig

    simulator = PhysicsSimulator()
    visualizer = PhysicsVisualizer()

    # Create projectile motion simulation
    print("Creating projectile motion simulation...")
    config = SimulationConfig(
        domain='classical_mechanics',
        dt=0.01,
        t_end=3.0,
        method='euler',
        initial_conditions={'position': [0.0, 0.0], 'velocity': [10.0, 20.0]},
        parameters={'mass': 1.0, 'gravity': 9.81}
    )

    result = simulator.run_simulation(config)
    print(f"Simulation complete: {len(result['positions'])} time steps")

    # Create visualizations
    print("\nGenerating visualizations...")

    # Trajectory plot
    print("  • Plotting trajectory...")
    visualizer.plot_trajectory(result, filename='/tmp/trajectory.png')

    # Phase space plot
    print("  • Plotting phase space...")
    visualizer.plot_phase_space(result, filename='/tmp/phase_space.png')

    # Energy plot
    print("  • Plotting energy evolution...")
    visualizer.plot_energy(result, filename='/tmp/energy.png')

    # Summary plot
    print("  • Creating summary visualization...")
    visualizer.plot_simulation_summary(result, filename='/tmp/summary.png')

    print("\n✅ Visualizations saved to /tmp/")
    print("   - /tmp/trajectory.png")
    print("   - /tmp/phase_space.png")
    print("   - /tmp/energy.png")
    print("   - /tmp/summary.png")
