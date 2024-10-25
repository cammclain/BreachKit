#!/bin/bash

# Fail on errors
set -e

# Ensure Tor directory has the correct ownership
chown -R debian-tor:debian-tor /var/lib/tor/hidden_service/

# Start Tor in the background
tor &

# Wait for Tor to initialize and create the hostname file
sleep 10

# Print the Tor hostname
cat /var/lib/tor/hidden_service/hostname

# Start NGINX in the foreground
nginx -g "daemon off;"
