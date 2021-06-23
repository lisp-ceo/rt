FROM rust:nightly-buster-slim as builder

WORKDIR /usr/src/rt
COPY . .
RUN make install
RUN cargo test
RUN cargo install --path

FROM debian:buster-slim

RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rt /usr/local/bin/rt

CMD ["rt"]
