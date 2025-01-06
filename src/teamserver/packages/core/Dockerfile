# ===================================================================
# Core Service Dockerfile
# This Dockerfile creates a Docker image for running the core loco service
# ===================================================================

# Start with Debian Bookworm Slim as the base image
# We use debian-slim since it provides a good balance of size and functionality
FROM debian-slim:bookworm

# Set the working directory inside the container to /app
# All subsequent commands will run from this directory
WORKDIR /app

# Update apt package list and install curl
# apt-get update: Updates the package index
# apt-get install -y curl: Installs curl without prompting for confirmation (-y flag)
RUN apt-get update && apt-get install -y curl

# Install Rust using rustup
# --proto '=https': Only use HTTPS protocol
# --tlsv1.2: Use TLS v1.2 for security
# -sSf: Silent mode, show errors, fail on errors
# | sh: Pipe the downloaded script to shell for execution
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source the Rust environment variables
# This adds cargo and rustc to the PATH
RUN source $HOME/.cargo/env

# Copy all files from the build context into the container
# The '.' means copy from current directory on host
# The second '.' means copy to current working directory in container (/app)
COPY . .

# Install the Loco framework CLI tool
# This is needed to run and manage the Loco application
RUN cargo install loco

# Install the Sea-ORM CLI tool
# Sea-ORM is the ORM we use for database operations
RUN cargo install sea-orm-cli

# Define the command to run when container starts
# cargo run --release: Runs the Rust application in release mode
# Release mode means optimized, production-ready build
CMD ["cargo", "run", "--release"]