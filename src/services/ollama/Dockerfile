# Use the appropriate base image for Ollama (replace this with the correct image if required)
FROM debian:bullseye-slim

# Install any necessary dependencies
RUN apt-get update && apt-get install -y \
    python3 python3-pip vim curl tor torsocks && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install Ollama
RUN curl -fsSL https://ollama.com/install.sh | sh

# Verify the installation
RUN ollama version

# Set the working directory
WORKDIR /ollama

# Copy model files into the container
COPY app/models/ /ollama/models/

# Expose the required port
EXPOSE 11434

# Install Python dependencies (if needed)
RUN pip install torch transformers  # Adjust packages as necessary

# Copy the entrypoint script or server code (adjust this if necessary)
COPY app/asgi.py /ollama/app/asgi.py

# Start the server (replace this with your actual command to run Ollama)
CMD ["python3", "/ollama/app/asgi.py"]
