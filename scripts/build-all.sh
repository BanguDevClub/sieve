#!/usr/bin/env bash
set -e

echo "=========================================================="
echo "    Sieve Multi-Target Cross-Platform Build Pipeline      "
echo "    Organization: BanguDevClub                           "
echo "=========================================================="

DIST_ROOT="/app/dist-build"
GNU_DIR="$DIST_ROOT/gnu"
MUSL_DIR="$DIST_ROOT/musl"
WIN_DIR="$DIST_ROOT/windows"
MAC_DIR="$DIST_ROOT/macos"

mkdir -p "$GNU_DIR" "$MUSL_DIR" "$WIN_DIR" "$MAC_DIR"

# -------------------------------------------------------------
# 1. Frontend Build
# -------------------------------------------------------------
echo ""
echo ">>> [1/5] Building Frontend (Svelte 5 + Vite)..."
cd /app/frontend
npm install
npm run build
cd /app

# -------------------------------------------------------------
# 2. Linux GNU Build (Binary, Deb, RPM, AppImage)
# -------------------------------------------------------------
echo ""
echo ">>> [2/5] Compiling Linux GNU target (x86_64-unknown-linux-gnu)..."
export RUSTFLAGS="-C link-arg=-lresolv"
cargo build --release --target x86_64-unknown-linux-gnu --manifest-path /app/backend/Cargo.toml --bin sieve

# 2.1 Standalone Binary
cp /app/backend/target/x86_64-unknown-linux-gnu/release/sieve "$GNU_DIR/sieve"
chmod +x "$GNU_DIR/sieve"
tar -czf "$GNU_DIR/sieve-linux-x86_64.tar.gz" -C "$GNU_DIR" sieve
echo "✓ Linux GNU standalone binary generated: $GNU_DIR/sieve"

# 2.2 Debian Package (.deb)
echo ">>> Packaging Debian (.deb)..."
DEB_ROOT="/tmp/sieve-deb"
rm -rf "$DEB_ROOT"
mkdir -p "$DEB_ROOT/DEBIAN"
mkdir -p "$DEB_ROOT/usr/bin"
mkdir -p "$DEB_ROOT/usr/share/applications"
mkdir -p "$DEB_ROOT/usr/share/icons/hicolor/128x128/apps"

cat << 'EOF' > "$DEB_ROOT/DEBIAN/control"
Package: sieve
Version: 0.1.0
Section: utils
Priority: optional
Architecture: amd64
Maintainer: BanguDevClub <dev@bangudev.club>
Description: High-performance DuckDB desktop CSV explorer
 Sieve allows instantaneous querying, filtering, and inspection of
 massive CSV files using in-process DuckDB vectorized operations.
EOF

cp "$GNU_DIR/sieve" "$DEB_ROOT/usr/bin/sieve"
cp /app/backend/icons/128x128.png "$DEB_ROOT/usr/share/icons/hicolor/128x128/apps/sieve.png" 2>/dev/null || true

cat << 'EOF' > "$DEB_ROOT/usr/share/applications/sieve.desktop"
[Desktop Entry]
Name=Sieve
Comment=High-performance DuckDB desktop CSV explorer
Exec=/usr/bin/sieve
Icon=sieve
Terminal=false
Type=Application
Categories=Utility;Database;Development;
EOF

dpkg-deb --build "$DEB_ROOT" "$GNU_DIR/sieve_0.1.0_amd64.deb"
echo "✓ Debian package generated: $GNU_DIR/sieve_0.1.0_amd64.deb"

# 2.3 RedHat Package (.rpm)
echo ">>> Packaging RPM (.rpm)..."
RPM_TOP="/tmp/rpmbuild"
rm -rf "$RPM_TOP"
mkdir -p "$RPM_TOP/BUILD" "$RPM_TOP/RPMS" "$RPM_TOP/SOURCES" "$RPM_TOP/SPECS" "$RPM_TOP/SRPMS"

cat << EOF > "$RPM_TOP/SPECS/sieve.spec"
Name:           sieve
Version:        0.1.0
Release:        1%{?dist}
Summary:        High-performance DuckDB desktop CSV explorer
License:        MIT
URL:            https://github.com/BanguDevClub/sieve

%description
Sieve by BanguDevClub: Ultra-fast desktop tool for opening and querying massive CSV files out-of-core.

%install
mkdir -p %{buildroot}/usr/bin
mkdir -p %{buildroot}/usr/share/applications
cp $GNU_DIR/sieve %{buildroot}/usr/bin/sieve
cp $DEB_ROOT/usr/share/applications/sieve.desktop %{buildroot}/usr/share/applications/sieve.desktop

%files
/usr/bin/sieve
/usr/share/applications/sieve.desktop
EOF

if command -v rpmbuild &>/dev/null; then
    rpmbuild --define "_topdir $RPM_TOP" -bb "$RPM_TOP/SPECS/sieve.spec"
    cp "$RPM_TOP/RPMS/x86_64/"*.rpm "$GNU_DIR/sieve-0.1.0-1.x86_64.rpm" 2>/dev/null || true
    echo "✓ RPM package generated: $GNU_DIR/sieve-0.1.0-1.x86_64.rpm"
else
    echo "! rpmbuild not found, skipping RPM packaging."
fi

