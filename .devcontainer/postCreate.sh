# This script is used to download rust toolchain needed for dioxus.
# Based on the Dioxus website: https://dioxuslabs.com/learn/0.7/getting_started/
sudo apt-get update
rustup toolchain install stable
rustup target add wasm32-unknown-unknown
# Install Dioxus CLI
curl -sSL https://dioxus.dev/install.sh | bash