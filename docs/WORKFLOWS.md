# GitHub Workflows

This project uses comprehensive GitHub Actions workflows to ensure code quality, security, and reliable releases.

## Workflows Overview

### 🔧 CI Workflow (`ci.yml`)
Runs on every push and pull request to main/develop branches.

**Features:**
- **Multi-platform testing**: Linux, Windows, macOS
- **Multi-version testing**: Stable and Beta Rust
- **Code quality checks**: `cargo fmt`, `cargo clippy`
- **Security auditing**: `cargo audit`
- **Code coverage**: Uses `cargo-llvm-cov` and uploads to Codecov
- **MSRV verification**: Tests against minimum supported Rust version
- **Documentation builds**: Ensures docs build without warnings
- **Dependency checks**: Uses `cargo-deny` for license and vulnerability scanning

### 🚀 Release Workflow (`release.yml`)
Triggered when tags are pushed (e.g., `v1.0.0`).

**Features:**
- **Multi-platform releases**: Builds for 6+ target platforms
- **Automated GitHub releases**: Creates releases with binaries
- **Crates.io publishing**: Automatically publishes to crates.io
- **Docker images**: Builds and pushes multi-arch Docker images
- **Cross-compilation**: Uses `cross` for additional targets

### 🔒 Security Workflow (`security.yml`)
Runs daily and on security-related changes.

**Features:**
- **Daily security audits**: `cargo audit` for known vulnerabilities
- **Supply chain security**: `cargo-vet` for dependency verification
- **Secret scanning**: `gitleaks` for leaked credentials
- **CodeQL analysis**: Advanced security analysis
- **Dependency review**: Automated dependency security checks

### 🤖 Dependabot (`dependabot.yml`)
Automated dependency updates.

**Features:**
- **Weekly updates**: Cargo, GitHub Actions, and Docker dependencies
- **Automatic PRs**: Creates pull requests for updates
- **Configurable limits**: Prevents PR spam
- **Custom labels**: Organizes dependency updates

## Required Secrets

For full functionality, configure these secrets in your GitHub repository:

### For Releases
- `CARGO_REGISTRY_TOKEN`: Token for publishing to crates.io
- `DOCKER_USERNAME`: Docker Hub username
- `DOCKER_PASSWORD`: Docker Hub token/password

### For Security (Optional)
- `CODECOV_TOKEN`: For enhanced Codecov integration

## Local Development Commands

Use the provided `Makefile` for common development tasks:

```bash
# Setup development environment
make install-tools
make setup-hooks

# Development workflow
make dev              # Format, check, and test
make ci-check         # Run all CI checks locally

# Building and testing
make build           # Debug build
make build-release   # Release build
make test           # Run tests
make bench          # Run benchmarks

# Code quality
make fmt            # Format code
make clippy         # Run lints
make audit          # Security audit

# Documentation
make docs           # Build docs
make serve-docs     # Build and open docs
```

## Pre-commit Hooks

Enable git hooks for automatic code quality checks:

```bash
make setup-hooks
```

This will run `cargo fmt`, `cargo clippy`, and `cargo test` before each commit.

## Workflow Files

- `.github/workflows/ci.yml` - Main CI pipeline
- `.github/workflows/release.yml` - Release automation
- `.github/workflows/security.yml` - Security scanning
- `.github/dependabot.yml` - Dependency updates
- `deny.toml` - Dependency policy configuration
- `clippy.toml` - Clippy linting configuration
- `rustfmt.toml` - Code formatting configuration

## Best Practices Implemented

1. **Fail Fast**: Early detection of issues with comprehensive checks
2. **Security First**: Multiple layers of security scanning
3. **Multi-platform**: Ensures compatibility across platforms
4. **Automated Updates**: Dependabot keeps dependencies current
5. **Documentation**: Automated doc building and deployment
6. **Performance**: Benchmarking and performance regression detection
7. **Supply Chain**: Verification of dependencies and build reproducibility

## Customization

To customize workflows for your needs:

1. **Modify targets**: Edit platform matrix in `release.yml`
2. **Adjust security**: Configure `deny.toml` for your license/security requirements
3. **Change schedules**: Modify cron schedules in workflow files
4. **Add secrets**: Configure additional secrets as needed for integrations
