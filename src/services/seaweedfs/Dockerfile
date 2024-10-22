# this is a simple dockerfile for the seaweedfs filestore
FROM bitnami/seaweedfs:latest

COPY . .

CMD ["weed", "server", "-dir", "/data"]