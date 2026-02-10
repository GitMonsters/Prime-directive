FROM python:3.10-slim

WORKDIR /workspace

# Install system dependencies
RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Copy requirements
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy application code
COPY agentrl_service.py .
COPY agentrl_wrapper.py .

# Create directories for logs and models
RUN mkdir -p /workspace/logs /workspace/models

# Expose port
EXPOSE 8888

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8888/health || exit 1

# Environment variables
ENV LOG_LEVEL=INFO
ENV SERVICE_HOST=0.0.0.0
ENV SERVICE_PORT=8888
ENV BATCH_SIZE=32
ENV MAX_TRAJECTORY_BUFFER=1000
ENV LEARNING_RATE=0.0001

# Run service
CMD ["python", "agentrl_service.py"]
