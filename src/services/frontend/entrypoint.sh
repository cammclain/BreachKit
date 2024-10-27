#!/bin/bash

set -e  # Exit on errors

# Ensure the hidden service directory has the correct ownership
sudo chown -R debian-tor:debian-tor /var/lib/tor/hidden_service/

# Start Tor in the background
echo "Starting Tor..."
sudo -u debian-tor tor &

# Wait for Tor to initialize and generate the .onion address
echo "Waiting for Tor to generate the .onion address..."
for i in {1..30}; do
    if [ -f /var/lib/tor/hidden_service/hostname ]; then
        echo "Tor hidden service is ready!"
        ONION_ADDRESS=$(cat /var/lib/tor/hidden_service/hostname)
        echo "Generated Onion Address: $ONION_ADDRESS"
        break
    fi
    sleep 1
done

# If the .onion address is not generated, exit with an error
if [ -z "$ONION_ADDRESS" ]; then
    echo "Failed to generate the .onion address. Exiting..."
    exit 1
fi

# Update the Nginx configuration with the .onion address
echo "Updating Nginx configuration with the .onion address..."
sed -i "s/server_name _;/server_name $ONION_ADDRESS;/g" /etc/nginx/nginx.conf

# Start Nginx in the foreground
echo "Starting Nginx..."
nginx -g "daemon off;"
