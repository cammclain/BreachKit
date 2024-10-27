#!/bin/bash

# Fail the script on any error
set -e

# Ensure Tor hidden service directory has the correct ownership
chown -R debian-tor:debian-tor /var/lib/tor/hidden_service/

# Start Tor in the background
su -s /bin/bash debian-tor -c "tor &"

sleep 35

# Wait until the Tor hostname file is created (max 30s)
echo "Waiting for Tor hidden service to initialize..."
for i in {1..30}; do
    if [ -f /var/lib/tor/hidden_service/hostname ]; then
        echo "Tor hidden service is ready!"
        cat /var/lib/tor/hidden_service/hostname
        break
    fi
    sleep 5
done

# Check if the virtual environment exists
if [ ! -d "venv" ]; then
    echo "Virtual environment not found. Exiting..."
    exit 1
fi

# Activate the virtual environment
source venv/bin/activate

# Run the Litestar application, binding only to localhost (safer with Tor)
uvicorn src.breachkit_core.asgi:app --host 0.0.0.0 --port 8003
