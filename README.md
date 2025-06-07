# 🚀 Rust Build Optimizer

[![Crates.io](https://img.shields.io/crates/v/rust-build-optimizer.svg)](https://crates.io/crates/rust-build-optimizer)
[![Documentation](https://docs.rs/rust-build-optimizer/badge.svg)](https://docs.rs/rust-build-optimizer)


A comprehensive Rust build optimization tool that dramatically improves build times and development workflow through intelligent configuration, tool management, and performance monitoring.

## ✨ Features

- **🚀 Dramatic Build Speed Improvements**: 27-50% faster incremental builds, 2-3x faster testing
- **🔧 Intelligent Configuration**: Auto-generates optimized Cargo configurations for your system
- **🛠️ Tool Management**: Automatically installs and manages optimization tools (sccache, cargo-nextest, fast linkers)
- **📊 Performance Monitoring**: Real-time build statistics and performance analysis
- **⚡ Development Workflow**: Watch mode, quick checks, and continuous development tools
- **🎯 Cross-Platform**: Supports macOS, Linux, and Windows with platform-specific optimizations

## 📦 Installation

Install rust-build-optimizer using Cargo:

```bash
cargo install rust-build-optimizer
```

## 🚀 Quick Start

### 1. Initialize Your Project

Navigate to your Rust project and run:

```bash
rust-build-optimizer initialize
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
rust-build-optimizer dev quick-check

# Optimized build
rust-build-optimizer build build

# Fast testing with cargo-nextest
rust-build-optimizer build test

# Continuous development with watch mode
rust-build-optimizer dev watch
```

### 3. Monitor Performance

```bash
# Check optimization status
rust-build-optimizer status

# Detailed performance analysis
rust-build-optimizer dev profile
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
rust-build-optimizer build check      # Fast cargo check
rust-build-optimizer build build      # Optimized cargo build
rust-build-optimizer build test       # Fast testing with nextest
rust-build-optimizer build clean      # Clean build artifacts
```

### Development Workflow
```bash
rust-build-optimizer dev quick-check  # Ultra-fast syntax check
rust-build-optimizer dev watch        # Continuous development
rust-build-optimizer dev profile      # Build performance analysis
rust-build-optimizer dev clean-build  # Clean optimized build
```

### Tool Management
```bash
rust-build-optimizer install-tools           # Install all recommended tools
rust-build-optimizer install-tools --list    # List available tools
rust-build-optimizer install-tools --only sccache,cargo-nextest  # Install specific tools
```

### Configuration
```bash
rust-build-optimizer config show      # Show current configuration
rust-build-optimizer config edit      # Edit configuration
rust-build-optimizer config validate  # Validate configuration
rust-build-optimizer config reset     # Reset to defaults
```

### Status & Monitoring
```bash
rust-build-optimizer status           # Show optimization status
rust-build-optimizer status --detailed # Detailed system information
rust-build-optimizer status --json    # JSON output for scripting
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

rust-build-optimizer uses a TOML configuration file located at:
- **macOS**: `~/Library/Application Support/rust-build-optimizer/config.toml`
- **Linux**: `~/.config/rust-build-optimizer/config.toml`
- **Windows**: `%APPDATA%\rust-build-optimizer\config.toml`

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
rust-build-optimizer build check

# Validate configuration
rust-build-optimizer config validate

# Reset configuration if needed
rust-build-optimizer config reset --force
```

### Performance Issues
```bash
# Profile build performance
rust-build-optimizer dev profile

# Check tool installation status
rust-build-optimizer status --detailed

# Clean and rebuild
rust-build-optimizer build clean --all
```

### Tool Installation Issues
```bash
# List available tools and their status
rust-build-optimizer install-tools --list

# Install tools manually
cargo install sccache cargo-nextest --locked

# Check system requirements
rust-build-optimizer status
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
