FROM debian:bullseye-slim


RUN apt-get update && apt-get install -y nginx \
    tor \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
