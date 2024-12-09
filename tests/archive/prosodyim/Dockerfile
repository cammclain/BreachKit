# Use the official Debian base image
FROM debian:bullseye-slim

# Install necessary packages (including Prosody and Tor)
RUN apt-get update && apt-get install -y \
    prosody \
    tor \
    vim curl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Expose necessary ports for Prosody (e.g., XMPP, BOSH)
EXPOSE 5222 5269 5280

# Create directories for Tor hidden service
RUN mkdir -p /var/lib/tor/hidden_service

# Copy configuration files for Prosody and Tor
COPY prosody.cfg.lua /etc/prosody/prosody.cfg.lua
COPY torrc /etc/tor/torrc

# Copy the setup script
COPY setup-prosody.sh /usr/local/bin/setup-prosody.sh
RUN chmod +x /usr/local/bin/setup-prosody.sh

# Set permissions for Tor hidden service
RUN chown -R debian-tor:debian-tor /var/lib/tor/hidden_service && \
    chmod 700 /var/lib/tor/hidden_service

# Use the setup script as the entrypoint
CMD ["/usr/local/bin/setup-prosody.sh"]
