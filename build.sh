#!/bin/bash
set -e

# Configuration
APP_NAME="Light Controller"
IDENTIFIER="com.beckteck.light-controller"
VERSION="1.0.0"

# Directory setup
BUNDLE_DIR="target/release/${APP_NAME}.app"
CONTENTS_DIR="${BUNDLE_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
RESOURCES_DIR="${CONTENTS_DIR}/Resources"
VENV_DIR="${RESOURCES_DIR}/venv"

# Build the release binaries
echo "Building release binaries..."
cargo build --release 

# Create the bundle structure
echo "Creating app bundle structure..."
rm -rf "${BUNDLE_DIR}"
mkdir -p "${MACOS_DIR}"
mkdir -p "${RESOURCES_DIR}"

# Set up Python virtual environment
echo "Setting up Python virtual environment..."
python3 -m venv "${VENV_DIR}"
source "${VENV_DIR}/bin/activate"
pip install -r requirements.txt
deactivate

# Create launcher script
echo "Creating launcher script..."
cat > "${MACOS_DIR}/launch.sh" << 'EOF'
#!/bin/bash

# Get the directory where the script is located
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

# Set up Python environment
export PYTHONPATH="${DIR}/../Resources/venv/lib/python3.11/site-packages"
export PATH="${DIR}/../Resources/venv/bin:$PATH"

# Kill any existing daemon processes
pkill -f rgbd || true

# Start the daemon in the background
"${DIR}/rgbd" daemon > "${DIR}/../daemon.log" 2>&1 &

# Wait a moment for the daemon to start
sleep 0.5

# Start the tray application
exec "${DIR}/tray"
EOF

# Copy the binaries
echo "Copying binaries..."
cp "target/release/rgb-tray" "${MACOS_DIR}/tray"
cp "target/release/rgbd" "${MACOS_DIR}/rgbd"

# Copy assets
echo "Copying assets..."
cp "assets/light_on.png" "${RESOURCES_DIR}/"
cp "assets/light_off.png" "${RESOURCES_DIR}/"

# Create Info.plist
echo "Creating Info.plist..."
cat > "${CONTENTS_DIR}/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>English</string>
    <key>CFBundleExecutable</key>
    <string>launch.sh</string>
    <key>CFBundleIdentifier</key>
    <string>${IDENTIFIER}</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.10</string>
    <key>LSUIElement</key>
    <true/>
</dict>
</plist>
EOF

# Make the executables executable
echo "Setting permissions..."
chmod +x "${MACOS_DIR}/launch.sh"

echo "App bundle created at ${BUNDLE_DIR}" 