# Sieve Custom Multi-Target Build Image
# Base: official rust:latest image (Debian-based)
FROM rust:latest

ENV DEBIAN_FRONTEND=noninteractive

# 1. Install base build tools, cross compilers, and GUI packaging tools
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    wget \
    build-essential \
    cmake \
    clang \
    pkg-config \
    libssl-dev \
    libwebkit2gtk-4.1-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    file \
    patchelf \
    squashfs-tools \
    dpkg-dev \
    rpm \
    nsis \
    musl-tools \
    musl-dev \
    gcc-mingw-w64-x86-64 \
    g++-mingw-w64-x86-64 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 2. Install Node.js (v22 LTS)
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g npm@latest && \
    rm -rf /var/lib/apt/lists/*

# 3. Add Rust cross-compilation targets
RUN rustup target add x86_64-unknown-linux-gnu && \
    rustup target add x86_64-unknown-linux-musl && \
    rustup target add x86_64-pc-windows-gnu

# 4. Install true musl C++ toolchain for compiling C++ libraries (DuckDB) with musl libc
RUN curl -fsSL https://musl.cc/x86_64-linux-musl-native.tgz | tar -xz -C /opt && \
    ln -sf /opt/x86_64-linux-musl-native/bin/* /usr/local/bin/ && \
    update-alternatives --set x86_64-w64-mingw32-gcc /usr/bin/x86_64-w64-mingw32-gcc-posix || true && \
    update-alternatives --set x86_64-w64-mingw32-g++ /usr/bin/x86_64-w64-mingw32-g++-posix || true

# 4. Working directory
WORKDIR /app

# 5. Default execution command
CMD ["/bin/bash", "/app/scripts/build-all.sh"]
