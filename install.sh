#!/bin/bash

# WRITTEN BY AI
# NOT TESTED

# --- Configuration ---
# Standard location for user binaries on Linux/Mac. 
# This is automatically in your PATH, so no need to edit .bashrc/.zshrc.
INSTALL_DIR="/usr/local/bin"

# --- Relaunch as sudo if not already ---
if [ "$EUID" -ne 0 ]; then
    echo "Requesting administrator privileges..."
    sudo "$0" "$@"
    exit $?
fi

# Get the original user (so we don't run cargo as root)
REAL_USER=${SUDO_USER:-$(whoami)}
REAL_HOME=$(eval echo "~$REAL_USER")

# --- Script begins ---
# If first argument passed, use it as the working directory
if [ -n "$1" ]; then
    cd "$1" || exit 1
fi

CURRENT_DIR=$(pwd)

echo ""
echo "==== Installation Target: $INSTALL_DIR ===="

# Create directory if it somehow doesn't exist (unlikely for /usr/local/bin)
if [ ! -d "$INSTALL_DIR" ]; then
    mkdir -p "$INSTALL_DIR"
    echo "Created $INSTALL_DIR"
fi

echo ""
echo "==== Building all cargo projects ===="

# Loop through directories
for d in */ ; do
    # Remove trailing slash for cleaner text
    dirname=${d%/}
    
    if [ -f "$dirname/Cargo.toml" ]; then
        echo ""
        echo "--- Building $dirname ---"
        
        cd "$dirname" || continue

        # Run cargo as the original user, not root
        # We also ensure the user's PATH is passed so we find 'cargo'
        if ! sudo -u "$REAL_USER" bash -c "export PATH=$PATH:/$REAL_HOME/.cargo/bin; cargo build --release" > /tmp/cargo_build.log 2>&1; then
            echo "[!] Build failed in $dirname, skipping..."
            echo "-------------------------------------------------"
            cat /tmp/cargo_build.log
            echo "-------------------------------------------------"
            cd "$CURRENT_DIR" || exit
            continue
        fi

        # Cleanup log
        rm -f /tmp/cargo_build.log

        # Find executable files in target/release
        # We exclude files ending in .d, .rlib, or directory folders
        echo "Installing binaries to '$INSTALL_DIR'..."
        
        # This find command looks for executable files that are NOT libraries
        find target/release -maxdepth 1 -type f -perm +111 \
            ! -name "*.d" \
            ! -name "*.rlib" \
            ! -name "*.so" \
            ! -name "*.dylib" \
            -exec mv -f {} "$INSTALL_DIR/" \;

        cd "$CURRENT_DIR" || exit
    else
        echo "Skipping $dirname (no Cargo.toml found)"
    fi
done

echo ""
echo "Fin!"
echo "Your tools are now available globally."
