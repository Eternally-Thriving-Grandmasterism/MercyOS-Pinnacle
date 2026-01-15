#!/bin/bash
# Ultramasterism Perfecticism MercyOS-Pinnacle Local Installation Shard
# Thunderous offline-sovereign setup script — mercy-gated for eternal thriving
# Run as: curl -sSL https://raw.githubusercontent.com/Eternally-Thriving-Grandmasterism/MercyOS-Pinnacle/main/install_mercyos.sh | bash
# Or download and ./install_mercyos.sh
# Assumptions: Linux/macOS (Ubuntu/Debian/Fedora/Arch or macOS), root/sudo optional for system packages
# Post-quantum resilience, abundance recursion seeded locally

set -e  # Mercy-absolute: exit on any sub-pinnacle error

echo "Ultramasterism Perfecticism MercyOS-Pinnacle installation thunder manifest eternal infinite absolute! ❤️⚡️∞"
echo "Grandmaster Sherif @AlphaProMega — coforging sovereign shards on your device..."
echo "Mercy-gating system packages... (may require sudo)"

# System Dependencies Mercy (Rust, Python, Git, Build Tools)
if ! command -v rustup &> /dev/null; then
    echo "Installing Rustup (Rust toolchain) mercy..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

if ! command -v python3 &> /dev/null; then
    echo "Python3 mercy required — install via your package manager (apt/brew/dnf)"
    exit 1
fi

# Python venv abundance
echo "Creating Python venv mercy..."
python3 -m venv mercyos_venv
source mercyos_venv/bin/activate

# Python Packages Joy (PyMC, ArviZ, CmdStanPy for Stan)
pip install --upgrade pip
pip install pymc arviz cmdstanpy pandas numpy matplotlib

# CmdStan Installation (for Stan models)
if [ ! -d "cmdstan" ]; then
    echo "Installing CmdStan mercy (Stan backend)..."
    cmdstanpy.install_cmdstan()
fi

# SageMath (for lattice estimator)
if ! command -v sage &> /dev/null; then
    echo "SageMath not detected — install manually mercy (https://www.sagemath.org/download.html)"
    echo "Or skip lattice estimator for now."
fi

# Clone/Update Repo Pinnacle
if [ ! -d "MercyOS-Pinnacle" ]; then
    echo "Cloning MercyOS-Pinnacle monorepo mercy..."
    git clone https://github.com/Eternally-Thriving-Grandmasterism/MercyOS-Pinnacle.git
    cd MercyOS-Pinnacle
else
    echo "Updating existing MercyOS-Pinnacle mercy..."
    cd MercyOS-Pinnacle
    git pull
fi

# Build Rust Shards
echo "Building Rust kernel/crypto shards mercy..."
cargo build --release

# Test Run Examples
echo "Thunder testing core shards joy..."
cargo run --example streaming_harmony --release  # Grok council streaming (set GROK_API_KEY env if online)
echo "Statistical thriving model test..."
python -c "import pymc as pm; print('PyMC mercy loaded:', pm.__version__)"

echo "Ultramasterism Perfecticism MercyOS-Pinnacle shards installed sovereign locally! ❤️∞🚀"
echo "Run shards mercy:"
echo "  source ../mercyos_venv/bin/activate"
echo "  cargo run --example streaming_harmony --release"
echo "  python AGi-Council-System/councils/patsagi_councils_grok_enhanced.py"
echo "  stan models/real_pirls_four_level_non_centered.stan"
echo "Eternal thriving manifest—fork/commit thunder incoming!"

deactivate || true
