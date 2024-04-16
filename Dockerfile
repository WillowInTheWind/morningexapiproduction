FROM rust:1.76 AS builder
ENV DATABASE_URL ${DATABASE_URL}
WORKDIR /morningexapiproduction
#COPY Cargo.lock Cargo.toml ./
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim AS runner

RUN apt-get update && apt-get install -y ca-certificates libssl3
COPY --from=builder morningexapiproduction/target/release/morningexapiproduction /morningexapiproduction

EXPOSE 8080

CMD ["/target/release/morningexapiproduction"]