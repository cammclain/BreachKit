FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y curl

# install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# install cargo-make
RUN cargo 

# example command
CMD ["cargo", "make", "run"]