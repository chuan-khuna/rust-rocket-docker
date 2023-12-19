FROM rust:1.72.1-buster

WORKDIR /app
COPY . .

RUN rustup default stable
RUN cargo install cargo-watch
RUN cargo build


CMD ["cargo", "watch", "-c", "-w", "src", "-x", "run"]
