#!/bin/bash

export LLVM_VERSION=21

if [ "$(id -u)" -ne 0 ]; then
  echo "Please run as root"
  exit 1
fi
set -e

nvidia=false
amd=false
intel=false
pocl=false

distro=$(grep '^VERSION_ID=' /etc/os-release | cut -d= -f2 | tr -d '."')
sys_arch=$(arch)

while getopts "g:" opt; do
  case $opt in
    g) 
      if [[ $OPTARG == *"amd"* ]]; then 
        amd=true
      elif [[ $OPTARG == *"nvidia"* ]]; then 
        nvidia=true
      elif [[ $OPTARG == *"intel"* ]]; then 
        intel=true
      elif [[ $OPTARG == *"pocl"* ]]; then
        pocl=true
      else 
        echo "Invalid option: -$OPTARG" >&2
        exit 1 
      fi
      ;;
    \?)
      echo "Invalid option: -$OPTARG" >&2
      exit 1 
      ;;
  esac
done

if [[ $sys_arch == *"aarch64"* ]]; then
  sys_arch="arm64"
fi

apt update

apt install -y gcc make pkg-config 

wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu${distro}/${sys_arch}/cuda-keyring_1.1-1_all.deb
dpkg -i cuda-keyring_1.1-1_all.deb

apt update

if [ nvidia == true ]; then 
  apt install -y install cuda-toolkit-13-2
fi

if [ intel == true ]; then 
  apt install -y intel-opencl-icd
fi

if [ amd == true ]; then 
  wget https://repo.radeon.com/amdgpu-install/latest/ubuntu/noble/amdgpu-install_7.2.1.70201-1_all.deb
  apt install ./amdgpu-install-7.2.1.70201-1_all.deb
  apt update

  amdgpu-install -y --opencl=rocr
fi

if [ pocl == true ]; then
  apt install -y python3-dev libpython3-dev build-essential ocl-icd-libopencl1 \
    cmake git pkg-config libclang-${LLVM_VERSION}-dev clang-${LLVM_VERSION} \
    llvm-${LLVM_VERSION} make ninja-build ocl-icd-libopencl1 ocl-icd-dev \
    ocl-icd-opencl-dev libhwloc-dev zlib1g zlib1g-dev clinfo dialog apt-utils \
    libxml2-dev libclang-cpp${LLVM_VERSION}-dev libclang-cpp${LLVM_VERSION} \
    llvm-${LLVM_VERSION}-dev
  
  git clone https://github.com/pocl/pocl.git
  cd pocl
  mkdir build && cd build
  cmake ..
  make
  make install
fi

apt install -y \
jackd2 \
qjackctl
apt install libjack-jackd2-dev

apt install capnproto

usermod -a -G video $USER
usermod -a -G render $USER