#!/bin/bash

# Fail on errors
set -e

# Start Tor in the background
tor &

# Wait for Tor to initialize
sleep 5

# Export PYTHONPATH
export PYTHONPATH=/ollama:$PYTHONPATH

# Run the Ollama ASGI app with exec to forward signals correctly
exec python3 /ollama/app/asgi.py
