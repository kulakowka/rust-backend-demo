# Rust Backend Demo

A modern, production-ready REST API and GraphQL server built with Rust, featuring a clean architecture with Service/Repository pattern, comprehensive API documentation, database integration, and **AI-powered endpoints using Google Gemini**.

## 🚀 Features

- **Dual API Support**: Both REST and GraphQL endpoints
- **AI Integration**: Google Gemini AI for chat and text generation
- **Clean Architecture**: Service/Repository pattern for maintainable code
- **OpenAPI Documentation**: Interactive Swagger UI at `/swagger-ui`
- **Database Integration**: PostgreSQL with SQLx and automatic migrations
- **Type Safety**: Full Rust type safety with validation
- **Error Handling**: Comprehensive error handling with custom error types
- **Logging**: Structured logging with tracing
- **Configuration**: Environment-based configuration with `.env` support
- **Streaming Support**: Server-Sent Events for real-time AI responses

## 📋 Tech Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum) 0.8
- **Database**: PostgreSQL with [SQLx](https://github.com/launchbadge/sqlx) 0.8
- **GraphQL**: [async-graphql](https://github.com/async-graphql/async-graphql) 7.0
- **API Docs**: [utoipa](https://github.com/juhaku/utoipa) 5.4 + Swagger UI
- **AI**: [google-generative-ai-rs](https://github.com/avastmick/google-generative-ai-rs) 0.3
- **Validation**: [validator](https://github.com/Keats/validator)
- **Async Runtime**: [Tokio](https://tokio.rs)

## 🏗️ Architecture

```
src/
├── main.rs           # Application entry point
├── config.rs         # Configuration management
├── state.rs          # Application state
├── model.rs          # Domain models
├── dto.rs            # Data Transfer Objects
├── repository.rs     # Database layer (Repository pattern)
├── service.rs        # Business logic layer
├── handler.rs        # REST API handlers
├── ai_model.rs       # AI domain models
├── ai_repository.rs  # AI client layer
├── ai_service.rs     # AI business logic
├── ai_handler.rs     # AI REST API handlers
├── schema.rs         # GraphQL schema
├── route.rs          # Route configuration
└── error.rs          # Error types
```

## 🛠️ Prerequisites

- Rust 1.70+ (edition 2024)
- PostgreSQL 15+
- Docker & Docker Compose (optional, for database)

## 📦 Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/kulakowka/rust-backend-demo.git
   cd rust-backend-demo
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   ```

   Edit `.env` and add your Gemini API key:
   ```env
   DATABASE_URL=postgres://postgres:password@localhost:5432/hello_cargo
   RUST_LOG=debug
   SERVER_HOST=127.0.0.1
   SERVER_PORT=3000
   GEMINI_API_KEY=your_gemini_api_key_here
   # Optional: specify Gemini model (defaults to gemini-2.0-flash-exp)
   # GEMINI_MODEL=gemini-3-pro
   ```

   Get your Gemini API key from [Google AI Studio](https://makersuite.google.com/app/apikey).
   
   **Supported Models**: gemini-2.0-flash-exp, gemini-3-pro, gemini-pro, and other Gemini models.

3. **Start PostgreSQL**
   ```bash
   docker-compose up -d
   ```

4. **Run the application**
   ```bash
   cargo run
   ```

   The server will start on `http://127.0.0.1:3001`

## 📚 API Documentation

### Swagger UI
Visit `http://127.0.0.1:3001/swagger-ui` for interactive API documentation.

### GraphQL Playground
Visit `http://127.0.0.1:3001/graphql` for GraphQL playground.

## 🔌 API Endpoints

### REST API

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/users` | List all users |
| GET | `/users/{id}` | Get user by ID |
| POST | `/users` | Create a new user |
| PUT | `/users/{id}` | Update user |
| DELETE | `/users/{id}` | Delete user |

### AI Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/ai/chat` | Chat with Gemini AI |
| POST | `/ai/generate` | Generate text from prompt |
| POST | `/ai/chat/stream` | Streaming chat with SSE |

**Example - Chat**:
```bash
curl -X POST http://127.0.0.1:3000/ai/chat \
  -H "Content-Type: application/json" \
  -d '{
    "message": "What is Rust?",
    "history": []
  }'
```

**Example - Generate**:
```bash
curl -X POST http://127.0.0.1:3000/ai/generate \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "Write a haiku about programming"
  }'
```

### GraphQL API

**Endpoint**: `POST /graphql`

**Queries**:
```graphql
query {
  users {
    id
    name
    email
    createdAt
    updatedAt
  }
  
  user(id: "uuid-here") {
    id
    name
    email
  }
}
```

**Mutations**:
```graphql
mutation {
  # User mutations
  createUser(input: {
    name: "John Doe"
    email: "john@example.com"
  }) {
    id
    name
    email
  }
  
  updateUser(id: "uuid-here", input: {
    name: "Jane Doe"
  }) {
    id
    name
    email
  }
  
  deleteUser(id: "uuid-here")
  
  # AI mutations
  chat(input: {
    message: "What is Rust?"
    history: []
  }) {
    response
    model
  }
  
  generate(input: {
    prompt: "Write a haiku about coding"
  }) {
    text
    model
  }
}
```

## 🧪 Testing

### Manual Testing Scripts

**Test REST API**:
```bash
chmod +x test_api.sh
./test_api.sh
```

**Test GraphQL API**:
```bash
chmod +x test_graphql.sh
./test_graphql.sh
```

### Example cURL Commands

**Create User**:
```bash
curl -X POST http://127.0.0.1:3001/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice", "email": "alice@example.com"}'
```

**Get All Users**:
```bash
curl http://127.0.0.1:3001/users
```

**Update User**:
```bash
curl -X PUT http://127.0.0.1:3001/users/{id} \
  -H "Content-Type: application/json" \
  -d '{"name": "Alice Updated"}'
```

**Delete User**:
```bash
curl -X DELETE http://127.0.0.1:3001/users/{id}
```

## 🗄️ Database

### Schema

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### Migrations

Migrations are automatically applied on startup. Migration files are located in `migrations/`.

To create a new migration:
```bash
sqlx migrate add <migration_name>
```

## 🔧 Development

### Build
```bash
cargo build
```

### Run with hot reload (using cargo-watch)
```bash
cargo install cargo-watch
cargo watch -x run
```

### Check code
```bash
cargo check
```

### Format code
```bash
cargo fmt
```

### Lint
```bash
cargo clippy
```

## 📝 Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgres://postgres:password@localhost:5432/hello_cargo` |
| `RUST_LOG` | Log level (trace, debug, info, warn, error) | `debug` |
| `SERVER_HOST` | Server host address | `127.0.0.1` |
| `SERVER_PORT` | Server port | `3001` |

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is open source and available under the MIT License.

## 🙏 Acknowledgments

Built with amazing Rust ecosystem tools:
- [Axum](https://github.com/tokio-rs/axum) - Web framework
- [SQLx](https://github.com/launchbadge/sqlx) - Async SQL toolkit
- [async-graphql](https://github.com/async-graphql/async-graphql) - GraphQL server
- [utoipa](https://github.com/juhaku/utoipa) - OpenAPI documentation
