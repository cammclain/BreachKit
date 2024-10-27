#!/bin/bash

# Fail on errors
set -e

# Ensure the Tor directory exists and has the correct ownership
mkdir -p /var/lib/tor/hidden_service
chown -R debian-tor:debian-tor /var/lib/tor/hidden_service
chmod 700 /var/lib/tor/hidden_service

# Start Tor
su -s /bin/bash debian-tor -c "tor &"

# Wait for Tor to generate the .onion address
echo "Waiting for Tor to generate the .onion address..."
for i in {1..30}; do
    if [ -f /var/lib/tor/hidden_service/hostname ]; then
        ONION_ADDRESS=$(cat /var/lib/tor/hidden_service/hostname)
        echo "Generated Onion Address: $ONION_ADDRESS"
        break
    fi
    sleep 1
done

if [ -z "$ONION_ADDRESS" ]; then
    echo "Failed to generate the .onion address. Exiting..."
    exit 1
fi

# Update the NGINX config with the .onion address
sed -i "s/server_name _;/server_name $ONION_ADDRESS;/g" /etc/nginx/nginx.conf

# Start NGINX
nginx -g "daemon off;"
