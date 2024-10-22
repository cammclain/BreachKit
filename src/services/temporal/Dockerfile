FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y curl

RUN curl -fsSL https://temporal.io/cli/install | bash

CMD ["temporal", "start", "--env", "development"]