# Use an official Python image from the Docker Hub
FROM debian:latest

RUN apt-get update && apt-get install -y \
    python3 python3-pip vim python3-venv build-essential libssl-dev libffi-dev python3-dev tor torsocks curl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /breachkit_core

COPY requirements.txt .

# Create the virtual environment
RUN python3 -m venv venv

# Set the virtual environment as the active environment
ENV PATH="/breachkit_core/venv/bin:$PATH"

# Install Python dependencies
RUN pip install -r requirements.txt --no-cache-dir

COPY . .

# Expose the backend port
EXPOSE 8003

# Use an entrypoint script for better startup control
ENTRYPOINT ["/breachkit_core/entrypoint.sh"]
