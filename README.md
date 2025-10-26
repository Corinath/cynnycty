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
- ArcadeDB
- Clerk account for authentication

### Backend Setup

1. Navigate to the backend directory:
   ```bash
   cd be
   ```

2. Copy the environment variables template:
   ```bash
   cp .env.example .env
   ```

3. Update `.env` with your configuration

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

## Development

### Backend
- The backend uses Axum for routing and handling HTTP requests
- CORS is configured to allow frontend development
- Logging is configured via tracing

### Frontend
- Built with SvelteKit and TypeScript
- Configured for development with hot module replacement

## License

MIT
