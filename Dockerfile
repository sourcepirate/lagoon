FROM rustlang/rust:nightly-buster-slim AS build

RUN apt-get update
RUN apt-get install -y build-essential clang

WORKDIR /app
COPY . /app
RUN cargo clean && cargo build --release --target x86_64-unknown-linux-gnu
RUN strip ./target/x86_64-unknown-linux-gnu/release/lagoon

FROM debian:buster-slim

WORKDIR /usr/src/lagoon

COPY --from=build /app/target/x86_64-unknown-linux-gnu/release/lagoon /usr/local/bin/lagoon

CMD [ "lagoon" ]

EXPOSE 4000