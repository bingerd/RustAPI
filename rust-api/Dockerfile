FROM rust:latest AS builder

WORKDIR /app

RUN apt update &&\
    rm -rf ~/.cache &&\
    apt clean all &&\
    apt install -y cmake clang wget unzip &&\
    rm -rf /var/lib/apt/lists/*

# file
COPY . .

RUN cargo fetch

RUN cargo test 

RUN cargo build --release

FROM rust:latest

# Install minimal runtime deps for your binary and libtorch
RUN apt update &&\
    apt install -y libssl-dev libclang-dev libgcc1 libstdc++6 libgomp1 libc++1 libc++abi1 &&\
    rm -rf /var/lib/apt/lists/*


WORKDIR /app

COPY models models

# Copy the compiled Rust binary from builder stage
COPY --from=builder /app/target/release/rust-api /app/rust-api

EXPOSE 5000

# Run the binary by default
CMD ["./rust-api"]


# -----------------------------
# # Runtime stage -> for linux
# FROM debian:bullseye-slim

# WORKDIR /app

# # Minimal libraries for Rust + ONNX Runtime
# RUN apt update && \
#     apt install -y libssl1.1 libgcc1 libstdc++6 libgomp1 libc++1 libc++abi1 && \
#     rm -rf /var/lib/apt/lists/*

# COPY models models
# COPY --from=builder /app/target/release/rust-api /app/rust-api

# EXPOSE 5000

# CMD ["./rust-api"]