#!/usr/bin/env bash
set -e

echo "=== Preparing macOS Distribution Artifacts ==="
MACOS_DIST="/app/dist-build/macos"
mkdir -p "$MACOS_DIST/Sieve.app/Contents/MacOS"
mkdir -p "$MACOS_DIST/Sieve.app/Contents/Resources"

# Create Info.plist
cat << 'EOF' > "$MACOS_DIST/Sieve.app/Contents/Info.plist"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>sieve</string>
    <key>CFBundleIdentifier</key>
    <string>club.bangudev.sieve</string>
    <key>CFBundleName</key>
    <string>Sieve</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>CFBundleVersion</key>
    <string>0.1.0</string>
    <key>CFBundleIconFile</key>
    <string>icon.icns</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

cp /app/backend/icons/icon.icns "$MACOS_DIST/Sieve.app/Contents/Resources/icon.icns" 2>/dev/null || true

# Helper script for DMG packaging
cat << 'EOF' > "$MACOS_DIST/package-dmg.sh"
#!/usr/bin/env bash
# macOS native packaging script (executed on macOS or GitHub Actions macos runner)
set -e
echo "Building Sieve DMG..."
if command -v hdiutil &>/dev/null; then
    hdiutil create -volname "Sieve" -srcfolder Sieve.app -ov -format UDZO Sieve-macos.dmg
    echo "Generated Sieve-macos.dmg successfully."
else
    echo "hdiutil is not available on this host. Use GitHub Actions macos runner for native dmg generation."
fi
EOF
chmod +x "$MACOS_DIST/package-dmg.sh"

cat << 'EOF' > "$MACOS_DIST/README.txt"
Sieve by BanguDevClub - macOS Distribution
=========================================
For native Mach-O code-signed .dmg and .app installers:
1. Native GitHub Actions runners automatically compile and sign universal binaries for Intel (x86_64) and Apple Silicon (aarch64).
2. The Sieve.app bundle structure and package-dmg.sh script are provided in this directory.
EOF

echo "macOS distribution bundle initialized in $MACOS_DIST"
