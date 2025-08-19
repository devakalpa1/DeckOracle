.PHONY: help up down logs ps restart clean build test backend-dev frontend-dev db-migrate db-reset backend-test frontend-test lint format check

# Default target
help: ## Show this help message
	@echo "DeckOracle Development Commands"
	@echo "================================"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# Docker Compose Commands
up: ## Start all services in detached mode
	docker compose --profile dev up -d --build

up-prod: ## Start production services
	docker compose --profile prod up -d --build

down: ## Stop and remove all containers
	docker compose down

down-volumes: ## Stop all services and remove volumes
	docker compose down -v

logs: ## Show logs from all services
	docker compose logs -f

logs-backend: ## Show backend logs
	docker compose logs -f backend

logs-frontend: ## Show frontend logs
	docker compose logs -f frontend

ps: ## Show running containers
	docker compose ps

restart: ## Restart all services
	docker compose restart

restart-backend: ## Restart backend service
	docker compose restart backend

restart-frontend: ## Restart frontend service
	docker compose restart frontend

clean: ## Clean up Docker resources
	docker compose down -v
	docker system prune -f

build: ## Build all Docker images
	docker compose build

build-backend: ## Build backend Docker image
	docker compose build backend

build-frontend: ## Build frontend Docker image
	docker compose build frontend

# Development Commands
backend-dev: ## Enter backend container shell
	docker compose exec backend bash

frontend-dev: ## Enter frontend container shell
	docker compose exec frontend sh

db-shell: ## Connect to PostgreSQL shell
	docker compose exec postgres psql -U postgres -d deckoracle_db

db-migrate: ## Run database migrations
	docker compose exec backend sqlx migrate run

db-revert: ## Revert last database migration
	docker compose exec backend sqlx migrate revert

db-reset: ## Reset database (drop and recreate)
	docker compose exec postgres psql -U postgres -c "DROP DATABASE IF EXISTS deckoracle_db;"
	docker compose exec postgres psql -U postgres -c "CREATE DATABASE deckoracle_db;"
	$(MAKE) db-migrate

# Testing Commands
test: backend-test frontend-test ## Run all tests

backend-test: ## Run backend tests
	cd backend && cargo test

frontend-test: ## Run frontend tests
	cd frontend && npm test -- --watchAll=false

backend-test-watch: ## Run backend tests in watch mode
	cd backend && cargo watch -x test

frontend-test-watch: ## Run frontend tests in watch mode
	cd frontend && npm test

# Code Quality Commands
lint: ## Run linters for both backend and frontend
	cd backend && cargo clippy -- -D warnings
	cd frontend && npm run lint

format: ## Format code for both backend and frontend
	cd backend && cargo fmt
	cd frontend && npm run format

check: ## Check code formatting and linting
	cd backend && cargo fmt -- --check
	cd backend && cargo clippy -- -D warnings
	cd frontend && npm run lint

# Local Development (without Docker)
local-backend: ## Run backend locally (requires Rust and PostgreSQL)
	cd backend && cargo run

local-frontend: ## Run frontend locally (requires Node.js)
	cd frontend && npm run dev

local-db: ## Start PostgreSQL locally using Docker
	docker run -d \
		--name deckoracle-postgres-local \
		-e POSTGRES_USER=postgres \
		-e POSTGRES_PASSWORD=deckoracle_password \
		-e POSTGRES_DB=deckoracle_db \
		-p 5432:5432 \
		postgres:15-alpine

# Installation Commands
install-backend: ## Install backend dependencies
	cd backend && cargo build

install-frontend: ## Install frontend dependencies
	cd frontend && npm ci

install: install-backend install-frontend ## Install all dependencies

# Production Commands
deploy-prod: ## Build and push production images
	docker compose -f docker-compose.yml -f docker-compose.prod.yml build
	@echo "Images built. Push to registry and deploy to production."

# Utility Commands
seed-db: ## Seed database with sample data
	docker compose exec backend cargo run --bin seed

backup-db: ## Backup database
	docker compose exec postgres pg_dump -U postgres deckoracle_db > backup_$(shell date +%Y%m%d_%H%M%S).sql

restore-db: ## Restore database from backup (usage: make restore-db FILE=backup.sql)
	docker compose exec -T postgres psql -U postgres deckoracle_db < $(FILE)

# GitHub Actions Local Testing
test-ci: ## Test CI pipeline locally using act
	act -j backend
	act -j frontend

# Environment Setup
setup: ## Initial setup for development
	cp backend/.env.example backend/.env
	@echo "Environment files created. Please update .env with your configuration."
	@echo "Run 'make up' to start the development environment."
