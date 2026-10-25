#!/usr/bin/env bash
set -euo pipefail

echo "Running workspace-wide cargo check..."
cargo check --workspace --all-targets --all-features

echo "Running workspace test suite..."
cargo test --workspace

echo "All workspace checks passed successfully!"
