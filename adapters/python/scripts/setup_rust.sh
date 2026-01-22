#!/bin/bash

# Automated Rust toolchain setup for Rengine IPL-to-Godot converter
# This script detects and installs Rust if not already present

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Check if Rust is already installed
check_rust() {
    if command -v rustc >/dev/null 2>&1 && command -v cargo >/dev/null 2>&1; then
        local rust_version=$(rustc --version | cut -d' ' -f2)
        local cargo_version=$(cargo --version | cut -d' ' -f2)
        log_success "Rust is already installed!"
        echo "   Rust version: $rust_version"
        echo "   Cargo version: $cargo_version"
        return 0
    else
        return 1
    fi
}

# Detect the operating system
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        echo "linux"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "macos"
    elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
        echo "windows"
    else
        echo "unknown"
    fi
}

# Install Rust using rustup (cross-platform)
install_rust_rustup() {
    log_info "Installing Rust using rustup..."

    # Download and run rustup installer
    if command -v curl >/dev/null 2>&1; then
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile default
    elif command -v wget >/dev/null 2>&1; then
        wget -qO- https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --default-profile default
    else
        log_error "Neither curl nor wget found. Please install curl or wget and try again."
        return 1
    fi

    # Source the cargo environment
    if [[ -f "$HOME/.cargo/env" ]]; then
        source "$HOME/.cargo/env"
    fi

    # Verify installation
    if command -v rustc >/dev/null 2>&1 && command -v cargo >/dev/null 2>&1; then
        log_success "Rust installed successfully!"
        rustc --version
        cargo --version
        return 0
    else
        log_error "Rust installation failed"
        return 1
    fi
}

# Install Rust using system package manager (fallback)
install_rust_system() {
    local os=$(detect_os)
    log_warning "Attempting system package manager installation for $os..."

    case $os in
        linux)
            # Detect Linux distribution
            if command -v apt-get >/dev/null 2>&1; then
                log_info "Detected Debian/Ubuntu. Installing via apt..."
                sudo apt-get update && sudo apt-get install -y curl build-essential
                install_rust_rustup
            elif command -v dnf >/dev/null 2>&1; then
                log_info "Detected Fedora/RHEL. Installing via dnf..."
                sudo dnf install -y curl gcc gcc-c++
                install_rust_rustup
            elif command -v pacman >/dev/null 2>&1; then
                log_info "Detected Arch Linux. Installing via pacman..."
                sudo pacman -S --noconfirm curl gcc
                install_rust_rustup
            else
                log_error "Unsupported Linux distribution. Please install Rust manually: https://rustup.rs/"
                return 1
            fi
            ;;
        macos)
            if command -v brew >/dev/null 2>&1; then
                log_info "Detected Homebrew. Installing Rust..."
                brew install rust
                if command -v rustc >/dev/null 2>&1; then
                    log_success "Rust installed via Homebrew!"
                    return 0
                fi
            else
                log_info "Installing via rustup (recommended for macOS)..."
                install_rust_rustup
            fi
            ;;
        windows)
            log_info "For Windows, rustup is the recommended installation method..."
            install_rust_rustup
            ;;
        *)
            log_error "Unsupported operating system. Please install Rust manually: https://rustup.rs/"
            return 1
            ;;
    esac
}

# Main installation function
install_rust() {
    log_info "Installing Rust toolchain..."

    # Try rustup first (works on all platforms)
    if install_rust_rustup; then
        return 0
    fi

    # Fallback to system package manager
    log_warning "Rustup installation failed, trying system package manager..."
    if install_rust_system; then
        return 0
    fi

    log_error "All installation methods failed. Please install Rust manually:"
    echo "   Visit: https://rustup.rs/"
    echo "   Or run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    return 1
}

# Update PATH for the current session
update_path() {
    # Add Cargo bin directory to PATH if not already there
    if [[ ":$PATH:" != *":$HOME/.cargo/bin:"* ]]; then
        export PATH="$HOME/.cargo/bin:$PATH"
        log_info "Added $HOME/.cargo/bin to PATH for this session"
    fi
}

# Verify the installation works
verify_installation() {
    log_info "Verifying Rust installation..."

    if ! command -v rustc >/dev/null 2>&1; then
        log_error "rustc command not found"
        return 1
    fi

    if ! command -v cargo >/dev/null 2>&1; then
        log_error "cargo command not found"
        return 1
    fi

    # Try to compile a simple program
    echo 'fn main() { println!("Hello, Rust!"); }' > /tmp/rust_test.rs
    if rustc /tmp/rust_test.rs -o /tmp/rust_test && /tmp/rust_test | grep -q "Hello, Rust!"; then
        log_success "Rust compilation test passed!"
        rm -f /tmp/rust_test.rs /tmp/rust_test
        return 0
    else
        log_error "Rust compilation test failed"
        return 1
    fi
}

# Main script
main() {
    echo "🔧 Rengine Rust Toolchain Setup"
    echo "================================="
    echo ""

    # Check if already installed
    if check_rust; then
        echo ""
        log_success "Rust is ready for IPL-to-Godot conversion!"
        echo "   You can now run: bun ipl-to-godot:convert:gta-sa"
        exit 0
    fi

    echo ""
    log_info "Rust not found. Starting automated installation..."
    echo ""

    # Detect OS
    local os=$(detect_os)
    log_info "Detected operating system: $os"

    # Install Rust
    if install_rust; then
        update_path

        if verify_installation; then
            echo ""
            log_success "Rust toolchain setup complete!"
            echo "   You can now build Godot extensions for IPL-to-Godot conversion"
            echo ""
            echo "   Next steps:"
            echo "   1. Run: bun ipl-to-godot:convert:gta-sa"
            echo "   2. In the generated project directory, run: ./build_extension.sh"
            echo "   3. Open the project in Godot and enjoy!"
            exit 0
        fi
    fi

    echo ""
    log_error "Rust installation failed"
    echo ""
    echo "Please try installing manually:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo ""
    echo "Or visit: https://rustup.rs/"
    exit 1
}

# Run main function
main "$@"