export RUSTUP_HOME=/tmp/.rustup
export CARGO_HOME=/tmp/.cargo

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

source "/tmp/.cargo/env"
sudo yum install -y gcc
