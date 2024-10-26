#!/bin/bash

# Enable memory overcommit
echo "vm.overcommit_memory=1" >> /etc/sysctl.conf
sysctl -p

# Start Redis with the provided config or default settings
exec redis-server /etc/redis/redis.conf
