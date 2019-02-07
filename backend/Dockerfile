FROM liuchong/rustup

ENV ROCKET_ADDRESS=0.0.0.0

ENV ROCKET_PORT=8000

WORKDIR /app

COPY . .

RUN rustup default nightly

RUN cargo install cargo-watch

CMD cargo watch -x run
