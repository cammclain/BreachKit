#!/bin/bash

# Fail on any error
set -e

# Start Tor in the background
tor &

# Wait for Tor to initialize
sleep 5

# Activate the virtual environment
source /ollama/venv/bin/activate

# Set Python path
export PYTHONPATH=/ollama:$PYTHONPATH

# Start the ASGI server
exec litestar run --app "src.breachkit_core.asgi:app" --host 127.0.0.1 --port 8003
