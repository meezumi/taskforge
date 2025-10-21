.PHONY: help install dev backend frontend docker-up docker-down clean test

help:
	@echo "TaskForge Development Commands"
	@echo "=============================="
	@echo "install        - Install all dependencies"
	@echo "dev            - Run both backend and frontend in development mode"
	@echo "backend        - Run backend server"
	@echo "frontend       - Run frontend dev server"
	@echo "docker-up      - Start Docker services (PostgreSQL, Redis, MinIO)"
	@echo "docker-down    - Stop Docker services"
	@echo "test           - Run all tests"
	@echo "clean          - Clean build artifacts"

install:
	@echo "📦 Installing Rust toolchain additions..."
	rustup target add wasm32-unknown-unknown
	cargo install trunk
	cargo install sqlx-cli --no-default-features --features postgres
	@echo "✅ Installation complete!"

docker-up:
	@echo "🐳 Starting Docker services..."
	docker-compose up -d postgres redis minio
	@echo "⏳ Waiting for services to be ready..."
	sleep 5
	@echo "✅ Services are running!"

docker-down:
	@echo "🛑 Stopping Docker services..."
	docker-compose down

backend:
	@echo "🚀 Starting backend server..."
	cd backend && cargo run

frontend:
	@echo "🎨 Starting frontend dev server..."
	cd frontend && trunk serve

dev: docker-up
	@echo "🚀 Starting TaskForge in development mode..."
	@echo "Backend will be available at http://localhost:3000"
	@echo "Frontend will be available at http://localhost:8080"

test:
	@echo "🧪 Running tests..."
	cd backend && cargo test
	cd frontend && cargo test

clean:
	@echo "🧹 Cleaning build artifacts..."
	cd backend && cargo clean
	cd frontend && cargo clean
	rm -rf frontend/dist
	@echo "✅ Clean complete!"
