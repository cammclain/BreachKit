# Dockerfile for Redis Service
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y redis-server && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Copy the entrypoint script
COPY entrypoint.sh /redis/entrypoint.sh
RUN chmod +x /redis/entrypoint.sh

EXPOSE 6379

ENTRYPOINT ["/redis/entrypoint.sh"]
