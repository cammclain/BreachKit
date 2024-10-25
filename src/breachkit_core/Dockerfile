# Use an official Python image from the Docker Hub
FROM debian:latest

RUN apt-get update && apt-get install -y \
    python3 python3-pip vim python3-venv build-essential libssl-dev libffi-dev python3-dev tor torsocks curl && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /breachkit_core

COPY requirements.txt .

# make the virtual environment
RUN python3 -m venv venv

# set the virtual environment as the active environment
ENV PATH="/breachkit_core/venv/bin:$PATH"

# install the dependencies
RUN pip3 install -r requirements.txt

COPY . .

CMD ["litestar", "run", "asgi.py", "--host", "127.0.0.1", "--port", "8003"]