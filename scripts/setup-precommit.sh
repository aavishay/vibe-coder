#!/bin/bash

# Setup script for pre-commit hooks and coverage requirements

set -e  # Exit on any error

echo "Setting up pre-commit hooks and coverage requirements..."

# Check if pre-commit is installed
if ! command -v pre-commit &> /dev/null
then
    echo "Installing pre-commit..."
    pip install pre-commit
fi

# Check if cargo-tarpaulin is installed
if ! command -v cargo-tarpaulin &> /dev/null
then
    echo "Installing cargo-tarpaulin..."
    cargo install cargo-tarpaulin
fi

# Install the pre-commit hooks
echo "Installing pre-commit hooks..."
pre-commit install

echo "Pre-commit hooks installed successfully!"

# Run pre-commit to verify installation
echo "Running pre-commit checks to verify installation..."
pre-commit run --all-files

echo "Setup complete! Pre-commit hooks will now run on every commit."
echo "They will check for:"
echo "  - Code formatting (cargo fmt)"
echo "  - Linting (cargo clippy)"
echo "  - Tests (cargo test)"
echo "  - Code coverage (90% minimum with cargo-tarpaulin)"