FROM rust:latest AS builder

WORKDIR /app

RUN apt update &&\
    rm -rf ~/.cache &&\
    apt clean all &&\
    apt install -y cmake clang wget unzip &&\
    rm -rf /var/lib/apt/lists/*

# install libtorch=2.7.0
# https://pytorch.org/get-started/locally/
RUN wget https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-2.7.0%2Bcpu.zip -O libtorch.zip &&\
    unzip -o libtorch.zip &&\
    rm libtorch.zip

ENV LIBTORCH=/app/libtorch
ENV LD_LIBRARY_PATH=/app/libtorch/lib:$LD_LIBRARY_PATH


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

COPY models .

# Copy the compiled Rust binary from builder stage
COPY --from=builder /app/target/release/rust-api /app/rust-api

# Copy the libtorch folder from builder stage
COPY --from=builder /app/libtorch /app/libtorch

ENV LIBTORCH=/app/libtorch
ENV LD_LIBRARY_PATH=/app/libtorch/lib:$LD_LIBRARY_PATH

EXPOSE 5000

# Run the binary by default
CMD ["./rust-api"]