FROM debian:bullseye-slim


RUN apt-get update && apt-get install -y nginx \
    tor \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

COPY torrc /etc/tor/torrc
RUN service tor restart

COPY nginx.conf /etc/nginx/nginx.conf

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh && \
    sed -i 's/\r$//' /entrypoint.sh && \
    chmod 755 /entrypoint.sh


EXPOSE 80

ENTRYPOINT [ "/entrypoint.sh" ]
