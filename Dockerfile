FROM messense/rust-musl-cross:x86_64-musl as chef
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get -y install pkg-config libssl-dev librust-openssl-sys-dev
RUN cargo install cargo-chef
WORKDIR /GHBackend

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /GHBackend/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /GHBackend/target/x86_64-unknown-linux-musl/release/GHBackend /GHBackend
ENTRYPOINT ["/GHBackend"]
EXPOSE 3000