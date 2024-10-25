#!/bin/bash

# Paths to required files
TOR_HOSTNAME_FILE="/var/lib/tor/hidden_service/hostname"
PROSODY_CONFIG="/etc/prosody/prosody.cfg.lua"

# Wait until Tor generates the hidden service hostname
echo "Waiting for Tor hidden service to generate hostname..."
while [ ! -f "$TOR_HOSTNAME_FILE" ]; do
    sleep 2
done

# Read the hidden service hostname
TOR_HOSTNAME=$(cat $TOR_HOSTNAME_FILE)
echo "Tor hidden service hostname generated: $TOR_HOSTNAME"

# Update the Prosody configuration file with the VirtualHost
echo "Setting VirtualHost in Prosody config..."
sed -i "/^VirtualHost /c\VirtualHost \"$TOR_HOSTNAME\"" "$PROSODY_CONFIG"

echo "Prosody configuration updated with VirtualHost: $TOR_HOSTNAME"

# Start Prosody
echo "Starting Prosody..."
prosodyctl start

# Keep the container running
tail -f /dev/null
