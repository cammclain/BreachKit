# this is a simple dockerfile for the seaweedfs filestore
FROM bitnami/seaweedfs:latest


WORKDIR /seaweedfs

COPY . .

EXPOSE 8888
EXPOSE 9333

CMD ["weed", "server", "-dir", "/data"]