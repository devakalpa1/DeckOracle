.PHONY: help dev build test clean migrate db-reset db-create db-drop lint fmt check install

# Default target
help:
	@echo "Available commands:"
	@echo "  make dev        - Run development server with hot reload"
	@echo "  make build      - Build release binary"
	@echo "  make test       - Run tests"
	@echo "  make migrate    - Run database migrations"
	@echo "  make db-reset   - Reset database (drop, create, migrate)"
	@echo "  make db-create  - Create database"
	@echo "  make db-drop    - Drop database"
	@echo "  make lint       - Run clippy linter"
	@echo "  make fmt        - Format code"
	@echo "  make check      - Check code without building"
	@echo "  make install    - Install dependencies and tools"

# Development server with hot reload
dev:
	cargo watch -x run

# Build release binary
build:
	cargo build --release

# Run tests
test:
	cargo test

# Run database migrations
migrate:
	sqlx migrate run

# Create database
db-create:
	sqlx database create

# Drop database
db-drop:
	sqlx database drop

# Reset database (drop, create, migrate)
db-reset: db-drop db-create migrate
	@echo "Database reset complete"

# Run clippy linter
lint:
	cargo clippy -- -D warnings

# Format code
fmt:
	cargo fmt

# Check code without building
check:
	cargo check

# Install required tools
install:
	cargo install sqlx-cli --no-default-features --features postgres
	cargo install cargo-watch
	@echo "Tools installed successfully"

# Clean build artifacts
clean:
	cargo clean
