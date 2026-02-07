# Build: docker build -t empathy-module:offline .
# Run:   docker run --rm --gpus all -v $(pwd):/workspace empathy-module:offline python demo_empathy.py

FROM rocm/pytorch:latest

WORKDIR /workspace

# Install system dependencies (all from cache, no internet during build)
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy Python module
COPY ising_empathy_module.py .
COPY gpu_agi_100_signifiers_test.py .
COPY gpu_all_tests.py .
COPY gpu_comprehensive_test.py .
COPY demo_empathy.py .

# Set environment for ROCm
ENV HIP_VISIBLE_DEVICES=0
ENV HSA_OVERRIDE_GFX_VERSION=11.0.0

# Default command: run demo
CMD ["python", "demo_empathy.py"]
