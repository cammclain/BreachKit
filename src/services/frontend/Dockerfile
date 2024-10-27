FROM debian:bullseye-slim

# Install necessary packages
RUN apt-get update && apt-get install -y nginx tor && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /var/lib/tor/hidden_service && \
    chown -R debian-tor:debian-tor /var/lib/tor && \
    chmod 700 /var/lib/tor/hidden_service


# Set up the debian-tor user and group
RUN usermod -a -G debian-tor www-data && \
    chown -R debian-tor:debian-tor /var/lib/tor && \
    chmod 700 /var/lib/tor/hidden_service

# Copy necessary configuration files
COPY torrc /etc/tor/torrc
COPY nginx.conf /etc/nginx/nginx.conf
COPY entrypoint.sh /entrypoint.sh

# Ensure entrypoint.sh has the correct permissions
RUN chmod +x /entrypoint.sh && sed -i 's/\r$//' /entrypoint.sh

EXPOSE 80

# Switch to the www-data user (non-root) to run services securely
USER www-data

# Start the service
ENTRYPOINT ["/entrypoint.sh"]
