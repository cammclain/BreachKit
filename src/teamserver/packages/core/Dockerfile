# This Dockerfile creates a Docker image for the core service.
# This image will run the core loco service.

FROM debian-slim:bookworm


WORKDIR /app

RUN apt-get update && apt-get install -y curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
RUN source $HOME/.cargo/env
COPY . .

RUN cargo install loco
RUN cargo install sea-orm-cli
CMD ["cargo", "run", "--release"]