#!/bin/bash

# Fail on errors
set -e

# Start Tor service in the background (optional)
service tor start

# Check if the virtual environment exists
if [ ! -d "venv" ]; then
    echo "Virtual environment not found. Exiting..."
    exit 1
fi

# Activate the virtual environment
source venv/bin/activate

# Run the Litestar application
exec litestar run --app src.breachkit_core.asgi:app --host 0.0.0.0 --port 8003 # TODO: Set this to localhost & use tor 

