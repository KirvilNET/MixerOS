#!/bin/sh
set -e

# ─────────────────────────────────────────────
# OpenCL WSL2 Workaround for yucky windows users
# Work around for WSL2 and OpenCL from https://github.com/microsoft/WSL/issues/6951#issuecomment-2745803886
# converted into a portable bash script with claude and verifyed by me.
# ─────────────────────────────────────────────

CUDA_VERSION="12.8.1"
CUDA_DRIVER="570.124.06"
CUDA_INSTALLER="cuda_${CUDA_VERSION}_${CUDA_DRIVER}_linux.run"
CUDA_URL="https://developer.download.nvidia.com/compute/cuda/${CUDA_VERSION}/local_installers/${CUDA_INSTALLER}"
LLVM_VERSION="14"
POCL_VERSION="v6.0"
DOWNLOADS_DIR="$HOME/Downloads"

# Colors!!!!
if [ -t 1 ]; then
    RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'; RESET='\033[0m'
else
    RED=''; GREEN=''; YELLOW=''; RESET=''
fi

info()    { printf "${GREEN}[INFO]${RESET}  %s\n" "$*"; }
warn()    { printf "${YELLOW}[WARN]${RESET}  %s\n" "$*"; }
error()   { printf "${RED}[ERROR]${RESET} %s\n" "$*" >&2; exit 1; }

if [ "$(id -u)" -ne 0 ]; then
    error "Please run as root or with sudo: sudo sh $0"
fi

# ─── Step 1: Fix any broken apt state ─────────────────────────────────────────
info "Cleaning and repairing apt..."
apt clean
dpkg --configure -a
apt update
apt --fix-broken install -y

# ─── Step 2: Download CUDA installer ──────────────────────────────────────────
info "Preparing downloads directory..."
mkdir -p "$DOWNLOADS_DIR"
cd "$DOWNLOADS_DIR"

if [ -f "$CUDA_INSTALLER" ]; then
    warn "CUDA installer already exists, skipping download."
else
    info "Downloading CUDA ${CUDA_VERSION}..."
    wget -c "$CUDA_URL" || error "Failed to download CUDA installer"
fi

# ─── Step 3: Install CUDA toolkit ─────────────────────────────────────────────
info "Installing build dependencies for CUDA..."
apt install -y gcc

info "Running CUDA installer (silent, toolkit only)..."
sh "./${CUDA_INSTALLER}" --silent --toolkit --no-opengl-libs \
    || error "CUDA installation failed"

# ─── Step 4: Install OpenCL and build dependencies ────────────────────────────
info "Installing OpenCL and build dependencies..."
apt install -y \
    python3-dev \
    libpython3-dev \
    build-essential \
    ocl-icd-libopencl1 \
    cmake \
    git \
    pkg-config \
    make \
    ninja-build \
    ocl-icd-dev \
    ocl-icd-opencl-dev \
    libhwloc-dev \
    zlib1g \
    zlib1g-dev \
    clinfo \
    dialog \
    apt-utils \
    libxml2-dev \
    opencl-headers

# ─── Step 5: Install LLVM ─────────────────────────────────────────────────────
info "Installing LLVM ${LLVM_VERSION}..."
apt install -y \
    "libclang-${LLVM_VERSION}-dev" \
    "clang-${LLVM_VERSION}" \
    "llvm-${LLVM_VERSION}" \
    "libclang-cpp${LLVM_VERSION}-dev" \
    "libclang-cpp${LLVM_VERSION}" \
    "llvm-${LLVM_VERSION}-dev"

# ─── Step 6: Clone and build PoCL ─────────────────────────────────────────────
info "Cloning PoCL ${POCL_VERSION}..."
cd "$DOWNLOADS_DIR"

if [ -d "pocl" ]; then
    warn "PoCL directory already exists, skipping clone."
else
    git clone https://github.com/pocl/pocl -b "$POCL_VERSION" \
        || error "Failed to clone PoCL"
fi

info "Configuring PoCL with CMake..."
mkdir -p pocl/build
cd pocl/build

cmake \
    -DCMAKE_C_FLAGS="-L/usr/lib/wsl/lib" \
    -DCMAKE_CXX_FLAGS="-L/usr/lib/wsl/lib" \
    -DWITH_LLVM_CONFIG="/usr/bin/llvm-config-${LLVM_VERSION}" \
    -DENABLE_HOST_CPU_DEVICES=OFF \
    -DENABLE_CUDA=ON \
    .. || error "CMake configuration failed"

info "Building PoCL using $(nproc) cores..."
make -j"$(nproc)" || error "PoCL build failed"

info "Installing PoCL..."
make install || error "PoCL install failed"

# ─── Step 7: Register PoCL ICD ────────────────────────────────────────────────
info "Registering PoCL ICD..."
mkdir -p /etc/OpenCL/vendors
cp /usr/local/etc/OpenCL/vendors/pocl.icd /etc/OpenCL/vendors/pocl.icd \
    || error "Failed to copy pocl.icd"

# ─── Done ─────────────────────────────────────────────────────────────────────
info "Done! Verifying OpenCL platforms..."
clinfo | head -20 || warn "clinfo returned no platforms — a reboot may be required"

info "Setup complete."