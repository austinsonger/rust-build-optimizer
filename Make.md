# 🔧 Makefile Documentation

This document explains how to use the Makefile in the rust-build-optimizer project for development, testing, and release workflows.

## 🚀 **Quick Start**

### **View Available Commands**
Start by running this to see all available targets:
```bash
make help
```

## 📋 **Available Commands**

### **🏗️ Build Commands**
```bash
make build          # Build the project in debug mode
make build-release  # Build the project in release mode
make install        # Install the binary locally
make clean          # Clean build artifacts
```

### **🧪 Testing Commands**
```bash
make test           # Run all tests
make test-ignored   # Run ignored tests
make check          # Run cargo check (fast syntax/type check)
make bench          # Run benchmarks
```

### **🎨 Code Quality Commands**
```bash
make fmt            # Format all code
make fmt-check      # Check if code is properly formatted
make clippy         # Run clippy lints
make ci-check       # Run all CI checks (format, clippy, test, audit)
```

### **📚 Documentation Commands**
```bash
make docs           # Build documentation
make serve-docs     # Build and open documentation in browser
make watch-docs     # Watch for changes and rebuild docs automatically
```

### **🔍 Development Tools**
```bash
make dev            # Run complete development workflow (format, check, test)
make watch          # Watch for changes and auto-run check and test
make profile        # Build for performance profiling
```

### **🔒 Security & Dependencies**
```bash
make audit          # Run security audit on dependencies
make deps           # Check dependency tree and outdated packages
make update         # Update dependencies
```

### **🚀 Release Commands**
```bash
make release        # Create optimized release build
make pre-release    # Complete release preparation (all checks + build + docs)
```

### **🐳 Docker Commands**
```bash
make docker         # Build Docker image
make docker-run     # Run Docker container
```

### **⚙️ Setup Commands**
```bash
make install-tools  # Install development tools (cargo-audit, etc.)
make setup-hooks    # Set up git hooks
```

## 🔄 **Common Development Workflows**

### **Starting Development**
```bash
# Quick development cycle - formats, checks, and tests
make dev
```

### **Continuous Development**
```bash
# Auto-run checks and tests when files change (requires cargo-watch)
make watch
```

### **Before Committing**
```bash
# Run all CI checks locally
make ci-check
```

### **Preparing for Release**
```bash
# Complete release preparation
make pre-release
```

### **Daily Development Routine**
```bash
# 1. Start with a clean check
make dev

# 2. Enable continuous testing during development
make watch

# 3. Before committing changes
make ci-check

# 4. Build and install locally to test
make install
```

## 🛠️ **Development Tools Installation**

The Makefile can install useful development tools:

```bash
make install-tools
```

This installs:
- `cargo-audit` - Security audit tool
- `cargo-outdated` - Check for outdated dependencies
- `cargo-deny` - Dependency license and security checker
- `cargo-criterion` - Benchmarking tool
- `cargo-llvm-cov` - Code coverage tool

## 📊 **Performance and Profiling**

### **Benchmarking**
```bash
make bench          # Run performance benchmarks
```

### **Profiling**
```bash
make profile        # Build optimized binary for profiling
```

### **Dependency Analysis**
```bash
make deps           # Show dependency tree and check for outdated packages
```

## 🔍 **Code Quality Checks**

### **Formatting**
```bash
make fmt            # Auto-format all code
make fmt-check      # Check formatting without making changes
```

### **Linting**
```bash
make clippy         # Run clippy with strict settings
```

### **Complete CI Check**
```bash
make ci-check       # Run format-check + clippy + test + audit
```

## 📖 **Documentation Workflow**

### **Build Documentation**
```bash
make docs           # Build docs
make serve-docs     # Build and open in browser
```

### **Continuous Documentation**
```bash
make watch-docs     # Auto-rebuild docs when files change
```

## 🐳 **Docker Workflow**

### **Build and Run Container**
```bash
make docker         # Build Docker image
make docker-run     # Run the container interactively
```

## 🔧 **Git Hooks Setup**

```bash
make setup-hooks    # Configure git hooks for automated checks
```

This sets up pre-commit hooks to run quality checks automatically.

## 💡 **Recommended Workflow**

### **Daily Development**
1. **Start**: `make dev` - Format, check, and test
2. **Develop**: `make watch` - Continuous testing
3. **Before commit**: `make ci-check` - Full quality check
4. **Install locally**: `make install` - Test the binary

### **Release Process**
1. **Prepare**: `make pre-release` - All checks + release build + docs
2. **Verify**: Test the release binary thoroughly
3. **Release**: Follow your release process

### **Maintenance**
1. **Security**: `make audit` - Check for security issues
2. **Dependencies**: `make deps` - Review dependency status
3. **Update**: `make update` - Update dependencies
4. **Clean**: `make clean` - Clean build artifacts

## 🚨 **Troubleshooting**

### **Build Issues**
```bash
make clean          # Clean all build artifacts
make check          # Quick syntax check
make build          # Full rebuild
```

### **Test Failures**
```bash
make test           # Run all tests
make test-ignored   # Run ignored tests (integration tests, etc.)
```

### **Formatting Issues**
```bash
make fmt-check      # Check what needs formatting
make fmt            # Auto-fix formatting
```

### **Tool Installation Issues**
```bash
make install-tools  # Install development tools
```

If tools fail to install, install them manually:
```bash
cargo install cargo-audit cargo-outdated cargo-watch
```

## 📝 **Notes**

- Most commands use `cargo` under the hood with optimized flags
- The Makefile is designed for both local development and CI/CD
- All formatting and linting commands use strict settings for code quality
- Docker commands help with containerized development and deployment
- Watch commands require `cargo-watch` to be installed

## 🔗 **Related Files**

- `Makefile` - The actual Makefile with all targets
- `clippy.toml` - Clippy configuration
- `rustfmt.toml` - Rust formatting configuration
- `deny.toml` - Dependency checking configuration
- `.githooks/` - Git hooks directory (if exists)

---

**Pro Tip**: Start your development session with `make dev && make watch` for the best experience! 🚀