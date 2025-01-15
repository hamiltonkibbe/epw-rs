
# Makefile for Rust Library Project

# Variables
CARGO := cargo
TARGET := target
TEST_FLAGS := --all --all-features

# Default target
all: build

## Build the library
build:
	$(CARGO) build --all-features

## Clean the build artifacts
clean:
	$(CARGO) clean

## Run tests
test:
	$(CARGO) test $(TEST_FLAGS)

## Run tests with updated dependencies
test-full: clean
	$(CARGO) test $(TEST_FLAGS)

## Format the code
format:
	$(CARGO) fmt

## Run clippy for linting
lint:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

## Check for common issues
check:
	$(CARGO) check

## Build documentation
doc:
	$(CARGO) doc --all-features --open

.PHONY: all build clean test test-full format lint check doc