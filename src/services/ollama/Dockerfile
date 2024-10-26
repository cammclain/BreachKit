# Use Debian Slim as the base image
FROM debian:bullseye-slim

# Set environment variables to prevent interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies (including Tor)
RUN apt-get update && apt-get install -y \
    python3 python3-pip python3-venv vim curl tor torsocks && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install Ollama CLI
RUN curl -fsSL https://ollama.com/install.sh | sh

# Create the working directory
WORKDIR /ollama

# Create a virtual environment and activate it
RUN python3 -m venv venv && \
    . venv/bin/activate && \
    pip install --upgrade pip

# Copy requirements.txt and install dependencies inside the virtual environment
COPY requirements.txt /ollama/requirements.txt
RUN . venv/bin/activate && \
    pip install -r /ollama/requirements.txt --no-cache-dir

# Copy the entire application folder (avoiding redundant copies)
COPY app/ /ollama/app/

# Copy the entrypoint script and make it executable
COPY entrypoint.sh /ollama/entrypoint.sh
RUN chmod +x /ollama/entrypoint.sh

# Expose the necessary port
EXPOSE 11434

# Run the entrypoint script
ENTRYPOINT ["/ollama/entrypoint.sh"]
