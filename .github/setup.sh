#!/usr/bin/env bash
set -euo pipefail

SUDO=""
if command -v sudo >/dev/null 2>&1; then
  SUDO="sudo"
fi

# fetch update software list
$SUDO apt-get update

# install java and base tools
$SUDO apt-get install -y default-jre-headless default-jdk-headless python3-pip bc curl wget pkg-config libfontconfig1-dev libfreetype6-dev

# install julia (host-arch aware for local container debugging)
ARCH="$(uname -m)"
if [[ "$ARCH" == "x86_64" || "$ARCH" == "amd64" ]]; then
  JULIA_TARBALL="julia-1.10.10-linux-x86_64.tar.gz"
  JULIA_URL="https://julialang-s3.julialang.org/bin/linux/x64/1.10/$JULIA_TARBALL"
elif [[ "$ARCH" == "aarch64" || "$ARCH" == "arm64" ]]; then
  JULIA_TARBALL="julia-1.10.10-linux-aarch64.tar.gz"
  JULIA_URL="https://julialang-s3.julialang.org/bin/linux/aarch64/1.10/$JULIA_TARBALL"
else
  echo "Unsupported architecture for Julia install: $ARCH" >&2
  exit 1
fi

$SUDO wget -q "$JULIA_URL"
$SUDO tar zxf "$JULIA_TARBALL"
JULIA_BIN="$(pwd)/julia-1.10.10/bin"
export PATH="$PATH:$JULIA_BIN"
if [[ -n "${GITHUB_PATH:-}" ]]; then
  echo "$JULIA_BIN" >> "$GITHUB_PATH"
fi

# install agents
julia --project=@. -e 'using Pkg; Pkg.instantiate()'
# TODO: refresh Project/Manifest to restore compatibility with Julia 1.11+ and then bump CI Julia.

# install python deps in isolated venv
python3 -m venv .venv
if [[ -n "${GITHUB_PATH:-}" ]]; then
  echo "$(pwd)/.venv/bin" >> "$GITHUB_PATH"
fi
source .venv/bin/activate
python -m pip install --upgrade pip
python -m pip install mesa==3.2.0
python -m pip install "git+https://github.com/mesa/mesa-frames.git"
python -m pip install beartype

# install netlogo
$SUDO wget https://downloads.netlogo.org/6.4.0/NetLogo-6.4.0-64.tgz
$SUDO tar -xzf NetLogo-6.4.0-64.tgz
$SUDO mv "NetLogo-6.4.0-64" netlogo

# install rust toolchain
curl https://sh.rustup.rs -sSf | sh -s -- -y
source "$HOME/.cargo/env"
if [[ -n "${GITHUB_PATH:-}" ]]; then
  echo "$HOME/.cargo/bin" >> "$GITHUB_PATH"
fi
