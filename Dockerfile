FROM messense/rust-musl-cross:x86_64-musl as chef
RUN apt-get update && apt-get install -y pkg-config libssl-dev openssl
RUN cargo install cargo-chef
WORKDIR /GHBackend

FROM chef AS planner
# Copy source code from previous stage
COPY . .
# Generate info for caching dependencies
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /GHBackend/recipe.json recipe.json
# Build & cache dependencies
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
# Copy source code from previous stage
COPY . .
# Build application
RUN cargo build --release --target x86_64-unknown-linux-musl

# Create a new stage with a minimal imagex
FROM scratch
COPY --from=builder /GHBackend/target/x86_64-unknown-linux-musl/release/GHBackend /GHBackend
ENTRYPOINT ["/GHBackend"]
EXPOSE 3000
