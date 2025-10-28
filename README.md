# RustAPI
Template repo for an Axum API. It contains the following:

1. Working container for Rust and LibTorch (C++).
2. Axum API

The dummy PyTorch used to do develop this contained custom Python code inside TorchScript, hence I defaulted to a dummy endpoint.


# Install Rust if not already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install evcxr_jupyter
cargo install evcxr_jupyter

# Register kernel with Jupyter
evcxr_jupyter --install


TODO:
1. Dynamically detect optimal Execution Provider (Linux, x64, ARM)
2. Test sklearn / pytorch / Lightgbm binaries in ONNX
3. Add simplistic data processing as POC (template)
4. Write unit tests
5. Create High Concurrency testing scenario in GCP
6. Creating scaling policies based on results.