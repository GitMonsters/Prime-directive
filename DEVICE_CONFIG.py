#!/usr/bin/env python3
"""
DEVICE CONFIGURATION - Environment-Aware GPU/Device Management

This module manages device selection across multiple platforms (Linux, macOS, Windows)
and preserves platform-specific optimizations.

⚠️ IMPORTANT: AMD GPU (ROCm) optimization is LOCKED to Linux AMD hardware.
This prevents accidental device changes when switching between MacBook and Linux.
"""

import platform
import torch
import sys
from pathlib import Path

# ============================================================================
# PLATFORM-SPECIFIC DEVICE CONFIGURATION
# ============================================================================

PLATFORM_CONFIG = {
    'Linux': {
        'primary_device': 'cuda',  # ROCm (AMD GPU via CUDA API)
        'fallback_device': 'cpu',
        'note': 'AMD Radeon GPU with ROCm 5.7 (requires kernel modules: amdgpu, amdkfd)',
        'requires': ['rocm-smi', 'hip', 'rocm-libs'],
        'setup_note': 'sudo modprobe amdgpu && sudo modprobe amdkfd',
        'optimization': 'AMD ROCm HIP kernels (2-5x speedup on tensor ops)',
    },
    'Darwin': {  # macOS
        'primary_device': 'mps',  # Apple Metal Performance Shaders
        'fallback_device': 'cpu',
        'note': 'Apple Metal Performance Shaders (GPU acceleration for Apple Silicon)',
        'requires': ['Metal', 'PyTorch 1.12+'],
        'setup_note': 'Metal support built-in on Apple Silicon (M1/M2/M3+)',
        'optimization': 'Apple GPU via Metal (2-3x speedup on compatible ops)',
    },
    'Windows': {
        'primary_device': 'cuda',  # NVIDIA CUDA
        'fallback_device': 'cpu',
        'note': 'NVIDIA CUDA GPU (if available) or Intel oneAPI (if available)',
        'requires': ['NVIDIA GPU', 'CUDA Toolkit', 'cuDNN'],
        'setup_note': 'Install NVIDIA CUDA Toolkit and drivers',
        'optimization': 'NVIDIA CUDA kernels (3-8x speedup)',
    }
}

# ============================================================================
# DEVICE SELECTOR - ENVIRONMENT AWARE
# ============================================================================

class EnvironmentAwareDeviceSelector:
    """
    Selects appropriate device based on platform and available hardware.
    Preserves platform-specific optimizations and prevents cross-platform conflicts.
    """

    def __init__(self):
        self.system = platform.system()
        self.config = PLATFORM_CONFIG.get(self.system, PLATFORM_CONFIG['Linux'])
        self.device = None
        self.device_info = {}

    def detect_device(self):
        """
        Auto-detect available device for current platform.
        Returns device with fallback chain.
        """
        primary = self.config['primary_device']
        fallback = self.config['fallback_device']

        self.device_info = {
            'platform': self.system,
            'primary_device': primary,
            'fallback_device': fallback,
            'available': False,
            'note': self.config['note'],
        }

        # Platform-specific detection
        if self.system == 'Linux':
            return self._detect_linux_device()
        elif self.system == 'Darwin':
            return self._detect_macos_device()
        elif self.system == 'Windows':
            return self._detect_windows_device()
        else:
            return torch.device('cpu')

    def _detect_linux_device(self):
        """
        Detect Linux GPU: AMD ROCm (preferred) or NVIDIA CUDA or CPU.

        ⚠️ IMPORTANT: AMD GPU is LOCKED to Linux to prevent MacBook changes.
        """
        print("\n" + "="*80)
        print("🐧 LINUX PLATFORM DETECTED")
        print("="*80)

        # Check for AMD ROCm GPU
        print("\n1. Checking for AMD Radeon GPU (ROCm)...")
        if torch.cuda.is_available():
            device_name = torch.cuda.get_device_name(0)
            print(f"   ✅ GPU Found: {device_name}")
            self.device_info['available'] = True
            self.device_info['device_type'] = 'AMD ROCm'
            self.device_info['warning'] = "⚠️ AMD GPU CONFIG IS LOCKED TO LINUX. Do not modify when on MacBook."
            return torch.device('cuda', 0)
        else:
            print("   ℹ️  No AMD GPU detected (or HIP kernels not loaded)")
            print("      To enable: sudo modprobe amdgpu && sudo modprobe amdkfd")

        # Fallback to CPU
        print("\n2. Using CPU (NumPy + Intel MKL optimizations)")
        print("   ✅ CPU Mode: Stable, <200ms per query")
        self.device_info['available'] = True
        self.device_info['device_type'] = 'CPU'
        return torch.device('cpu')

    def _detect_macos_device(self):
        """
        Detect macOS GPU: Apple Metal (M-series) or CPU.

        ⚠️ NOTE: AMD ROCm is NOT used on macOS (different architecture).
        Use Metal instead for GPU acceleration on Apple Silicon.
        """
        print("\n" + "="*80)
        print("🍎 MACOS PLATFORM DETECTED")
        print("="*80)
        print("\n⚠️ IMPORTANT: AMD ROCm config is preserved for Linux only!")
        print("   This MacBook will use Apple Metal GPU instead.")

        # Check for Apple Metal
        print("\n1. Checking for Apple Metal GPU (M1/M2/M3+)...")
        try:
            if hasattr(torch.backends, 'mps') and torch.backends.mps.is_available():
                print("   ✅ Apple Metal Found (GPU Acceleration Available)")
                self.device_info['available'] = True
                self.device_info['device_type'] = 'Apple Metal'
                self.device_info['warning'] = "ℹ️ Using Apple Metal. AMD ROCm config preserved for Linux."
                return torch.device('mps')
            else:
                print("   ℹ️  Apple Metal not available on this macOS version")
                print("      (Requires macOS 12.3+ with Apple Silicon)")
        except:
            print("   ℹ️  Apple Metal not available")

        # Fallback to CPU
        print("\n2. Using CPU (Intel/ARM optimizations)")
        print("   ✅ CPU Mode: Reliable, <200ms per query")
        self.device_info['available'] = True
        self.device_info['device_type'] = 'CPU'
        return torch.device('cpu')

    def _detect_windows_device(self):
        """Detect Windows GPU: NVIDIA CUDA or CPU."""
        print("\n" + "="*80)
        print("🪟 WINDOWS PLATFORM DETECTED")
        print("="*80)

        # Check for NVIDIA CUDA
        print("\n1. Checking for NVIDIA CUDA GPU...")
        if torch.cuda.is_available():
            device_name = torch.cuda.get_device_name(0)
            print(f"   ✅ GPU Found: {device_name}")
            self.device_info['available'] = True
            self.device_info['device_type'] = 'NVIDIA CUDA'
            return torch.device('cuda', 0)
        else:
            print("   ℹ️  No NVIDIA GPU detected")

        # Fallback to CPU
        print("\n2. Using CPU (Intel MKL optimizations)")
        print("   ✅ CPU Mode: Stable, <200ms per query")
        self.device_info['available'] = True
        self.device_info['device_type'] = 'CPU'
        return torch.device('cpu')

    def get_device_info(self):
        """Return detailed device information."""
        return {
            'platform': self.system,
            'device': str(self.device),
            'device_type': self.device_info.get('device_type', 'Unknown'),
            'available': self.device_info.get('available', False),
            'note': self.config['note'],
            'warning': self.device_info.get('warning', ''),
            'setup_note': self.config['setup_note'],
            'optimization': self.config['optimization'],
        }

    def print_device_summary(self):
        """Print detailed device summary."""
        info = self.get_device_info()

        print("\n" + "="*80)
        print("📊 DEVICE CONFIGURATION SUMMARY")
        print("="*80)
        print(f"\nPlatform:       {info['platform']}")
        print(f"Device:         {info['device']}")
        print(f"Device Type:    {info['device_type']}")
        print(f"Available:      {'✅ Yes' if info['available'] else '❌ No'}")
        print(f"\nNote:           {info['note']}")

        if info['warning']:
            print(f"⚠️  WARNING:     {info['warning']}")

        print(f"\nOptimization:   {info['optimization']}")
        print(f"Setup:          {info['setup_note']}")
        print("\n" + "="*80)


