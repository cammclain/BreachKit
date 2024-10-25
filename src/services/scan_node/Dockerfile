FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y python3 python3-pip

COPY . /app

RUN pip3 install -r requirements.txt