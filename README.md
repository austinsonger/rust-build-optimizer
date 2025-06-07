# 🚀 Atlas

[![Crates.io](https://img.shields.io/crates/v/atlas.svg)](https://crates.io/crates/atlas)
[![Documentation](https://docs.rs/atlas/badge.svg)](https://docs.rs/atlas)


A comprehensive Rust build optimization tool that dramatically improves build times and development workflow through intelligent configuration, tool management, and performance monitoring.

## ✨ Features

- **🚀 Dramatic Build Speed Improvements**: 27-50% faster incremental builds, 2-3x faster testing
- **🔧 Intelligent Configuration**: Auto-generates optimized Cargo configurations for your system
- **🛠️ Tool Management**: Automatically installs and manages optimization tools (sccache, cargo-nextest, fast linkers)
- **📊 Performance Monitoring**: Real-time build statistics and performance analysis
- **⚡ Development Workflow**: Watch mode, quick checks, and continuous development tools
- **🎯 Cross-Platform**: Supports macOS, Linux, and Windows with platform-specific optimizations

## 📦 Installation

Install Atlas using Cargo:

```bash
cargo install atlas
```

## 🚀 Quick Start

### 1. Initialize Your Project

Navigate to your Rust project and run:

```bash
atlas initialize
```

This will:
- ✅ Detect your system configuration
- ✅ Install optimized Cargo configuration
- ✅ Add optimized build profiles to Cargo.toml
- ✅ Install required optimization tools
- ✅ Create optimization scripts

### 2. Start Building Faster

```bash
# Fast syntax check
atlas dev quick-check

# Optimized build
atlas build build

# Fast testing with cargo-nextest
atlas build test

# Continuous development with watch mode
atlas dev watch
```

### 3. Monitor Performance

```bash
# Check optimization status
atlas status

# Detailed performance analysis
atlas dev profile
```

## 📈 Performance Improvements

| Optimization | Speed Improvement | Use Case |
|--------------|------------------|----------|
| **Incremental compilation** | 50-80% | Development builds |
| **sccache** | 30-50% | Clean builds |
| **Fast linker (lld/mold)** | 20-30% | Linking phase |
| **cargo-nextest** | 2-3x | Test execution |
| **Parallel compilation** | 20-40% | Multi-core utilization |
| **Optimized profiles** | 15-25% | Debug builds |

## 🛠️ Commands

### Build Commands
```bash
atlas build check      # Fast cargo check
atlas build build      # Optimized cargo build
atlas build test       # Fast testing with nextest
atlas build clean      # Clean build artifacts
```

### Development Workflow
```bash
atlas dev quick-check  # Ultra-fast syntax check
atlas dev watch        # Continuous development
atlas dev profile      # Build performance analysis
atlas dev clean-build  # Clean optimized build
```

### Tool Management
```bash
atlas install-tools           # Install all recommended tools
atlas install-tools --list    # List available tools
atlas install-tools --only sccache,cargo-nextest  # Install specific tools
```

### Configuration
```bash
atlas config show      # Show current configuration
atlas config edit      # Edit configuration
atlas config validate  # Validate configuration
atlas config reset     # Reset to defaults
```

### Status & Monitoring
```bash
atlas status           # Show optimization status
atlas status --detailed # Detailed system information
atlas status --json    # JSON output for scripting
```

## 🔧 What Gets Optimized

### Cargo Configuration (`.cargo/config.toml`)
- **Parallel compilation** using all CPU cores
- **Fast linkers** (zld for macOS, mold for Linux, lld for Windows)
- **Separate rust-analyzer** target directory to avoid conflicts
- **Incremental compilation** settings
- **Optimized dependency resolution** with sparse registry protocol

### Build Profiles (`Cargo.toml`)
- **Development profile** optimized for fast compilation
- **Release profile** optimized for performance with thin LTO
- **Test profile** for faster test execution
- **Dependencies optimization** in development mode

### Tool Installation
- **sccache** - Compilation cache for faster builds
- **cargo-nextest** - Fast test runner (2-3x faster than cargo test)
- **cargo-udeps** - Find unused dependencies
- **cargo-hakari** - Workspace optimization
- **cargo-watch** - Auto-rebuild on file changes
- **Fast linkers** - Platform-specific fast linkers

## 🎯 Platform-Specific Optimizations

### macOS
- **Apple Silicon (M1/M2)**: Optimized for native performance
- **Intel Macs**: Uses zld fast linker
- **Homebrew integration** for tool installation

### Linux
- **mold linker**: Fastest available linker for Linux
- **Package manager detection**: Supports apt, yum, pacman
- **Multi-distribution support**

### Windows
- **LLD linker**: LLVM linker for faster linking
- **winget integration** for tool installation

## 📊 Configuration

Atlas uses a TOML configuration file located at:
- **macOS**: `~/Library/Application Support/atlas/config.toml`
- **Linux**: `~/.config/atlas/config.toml`
- **Windows**: `%APPDATA%\atlas\config.toml`

### Example Configuration

```toml
[build]
parallel_jobs = 8
incremental = true
target_cpu = "native"
use_fast_linker = true
separate_rust_analyzer_target = true
enable_sccache = true

[tools]
auto_install = true
preferred_tools = ["sccache", "cargo-nextest", "cargo-udeps"]
install_timeout_seconds = 300

[optimization]
clean_old_artifacts = true
artifact_retention_days = 7
check_unused_deps = true
optimize_profiles = true

[development]
watch_mode_enabled = true
watch_paths = ["src", "Cargo.toml"]
auto_test_on_change = false
quick_check_on_save = true
```

## 🔍 Troubleshooting

### Build Errors
```bash
# Check for compilation errors
atlas build check

# Validate configuration
atlas config validate

# Reset configuration if needed
atlas config reset --force
```

### Performance Issues
```bash
# Profile build performance
atlas dev profile

# Check tool installation status
atlas status --detailed

# Clean and rebuild
atlas build clean --all
```

### Tool Installation Issues
```bash
# List available tools and their status
atlas install-tools --list

# Install tools manually
cargo install sccache cargo-nextest --locked

# Check system requirements
atlas status
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## 📄 License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

- The Rust community for creating amazing tools like sccache and cargo-nextest
- The LLVM project for fast linkers
- All contributors who help make Rust builds faster

---

**Happy coding! 🦀**
