FROM clux/muslrust:stable AS chef

USER root
RUN cargo install cargo-chef
WORKDIR /scrounch_backend

# --
FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# --
FROM chef AS builder

COPY --from=planner /scrounch_backend/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl --no-default-features --features rustls-tls,cache,vendored

# --
FROM scratch

ENV PATH=/bin
COPY --from=builder /scrounch_backend/target/x86_64-unknown-linux-musl/release/scrounch_backend /scrounch_backend
ENTRYPOINT ["/scrounch_backend"]
EXPOSE 3000
