# TaskForge 🚀

A modern, multi-tenant SaaS project management platform built with Rust and WebAssembly.

## 🎯 Features

- **Multi-Tenant Architecture**: Complete data isolation between organizations
- **Role-Based Access Control (RBAC)**: Admin, Manager, Member roles with fine-grained permissions
- **Real-Time Collaboration**: WebSocket-based real-time updates
- **File Management**: Secure file uploads and storage
- **Task Management**: Projects, tasks, subtasks with dependencies
- **Team Collaboration**: Comments, mentions, activity feeds
- **API**: RESTful API with JWT authentication
- **Modern Frontend**: Built with Rust WebAssembly (Leptos framework)

## 🛠️ Tech Stack

### Backend
- **Framework**: Axum (async web framework)
- **Database**: PostgreSQL with SQLx
- **Authentication**: JWT tokens
- **Real-time**: WebSockets
- **Caching**: Redis
- **Storage**: Local/S3-compatible (MinIO for development)

### Frontend
- **Framework**: Leptos (Rust WASM)
- **Styling**: TailwindCSS
- **State Management**: Leptos signals

### DevOps
- **Containerization**: Docker & Docker Compose
- **CI/CD**: GitHub Actions
- **Testing**: Cargo test + integration tests

## 🚀 Quick Start

### Prerequisites
- Rust 1.90+ (install from https://rustup.rs)
- Docker & Docker Compose
- Node.js (for TailwindCSS)

### Development Setup

1. **Clone and navigate to project**
   ```bash
   cd /home/kuro/Downloads/projects/taskforge
   ```

2. **Start infrastructure services**
   ```bash
   docker-compose up -d postgres redis minio
   ```

3. **Run database migrations**
   ```bash
   cd backend
   cargo install sqlx-cli
   sqlx migrate run
   ```

4. **Start backend server**
   ```bash
   cd backend
   cargo run
   ```

5. **Start frontend development server**
   ```bash
   cd frontend
   trunk serve
   ```

6. **Access the application**
   - Frontend: http://localhost:8080
   - Backend API: http://localhost:3000
   - MinIO Console: http://localhost:9001

## 📚 Documentation

- [Architecture Overview](docs/architecture.md)
- [API Documentation](docs/api.md)
- [Database Schema](docs/database.md)
- [Deployment Guide](docs/deployment.md)

## 🧪 Testing

```bash
# Backend tests
cd backend
cargo test

# Frontend tests
cd frontend
cargo test
```

## 🔒 Security Features

- JWT-based authentication
- Password hashing with Argon2
- CORS configuration
- SQL injection prevention (SQLx compile-time checks)
- XSS protection
- Rate limiting


Built using Rust
