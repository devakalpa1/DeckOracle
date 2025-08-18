# DeckOracle 🔮

[![Backend CI](https://github.com/deckoracle/deckoracle/actions/workflows/backend.yml/badge.svg)](https://github.com/deckoracle/deckoracle/actions/workflows/backend.yml)
[![Frontend CI](https://github.com/deckoracle/deckoracle/actions/workflows/frontend.yml/badge.svg)](https://github.com/deckoracle/deckoracle/actions/workflows/frontend.yml)
[![Security Scan](https://github.com/deckoracle/deckoracle/actions/workflows/security.yml/badge.svg)](https://github.com/deckoracle/deckoracle/actions/workflows/security.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 🎯 Project Overview

DeckOracle is a modern, AI-powered adaptive learning flashcard platform designed to revolutionize how people study and retain information. Built with performance, scalability, and user autonomy in mind, DeckOracle combines intelligent spaced repetition algorithms with a beautiful, intuitive interface.

### Key Features

- 🧠 **AI-Powered Learning**: Optional AI recommendations that adapt to your learning style
- 📚 **Flexible Study Modes**: Choose from spaced repetition, free study, sequential review, or random selection
- 🎨 **Rich Content Support**: Create cards with text, LaTeX math, images, and multimedia
- 👥 **Collaboration**: Share decks and study together with real-time synchronization
- 📊 **Detailed Analytics**: Track your progress with comprehensive learning insights
- 🔒 **Privacy-First**: Your data, your control - with granular privacy settings
- ♿ **Accessible**: WCAG 2.1 AA compliant with full keyboard navigation

## 🏗️ Architecture

![Architecture Diagram](docs/architecture.png)

### Tech Stack

#### Backend
- **Language**: Rust 1.79+
- **Framework**: Axum (async web framework)
- **Database**: PostgreSQL 15 with SQLx
- **Caching**: Redis 7
- **Authentication**: JWT with Argon2 password hashing

#### Frontend
- **Framework**: React 18 with TypeScript
- **Bundler**: Vite
- **Styling**: Tailwind CSS
- **State Management**: Zustand
- **Testing**: Jest + React Testing Library

#### Infrastructure
- **Containerization**: Docker & Docker Compose
- **CI/CD**: GitHub Actions
- **Monitoring**: Prometheus + Grafana (coming soon)
- **Documentation**: OpenAPI/Swagger

## 🚀 Quick Start

### Prerequisites

- Docker Desktop (includes Docker Compose)
- Make (optional, for convenience commands)
- Git

### Setup

1. **Clone the repository**
   ```bash
   git clone https://github.com/deckoracle/deckoracle.git
   cd deckoracle
   ```

2. **Configure environment**
   ```bash
   make setup
   # Or manually:
   cp backend/.env.example backend/.env
   # Edit .env with your configuration
   ```

3. **Start the development environment**
   ```bash
   make up
   # Or with Docker Compose directly:
   docker compose --profile dev up -d --build
   ```

4. **Run database migrations**
   ```bash
   make db-migrate
   ```

5. **Access the application**
   - Frontend: http://localhost:5173
   - Backend API: http://localhost:8080
   - pgAdmin: http://localhost:5050 (admin@deckoracle.com / admin)

### Stopping the Environment

```bash
make down        # Stop containers
make down-volumes # Stop and remove all data
```

## 💻 Development Workflow

### Hot Reload

Both frontend and backend support hot reload in development:

- **Backend**: Uses `cargo-watch` to automatically rebuild on file changes
- **Frontend**: Vite dev server with HMR (Hot Module Replacement)

### Common Commands

```bash
# Development
make backend-dev    # Enter backend container shell
make frontend-dev   # Enter frontend container shell
make db-shell      # Access PostgreSQL CLI
make logs          # View all service logs
make logs-backend  # View backend logs only
make logs-frontend # View frontend logs only

# Testing
make test          # Run all tests
make backend-test  # Run backend tests
make frontend-test # Run frontend tests

# Code Quality
make lint          # Run linters
make format        # Format code
make check         # Check formatting and linting

# Database
make db-migrate    # Run migrations
make db-reset      # Reset database
make backup-db     # Backup database
```

### Project Structure

```
DeckOracle/
├── backend/               # Rust/Axum backend
│   ├── src/              # Source code
│   ├── migrations/       # SQLx migrations
│   ├── tests/           # Integration tests
│   └── Cargo.toml       # Rust dependencies
├── frontend/             # React frontend
│   ├── src/             # Source code
│   ├── public/          # Static assets
│   └── package.json     # Node dependencies
├── .github/
│   └── workflows/       # CI/CD pipelines
├── docs/                # Documentation
├── docker-compose.yml   # Docker services
└── Makefile            # Development commands
```

## 🧪 Testing

### Backend Testing

```bash
cd backend
cargo test              # Run all tests
cargo test --lib       # Unit tests only
cargo test --test '*'  # Integration tests only
cargo tarpaulin        # Generate coverage report
```

### Frontend Testing

```bash
cd frontend
npm test               # Run tests in watch mode
npm test -- --coverage # Generate coverage report
npm run test:e2e      # End-to-end tests
npm run test:a11y     # Accessibility tests
```

## 📚 API Documentation

API documentation is available via Swagger UI when the backend is running:

- Development: http://localhost:8080/swagger-ui
- [API Reference](docs/api-reference.md) (coming soon)

### Main API Endpoints

- `POST /api/v1/auth/register` - User registration
- `POST /api/v1/auth/login` - User login
- `GET /api/v1/decks` - List decks
- `POST /api/v1/decks` - Create deck
- `GET /api/v1/decks/:id/cards` - Get deck cards
- `POST /api/v1/study/session` - Start study session
- `WebSocket /ws/study/:session_id` - Real-time study updates

## 🔄 CI/CD Pipeline

### Automated Workflows

1. **Backend CI** (`backend.yml`)
   - Rust formatting check
   - Clippy linting
   - Unit and integration tests
   - Security audit
   - Code coverage

2. **Frontend CI** (`frontend.yml`)
   - ESLint and Prettier checks
   - TypeScript compilation
   - Jest tests
   - Build verification
   - Lighthouse performance audit

3. **Security Scanning** (`security.yml`)
   - CodeQL analysis
   - Dependency vulnerability scanning
   - Docker image scanning
   - Secret detection

### Branch Protection

- `main` branch requires:
  - All CI checks passing
  - Code review approval
  - Up-to-date with base branch

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Commit Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add new study mode
fix: resolve deck sharing issue
docs: update API documentation
chore: upgrade dependencies
test: add unit tests for auth
```

### Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'feat: add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 🔐 Security

### Reporting Security Issues

Please report security vulnerabilities to security@deckoracle.com. Do not open public issues for security problems.

### Security Features

- 🔒 TLS 1.3 encryption
- 🛡️ OWASP Top 10 protection
- 🔑 Secure session management
- 🚫 Rate limiting and DDoS protection
- 📝 Comprehensive audit logging

## 📊 Performance

### Benchmarks

- Backend response time: < 100ms (p95)
- Frontend load time: < 2s (initial)
- Database queries: < 50ms (p95)
- WebSocket latency: < 100ms

### Optimization Strategies

- Database query optimization with proper indexing
- Redis caching for frequently accessed data
- React code splitting and lazy loading
- CDN for static assets
- Connection pooling

## 🗺️ Roadmap

- [ ] Mobile applications (iOS/Android)
- [ ] Offline mode with sync
- [ ] Advanced AI features
- [ ] Plugin system
- [ ] Marketplace for shared decks
- [ ] Video/audio card support
- [ ] VR study mode

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Rust community for excellent libraries
- React team for the amazing framework
- Contributors and early adopters
- Open source projects that inspire us

## 📞 Support

- Documentation: [docs.deckoracle.com](https://docs.deckoracle.com)
- Discord: [discord.gg/deckoracle](https://discord.gg/deckoracle)
- Email: support@deckoracle.com
- Issues: [GitHub Issues](https://github.com/deckoracle/deckoracle/issues)

---

Built with ❤️ by the DeckOracle Team