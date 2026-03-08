#!/usr/bin/env bash
set -euo pipefail

export DOCKER_DEV_IMAGE="${DOCKER_DEV_IMAGE:-dev-rust:1.93}"
export DOCKER_TEST_CMD="${DOCKER_TEST_CMD:-cargo llvm-cov --features examples --fail-under-lines 100 --fail-under-regions 100}"

if ! command -v st-docker-test >/dev/null 2>&1; then
  echo "ERROR: st-docker-test not found on PATH." >&2
  echo "Set up standard-tooling: export PATH=../standard-tooling/.venv/bin:\$PATH" >&2
  exit 1
fi
exec st-docker-test
