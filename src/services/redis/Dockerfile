FROM debian:bullseye-slim

# Install Redis and dependencies
RUN apt-get update && apt-get install -y \
    redis-server && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy custom Redis config if necessary (optional)
COPY redis.conf /etc/redis/redis.conf

# Set the working directory
WORKDIR /redis

# Copy the entrypoint script
COPY entrypoint.sh /redis/entrypoint.sh
RUN chmod +x /redis/entrypoint.sh

EXPOSE 6379

ENTRYPOINT ["/redis/entrypoint.sh"]
