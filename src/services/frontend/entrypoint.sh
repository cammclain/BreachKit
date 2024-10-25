#!/bin/bash

# Fail on errors
set -e

# Ensure the Tor directory has the correct ownership
chown -R debian-tor:debian-tor /var/lib/tor/hidden_service/

# Start Tor in the background
tor &

# Wait for Tor to initialize and create the hostname file
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

# If the hostname is still not available, exit with an error
if [ -z "$ONION_ADDRESS" ]; then
    echo "Failed to generate the .onion address. Exiting..."
    exit 1
fi

# Update the NGINX config with the generated .onion address
sed -i "s/server_name _;/server_name $ONION_ADDRESS;/g" /etc/nginx/nginx.conf

# Start NGINX in the foreground
nginx -g "daemon off;"
