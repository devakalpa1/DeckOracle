# DeckOracle Backend

A high-performance, scalable backend for the DeckOracle flashcard learning platform built with Rust, Axum, and PostgreSQL.

## 🚀 Features

- **RESTful API** - Clean REST endpoints for all resources
- **Hierarchical Organization** - Folders contain decks, decks contain cards
- **CSV Import/Export** - Bulk card management via CSV
- **Study Sessions** - Track learning progress and statistics
- **Type-Safe** - Leverages Rust's type system for reliability
- **Async/Await** - Fully asynchronous with Tokio runtime
- **Database Migrations** - Version-controlled schema with SQLx

## 📁 Project Structure

```
backend/
├── src/
│   ├── config/         # Configuration management
│   ├── handlers/       # HTTP request handlers
│   ├── models/         # Data models and DTOs
│   ├── services/       # Business logic layer
│   ├── middleware/     # Auth and other middleware
│   ├── utils/          # Utilities and error handling
│   ├── state.rs        # Application state
│   └── main.rs         # Entry point
├── migrations/         # SQL migrations
├── Cargo.toml         # Dependencies
└── .env               # Environment variables
```

## 🛠️ Prerequisites

- Rust 1.70+ 
- PostgreSQL 14+
- cargo-watch (optional, for hot reload)

## 🏃 Quick Start

1. **Clone the repository**
```bash
cd backend
```

2. **Set up PostgreSQL**
```bash
# Create database
createdb deckoracle_db

# Or using psql
psql -U postgres -c "CREATE DATABASE deckoracle_db;"
```

3. **Configure environment**
```bash
# Copy example env file
cp .env.example .env

# Edit .env with your database credentials
```

4. **Install SQLx CLI**
```bash
cargo install sqlx-cli --no-default-features --features postgres
```

5. **Run migrations**
```bash
sqlx migrate run
```

6. **Run the server**
```bash
# Development
cargo run

# With hot reload
cargo watch -x run

# Production build
cargo build --release
./target/release/deckoracle-backend
```

The server will start on `http://localhost:8080`

## 📚 API Documentation

### Base URL
```
http://localhost:8080/api/v1
```

### Endpoints

#### Folders
- `GET /folders` - List all user folders
- `POST /folders` - Create a new folder
- `GET /folders/:id` - Get folder details
- `PATCH /folders/:id` - Update folder
- `DELETE /folders/:id` - Delete folder
- `GET /folders/:id/contents` - Get folder with subfolders and decks

#### Decks
- `GET /decks` - List all user decks
- `POST /decks` - Create a new deck
- `GET /decks/:id` - Get deck details
- `GET /decks/:id/stats` - Get deck with statistics
- `PATCH /decks/:id` - Update deck
- `DELETE /decks/:id` - Delete deck
- `POST /decks/:id/csv` - Import cards from CSV
- `GET /decks/:id/csv` - Export cards to CSV

#### Cards
- `GET /cards?deck_id=uuid` - List cards in a deck
- `POST /cards?deck_id=uuid` - Create a new card
- `POST /cards/bulk?deck_id=uuid` - Bulk create cards
- `GET /cards/:id` - Get card details
- `PATCH /cards/:id` - Update card
- `DELETE /cards/:id` - Delete card

#### Study Sessions
- `GET /study/sessions` - List user study sessions
- `POST /study/sessions` - Create new study session
- `GET /study/sessions/:id` - Get session details
- `POST /study/sessions/:id/complete` - Complete session
- `GET /study/sessions/:id/progress` - Get session progress
- `POST /study/sessions/:id/progress` - Record card progress

### Request/Response Examples

#### Create a Deck
```json
POST /api/v1/decks
{
  "name": "Spanish Vocabulary",
  "description": "Common Spanish words and phrases",
  "folder_id": "uuid-here",
  "is_public": false
}
```

#### Import CSV
```csv
POST /api/v1/decks/:id/csv
Content-Type: text/csv

front,back
Hello,Hola
Goodbye,Adiós
Thank you,Gracias
```

## 🔧 Development

### Running Tests
```bash
cargo test
```

### Linting
```bash
cargo clippy -- -D warnings
```

### Formatting
```bash
cargo fmt
```

### Database Reset
```bash
sqlx database drop
sqlx database create
sqlx migrate run
```

## 🐳 Docker Support

```bash
# Build image
docker build -t deckoracle-backend .

# Run container
docker run -p 8080:8080 --env-file .env deckoracle-backend
```

## 🔐 Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| DATABASE_URL | PostgreSQL connection string | Required |
| SERVER_HOST | Server bind address | 127.0.0.1 |
| SERVER_PORT | Server port | 8080 |
| CORS_ORIGIN | Allowed CORS origin | http://localhost:5173 |
| JWT_SECRET | JWT signing secret | Required for auth |
| RUST_LOG | Log level | debug |

## 🏗️ Architecture

### Layers
1. **Handlers** - HTTP request/response handling
2. **Services** - Business logic and validation
3. **Models** - Data structures and DTOs
4. **Database** - PostgreSQL with SQLx

### Key Technologies
- **Axum** - Web framework
- **SQLx** - Compile-time checked SQL
- **Tokio** - Async runtime
- **Tower** - Middleware and services
- **Serde** - Serialization/deserialization

## 📝 TODO

- [ ] Add authentication middleware
- [ ] Implement user registration/login
- [ ] Add WebSocket support for real-time features
- [ ] Implement spaced repetition algorithm
- [ ] Add file upload for multimedia cards
- [ ] Create OpenAPI documentation
- [ ] Add rate limiting
- [ ] Implement caching with Redis

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

## 📄 License

MIT License - see LICENSE file for details

## 🆘 Troubleshooting

### Database Connection Issues
- Ensure PostgreSQL is running
- Check DATABASE_URL in .env
- Verify database exists: `psql -U postgres -l`

### Migration Errors
- Run `sqlx migrate revert` to rollback
- Check migration files in `/migrations`
- Ensure database user has CREATE permissions

### Compilation Errors
- Update Rust: `rustup update`
- Clear cache: `cargo clean`
- Check dependencies: `cargo tree`

## 📧 Support

For issues and questions, please open a GitHub issue.
