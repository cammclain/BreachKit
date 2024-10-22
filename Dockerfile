FROM debian:latest

RUN apt-get update && apt-get install -y python3 \
    python3-pip \
    python3-venv \
    build-essential \
    libssl-dev \
    libffi-dev \
    python3-dev \
    tor \
    torsocks

COPY requirements.txt .
RUN pip3 install -r requirements.txt

COPY . .

CMD ["systemctl", "start", "tor", "&&", "litestar", "run", "asgi.py"]