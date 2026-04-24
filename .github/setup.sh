#!/usr/bin/env bash
set -euo pipefail

# fetch update software list
sudo apt-get update

# give permissions
sudo chmod a+rwx ./
sudo chmod -R 777 ./

# install java and base tools
sudo apt-get install -y default-jre-headless default-jdk-headless python3-pip bc curl

# install julia
sudo wget -q https://julialang-s3.julialang.org/bin/linux/x64/1.11/julia-1.11.5-linux-x86_64.tar.gz
sudo tar zxf julia-1.11.5-linux-x86_64.tar.gz
JULIA_BIN="$(pwd)/julia-1.11.5/bin"
export PATH="$PATH:$JULIA_BIN"
echo "$JULIA_BIN" >> "$GITHUB_PATH"

# install agents
julia --project=@. -e 'using Pkg; Pkg.instantiate()'

# install python deps in isolated venv
python3 -m venv .venv
echo "$(pwd)/.venv/bin" >> "$GITHUB_PATH"
source .venv/bin/activate
python -m pip install --upgrade pip
python -m pip install mesa==3.2.0
python -m pip install "git+https://github.com/mesa/mesa-frames.git"
python -m pip install beartype

# install netlogo
sudo wget https://downloads.netlogo.org/6.4.0/NetLogo-6.4.0-64.tgz
sudo tar -xzf NetLogo-6.4.0-64.tgz
sudo mv "NetLogo-6.4.0-64" netlogo

# install rust toolchain
curl https://sh.rustup.rs -sSf | sh -s -- -y
source "$HOME/.cargo/env"
echo "$HOME/.cargo/bin" >> "$GITHUB_PATH"
