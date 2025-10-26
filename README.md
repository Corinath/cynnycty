# Cynnycty - Social Media Platform

A modern social media platform built with Rust, SvelteKit, and ArcadeDB.

## Tech Stack

### Backend (`/be`)
- **Language**: Rust
- **Web Framework**: Axum
- **Database**: ArcadeDB
- **Authentication**: Clerk
- **Cloud Infrastructure**: Google Cloud Platform (GCP)

### Frontend (`/fe`)
- **Framework**: SvelteKit
- **Language**: TypeScript
- **Authentication**: Clerk

## Project Structure

```
.
├── be/                 # Backend (Rust + Axum)
│   ├── src/
│   │   ├── main.rs    # Application entry point
│   │   ├── routes/    # API route handlers
│   │   └── db/        # Database connection & queries
│   ├── Cargo.toml     # Rust dependencies
│   └── .env.example   # Environment variables template
│
└── fe/                 # Frontend (SvelteKit)
    ├── src/
    └── package.json
```

## Getting Started

### Prerequisites
- Rust (1.88+)
- Node.js (20+)
- Docker & Docker Compose (recommended for easy setup)
- OR ArcadeDB installed manually (if not using Docker)
- Clerk account for authentication (optional for initial setup)

### Quick Start with Docker (Recommended)

The easiest way to get started is using Docker Compose:

```bash
# Start ArcadeDB in a Docker container
docker-compose up -d

# Check that ArcadeDB is running
docker ps

# View ArcadeDB logs
docker-compose logs -f arcadedb
```

ArcadeDB will be available at:
- HTTP API: `http://localhost:2480`
- Web Console: `http://localhost:2480` (login with `root` / `cynnycty2024`)
- Binary Protocol: `localhost:2424`

The database `cynnycty` will be automatically created on first startup.

To stop the services:
```bash
docker-compose down
```

### Manual ArcadeDB Setup (Alternative)

If you prefer to run ArcadeDB without Docker:

1. Download and install ArcadeDB from https://arcadedb.com
2. Start ArcadeDB server:
   ```bash
   bin/server.sh
   ```
3. Create a database named `cynnycty` (or configure your preferred name in .env)
4. Default connection: `http://localhost:2480`

### Backend Setup

1. Navigate to the backend directory:
   ```bash
   cd be
   ```

2. Copy the environment variables template:
   ```bash
   cp .env.example .env
   ```

3. Update `.env` with your ArcadeDB configuration:
   ```env
   ARCADE_DB_HOST=localhost
   ARCADE_DB_PORT=2480
   ARCADE_DB_NAME=cynnycty
   ARCADE_DB_USER=root
   ARCADE_DB_PASSWORD=your_password
   ```

4. Build and run the backend:
   ```bash
   cargo run
   ```

The backend server will start on `http://localhost:3000`

### Frontend Setup

1. Navigate to the frontend directory:
   ```bash
   cd fe
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Run the development server:
   ```bash
   npm run dev
   ```

The frontend will start on `http://localhost:5173`

## API Endpoints

### Health Check
- `GET /health` - Basic health check
- `GET /api/v1/health` - API health check
- `GET /api/v1/db/health` - Database health check

## Development

### Backend
- The backend uses Axum for routing and handling HTTP requests
- CORS is configured to allow frontend development
- Logging is configured via tracing
- ArcadeDB integration using `arcadedb-rs` client library
- Database connection pooling with async/await support

### Frontend
- Built with SvelteKit and TypeScript
- Configured for development with hot module replacement

## License

MIT
