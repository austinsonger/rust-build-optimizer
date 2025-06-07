# Makefile for rust-build-optimizer development

.PHONY: help install build test check fmt clippy clean release docker audit deps update bench docs serve-docs setup-hooks

# Default target
help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

install: ## Install the binary
	cargo install --path .

build: ## Build the project
	cargo build

build-release: ## Build the project in release mode
	cargo build --release

test: ## Run all tests
	cargo test

test-ignored: ## Run ignored tests
	cargo test -- --ignored

check: ## Run cargo check
	cargo check --all-targets --all-features

fmt: ## Format code
	cargo fmt --all

fmt-check: ## Check code formatting
	cargo fmt --all -- --check

clippy: ## Run clippy lints
	cargo clippy --all-targets --all-features -- -D warnings

clean: ## Clean build artifacts
	cargo clean

release: ## Create a release build with optimizations
	cargo build --release

docker: ## Build Docker image
	docker build -t rust-build-optimizer .

docker-run: ## Run Docker container
	docker run --rm -it rust-build-optimizer

audit: ## Run security audit
	cargo audit

deps: ## Check dependency status
	cargo tree
	@echo "\n--- Outdated dependencies ---"
	cargo outdated || echo "cargo-outdated not installed"

update: ## Update dependencies
	cargo update

bench: ## Run benchmarks
	cargo bench

docs: ## Build documentation
	cargo doc --no-deps --all-features

serve-docs: ## Build and serve documentation
	cargo doc --no-deps --all-features --open

setup-hooks: ## Set up git hooks
	chmod +x .githooks/pre-commit
	git config core.hooksPath .githooks

# CI-like checks
ci-check: fmt-check clippy test audit ## Run all CI checks locally

# Development workflow
dev: ## Run development workflow (format, check, test)
	$(MAKE) fmt
	$(MAKE) check
	$(MAKE) test

# Release preparation
pre-release: ## Prepare for release (all checks + build)
	$(MAKE) ci-check
	$(MAKE) build-release
	$(MAKE) docs

# Install development tools
install-tools: ## Install development tools
	cargo install cargo-audit
	cargo install cargo-outdated
	cargo install cargo-deny
	cargo install cargo-criterion
	cargo install cargo-llvm-cov

# Performance profiling
profile: ## Profile the application
	cargo build --release
	@echo "Build complete. Run with profiling tools as needed."

# Watch for changes (requires cargo-watch)
watch: ## Watch for changes and run tests
	cargo watch -x check -x test

watch-docs: ## Watch for changes and rebuild docs
	cargo watch -x "doc --no-deps --all-features"