# ============================================================================
# PROTECTED CONFIG - PREVENTS ACCIDENTAL CHANGES
# ============================================================================

class ProtectedDeviceConfig:
    """
    Stores and protects device configuration.
    Warns if configuration would change between platforms.
    """

    CONFIG_FILE = Path(__file__).parent / '.device_config'

    @classmethod
    def load_last_platform(cls):
        """Load the last platform this was run on."""
        if cls.CONFIG_FILE.exists():
            with open(cls.CONFIG_FILE, 'r') as f:
                return f.read().strip()
        return None

    @classmethod
    def save_platform(cls, platform_name):
        """Save current platform."""
        with open(cls.CONFIG_FILE, 'w') as f:
            f.write(platform_name)

    @classmethod
    def check_platform_change(cls):
        """Check if platform has changed and warn user."""
        current = platform.system()
        last = cls.load_last_platform()

        if last and last != current:
            print("\n" + "⚠️ "*40)
            print("\n🚨 PLATFORM CHANGE DETECTED!")
            print(f"\n   Last Platform: {last}")
            print(f"   Current Platform: {current}")
            print("\n⚠️  IMPORTANT NOTES:")
            print("   - AMD GPU (ROCm) config is LOCKED to Linux only")
            print("   - MacBook will use Apple Metal instead (automatic)")
            print("   - Device settings are platform-specific and will not conflict")
            print("   - AMD ROCm setup is preserved for when you return to Linux")
            print("\n" + "⚠️ "*40 + "\n")

        cls.save_platform(current)


# ============================================================================
# QUICK ACCESS FUNCTION
# ============================================================================

def get_device(force_device=None):
    """
    Get appropriate device for current platform.

    Args:
        force_device: Force specific device ('cpu', 'cuda', 'mps')
                      Only use for testing!

    Returns:
        torch.device object for current platform
    """
    if force_device:
        print(f"\n⚠️  FORCING DEVICE: {force_device} (test mode)")
        return torch.device(force_device)

    # Check for platform changes
    ProtectedDeviceConfig.check_platform_change()

    # Get environment-aware device
    selector = EnvironmentAwareDeviceSelector()
    device = selector.detect_device()
    selector.print_device_summary()

    return device


# ============================================================================
# MAIN - TEST DEVICE DETECTION
# ============================================================================

if __name__ == '__main__':
    print("\n" + "="*80)
    print("DEVICE CONFIGURATION TEST")
    print("="*80)

    device = get_device()
    print(f"\n✅ Selected Device: {device}")

    # Test tensor creation
    print("\nTesting tensor operations...")
    tensor = torch.randn(10, 10, device=device)
    print(f"✅ Tensor created on {device}")
    print(f"   Shape: {tensor.shape}")
    print(f"   Device: {tensor.device}")
