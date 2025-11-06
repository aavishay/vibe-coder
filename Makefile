# Makefile for Vibe Coder development

# Default target
.PHONY: help
help: ## Show this help message
	@echo "Vibe Coder Development Commands:"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# Build targets
.PHONY: build
build: ## Build the project in debug mode
	cargo build

.PHONY: release
release: ## Build the project in release mode
	cargo build --release

.PHONY: run
run: ## Run the project in debug mode
	cargo run

.PHONY: run-release
run-release: ## Run the project in release mode
	cargo run --release

# Test targets
.PHONY: test
test: ## Run all tests
	cargo test

.PHONY: test-verbose
test-verbose: ## Run all tests with output
	cargo test -- --nocapture

# Code quality targets
.PHONY: fmt
fmt: ## Format code with rustfmt
	cargo fmt

.PHONY: fmt-check
fmt-check: ## Check code formatting
	cargo fmt -- --check

.PHONY: clippy
clippy: ## Run clippy linter
	cargo clippy

.PHONY: clippy-fix
clippy-fix: ## Run clippy and apply automatic fixes
	cargo clippy --fix

# Coverage targets
.PHONY: coverage
coverage: ## Generate code coverage report
	cargo tarpaulin --ignore-tests --line --output-dir coverage

.PHONY: coverage-html
coverage-html: ## Generate HTML coverage report
	cargo tarpaulin --ignore-tests --line --output-dir coverage --out Html

.PHONY: coverage-check
coverage-check: ## Check code coverage meets 90% requirement
	cargo tarpaulin --ignore-tests --line --fail-under 90

# Pre-commit targets
.PHONY: setup-hooks
setup-hooks: ## Install pre-commit hooks
	./scripts/setup-precommit.sh

.PHONY: precommit
precommit: ## Run all pre-commit checks
	pre-commit run --all-files

# Clean targets
.PHONY: clean
clean: ## Clean build artifacts
	cargo clean

# Documentation targets
.PHONY: doc
doc: ## Generate documentation
	cargo doc --open

# Install dependencies
.PHONY: install-deps
install-deps: ## Install development dependencies
	cargo install cargo-tarpaulin pre-commit