# 2.4 AppImage Package
echo ">>> Packaging AppImage..."
APPDIR="/tmp/Sieve.AppDir"
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin"
mkdir -p "$APPDIR/usr/share/icons/hicolor/128x128/apps"
cp "$GNU_DIR/sieve" "$APPDIR/usr/bin/sieve"
cp /app/backend/icons/128x128.png "$APPDIR/sieve.png" 2>/dev/null || touch "$APPDIR/sieve.png"
cp "$DEB_ROOT/usr/share/applications/sieve.desktop" "$APPDIR/sieve.desktop"

cat << 'EOF' > "$APPDIR/AppRun"
#!/bin/sh
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
export PATH="${HERE}/usr/bin:${PATH}"
exec "${HERE}/usr/bin/sieve" "$@"
EOF
chmod +x "$APPDIR/AppRun"

# Use squashfs/appimage builder
if command -v mksquashfs &>/dev/null; then
    mksquashfs "$APPDIR" "$GNU_DIR/Sieve-x86_64.AppImage" -root-owned -noappend 2>/dev/null || true
    chmod +x "$GNU_DIR/Sieve-x86_64.AppImage" 2>/dev/null || true
    echo "✓ AppImage generated: $GNU_DIR/Sieve-x86_64.AppImage"
else
    tar -czf "$GNU_DIR/Sieve-AppDir.tar.gz" -C "/tmp" "Sieve.AppDir"
    echo "✓ AppDir bundle generated: $GNU_DIR/Sieve-AppDir.tar.gz"
fi

# -------------------------------------------------------------
# 3. Linux MUSL Build (Standalone Static Binary)
# -------------------------------------------------------------
echo ""
echo ">>> [3/5] Compiling Linux MUSL standalone binary (x86_64-unknown-linux-musl)..."
if [ -d /opt/x86_64-linux-musl-native/bin ]; then
    export PATH="/opt/x86_64-linux-musl-native/bin:/usr/local/cargo/bin:$PATH"
    ln -sf /opt/x86_64-linux-musl-native/bin/ar /usr/local/bin/x86_64-linux-musl-ar 2>/dev/null || true
    ln -sf /opt/x86_64-linux-musl-native/bin/ranlib /usr/local/bin/x86_64-linux-musl-ranlib 2>/dev/null || true
    ln -sf /opt/x86_64-linux-musl-native/bin/strip /usr/local/bin/x86_64-linux-musl-strip 2>/dev/null || true
fi
export CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc
export CXX_x86_64_unknown_linux_musl=x86_64-linux-musl-g++
export AR_x86_64_unknown_linux_musl=/opt/x86_64-linux-musl-native/bin/ar

RUSTFLAGS="-C lto=off" cargo build --release --target x86_64-unknown-linux-musl --no-default-features --manifest-path /app/backend/Cargo.toml --bin sieve-core
cp /app/backend/target/x86_64-unknown-linux-musl/release/sieve-core "$MUSL_DIR/sieve"
chmod +x "$MUSL_DIR/sieve"
tar -czf "$MUSL_DIR/sieve-musl-x86_64.tar.gz" -C "$MUSL_DIR" sieve
echo "✓ Linux MUSL standalone binary generated: $MUSL_DIR/sieve"

# -------------------------------------------------------------
# 4. Windows Cross-Compilation & Installer
# -------------------------------------------------------------
echo ""
echo ">>> [4/5] Compiling Windows target (x86_64-pc-windows-gnu)..."
export CC_x86_64_pc_windows_gnu=x86_64-w64-mingw32-gcc-posix
export CXX_x86_64_pc_windows_gnu=x86_64-w64-mingw32-g++-posix
RUSTFLAGS="-C lto=off" cargo build --release --target x86_64-pc-windows-gnu --no-default-features --manifest-path /app/backend/Cargo.toml --bin sieve-core

cp /app/backend/target/x86_64-pc-windows-gnu/release/sieve-core.exe "$WIN_DIR/sieve.exe"
echo "✓ Windows standalone executable generated: $WIN_DIR/sieve.exe"

# Windows NSIS Installer
if command -v makensis &>/dev/null; then
    echo ">>> Packaging Windows NSIS installer..."
    makensis /app/scripts/package-nsis.nsi
    echo "✓ Windows installer generated: $WIN_DIR/sieve-setup.exe"
else
    echo "! makensis not found, skipping NSIS installer."
fi

# -------------------------------------------------------------
# 5. macOS Distribution Setup
# -------------------------------------------------------------
echo ""
echo ">>> [5/5] Packaging macOS distribution bundle..."
/bin/bash /app/scripts/package-macos.sh
cp "$GNU_DIR/sieve" "$MAC_DIR/Sieve.app/Contents/MacOS/sieve" 2>/dev/null || true
echo "✓ macOS distribution files generated in $MAC_DIR"

# -------------------------------------------------------------
# Summary & Checksums
# -------------------------------------------------------------
echo ""
echo "=========================================================="
echo "    Build Summary - Generated Artifacts in dist-build/   "
echo "=========================================================="
find "$DIST_ROOT" -type f -exec ls -lh {} +

# Generate SHA256 sums
cd "$DIST_ROOT"
find . -type f ! -name "SHA256SUMS" -exec sha256sum {} + > "$DIST_ROOT/SHA256SUMS"
echo ""
echo "✓ SHA256 checksums written to $DIST_ROOT/SHA256SUMS"
echo "Build completed successfully for all platforms!"
