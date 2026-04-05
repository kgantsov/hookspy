# HookSpy

A real-time webhook testing and inspection tool built with Rust. HookSpy provides unique URLs that capture and display incoming HTTP requests in real-time.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Real-time Webhook Inspection** - View incoming webhook requests instantly via WebSocket connections
- **Unique Webhook URLs** - Generate unique endpoints for each webhook you want to test
- **Request History** - Store and review all webhook requests with full headers and body
- **OAuth Authentication** - Secure access with OAuth 2.0 integration
- **High Performance** - Built with Rust and Axum
- **Modern UI** - Responsive frontend built with Yew (Rust WebAssembly)
- **Single Binary Deployment** - Frontend assets are embedded into the backend binary
- **LibSQL Database** - Lightweight, embedded database for data persistence

## Architecture

HookSpy is a full-stack Rust application:

- **Backend**: Axum-based REST API with WebSocket support
- **Frontend**: Yew-based WebAssembly SPA
- **Database**: LibSQL for data persistence

For detailed architecture documentation, see [ARCHITECTURE.md](./docs/ARCHITECTURE.md).

## Quick Start

### Prerequisites

- Rust (1.70+)
- Trunk (for frontend development): `cargo install trunk`

### Environment Variables

Create a `.env` file or set the following environment variables:

```env
OAUTH_CLIENT_ID=your_oauth_client_id
OAUTH_CLIENT_SECRET=your_oauth_client_secret
OAUTH_AUTH_URL=https://your-auth-provider.com/oauth/authorize
OAUTH_TOKEN_URL=https://your-auth-provider.com/oauth/token
OAUTH_REDIRECT_URL=http://localhost:3000/api/auth/callback

JWT_SECRET=your-secret-key-minimum-32-characters
```

### Development

Start the frontend dev server with hot reload:

```sh
make start_frontend
```

In a separate terminal, start the backend:

```sh
make start_backend
```

Open your browser at `http://localhost:8080`.

### Production Build

```sh
make build
```

This will:
1. Build the frontend with Trunk (optimized release build)
2. Embed the frontend assets into the backend binary
3. Build the backend with Cargo (release mode)

The final binary will be at `backend/target/release/hookspy`.

### Running the Binary

```sh
./backend/target/release/hookspy \
  --address 0.0.0.0:3000 \
  --domain https://your-domain.com \
  --database-path /path/to/hookspy.db
```

| Flag | Short | Description | Default |
|------|-------|-------------|---------|
| `--address` | `-a` | Address to listen on | `0.0.0.0:3000` |
| `--domain` | `-d` | Domain of the webhook server | `http://0.0.0.0:3000` |
| `--database-path` | `-p` | Path to the database file | `hookspy.db` |

## Usage

### Creating a Webhook

1. Log in using OAuth
2. Click "Create Webhook" and provide a name
3. Copy the generated unique URL
4. Use this URL as the webhook endpoint in your external service

### Inspecting Requests

1. Navigate to your webhook from the sidebar
2. View real-time requests as they arrive
3. Click on any request to see full headers, body, timestamp, and method

### Deleting Webhooks

Select a webhook and click the delete button. All associated requests will also be deleted.

## API Endpoints

### Webhooks

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/webhooks` | Create a new webhook |
| `GET` | `/api/webhooks` | List all webhooks for the authenticated user |
| `GET` | `/api/webhooks/:webhook_id` | Get webhook details |
| `DELETE` | `/api/webhooks/:webhook_id` | Delete a webhook |
| `POST` | `/api/webhooks/:webhook_id` | Receive a webhook payload (public) |
| `GET` | `/api/webhooks/:webhook_id/requests` | Get all requests for a webhook |

### Authentication

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/auth/login` | Initiate OAuth login flow |
| `GET` | `/api/auth/callback` | OAuth callback endpoint |

### WebSocket

| Path | Description |
|------|-------------|
| `/ws/webhooks/:webhook_id/notifications` | Real-time notifications for a webhook |
| `/ws/user/notifications` | Real-time notifications for the authenticated user |

## Project Structure

```
hookspy/
├── backend/              # Rust backend (Axum)
│   ├── src/
│   │   ├── auth/        # JWT and OAuth authentication
│   │   ├── dao/         # Data Access Objects
│   │   ├── handlers/    # HTTP request handlers
│   │   ├── model/       # Data models
│   │   ├── notification/# WebSocket notification system
│   │   └── schema/      # Request/Response schemas
│   └── Cargo.toml
├── frontend/            # Rust frontend (Yew + WASM)
│   ├── src/
│   │   ├── components/  # Reusable UI components
│   │   ├── hooks/       # Custom Yew hooks
│   │   └── pages/       # Page components
│   └── Cargo.toml
├── docs/                # Documentation
├── Makefile             # Build automation
└── README.md
```

## Technologies

### Backend
- **Axum** - Web framework
- **LibSQL** - Embedded database
- **Tokio** - Async runtime
- **Serde** - Serialization/deserialization
- **OAuth2** - OAuth 2.0 client
- **JWT** - JSON Web Token handling

### Frontend
- **Yew** - Rust framework for WebAssembly
- **Yew Router** - Client-side routing
- **Gloo** - Web APIs for Wasm
- **Web-sys** - Web platform bindings

## Running Tests

```sh
# Backend tests
cd backend && cargo test

# Frontend tests
cd frontend && cargo test
```

## Documentation

- [Architecture Overview](./docs/ARCHITECTURE.md)
- [API Documentation](./docs/API.md)
- [Database Schema](./docs/DATABASE.md)
- [Deployment Guide](./docs/DEPLOYMENT.md)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
