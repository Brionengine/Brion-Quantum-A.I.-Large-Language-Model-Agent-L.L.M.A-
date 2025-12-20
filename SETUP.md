# Brion Quantum AI Lab - Complete Setup Guide

This guide will help you set up the complete Brion.qt website with Rust backend, C library, and all optimizations.

## Architecture Overview

```
Brion.qt Website
├── Frontend (HTML/CSS/JS)
├── Rust Backend (Actix-web)
├── C Library (Performance-critical operations)
├── Redis Cache (Optional)
└── Nginx (Reverse proxy, optional)
```

## Quick Start

### Option 1: Docker (Recommended)

```bash
# Start all services
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f backend
```

The website will be available at `http://localhost`

### Option 2: Manual Setup

#### 1. Build C Library

```bash
cd c-lib
make clean
make -j$(nproc)
cd ..
```

#### 2. Start Redis (Optional but Recommended)

```bash
# Using Docker
docker run -d -p 6379:6379 redis:7-alpine

# Or install locally
# Ubuntu/Debian: sudo apt-get install redis-server
# macOS: brew install redis
```

#### 3. Build and Run Rust Backend

```bash
cd backend
cargo build --release
cargo run --release
```

Backend will run on `http://localhost:8080`

#### 4. Serve Frontend

```bash
# Option A: Simple HTTP server
python3 -m http.server 8000

# Option B: Nginx (see nginx.conf)
# Option C: Open index.html directly (limited - CORS issues)
```

## Configuration

### Backend Configuration

Edit `backend/config.toml`:

```toml
[server]
port = 8080
workers = 0  # Auto-detect

[cache]
redis_url = "redis://127.0.0.1:6379/"
ttl_seconds = 3600
```

### Frontend API URL

Edit `scripts/main.js`:

```javascript
const API_BASE_URL = 'http://localhost:8080/api/v1';
// Or for production:
// const API_BASE_URL = 'https://api.brion.qt/api/v1';
```

## Development

### Backend Development

```bash
cd backend

# Run with auto-reload (requires cargo-watch)
cargo install cargo-watch
cargo watch -x "run"

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

### Frontend Development

```bash
# Use a local server to avoid CORS issues
python3 -m http.server 8000

# Or use a proper dev server
npx serve .
```

## Production Deployment

### 1. Build for Production

```bash
# Build C library with optimizations
cd c-lib
make clean
make -j$(nproc)
cd ..

# Build Rust backend
cd backend
cargo build --release
cd ..
```

### 2. Deploy with Docker

```bash
# Build and push images
docker-compose build
docker-compose push  # If using registry

# Deploy
docker-compose up -d
```

### 3. Configure Domain (Brion.qt)

1. Point DNS A record to your server IP
2. Update `nginx.conf` with your domain
3. Set up SSL certificate (Let's Encrypt recommended)
4. Update frontend `API_BASE_URL` to production API

### 4. Environment Variables

Create `.env` file:

```env
API_URL=https://api.brion.qt
REDIS_URL=redis://redis:6379/
RUST_LOG=info
```

## Performance Tuning

### Backend

- Adjust `workers` in `config.toml` (default: auto-detect)
- Tune Redis connection pool
- Enable compression for large responses

### C Library

- Compile with `-march=native` for CPU-specific optimizations
- Adjust `BLOCK_SIZE` in matrix operations for your cache size
- Use profile-guided optimization (PGO) for production

### Caching

- Redis: For distributed caching
- In-memory: For single-instance deployments
- TTL: Adjust based on data update frequency

## Monitoring

### Health Checks

```bash
# Backend health
curl http://localhost:8080/api/v1/health

# Redis
redis-cli ping
```

### Performance Metrics

```bash
# Get stats
curl http://localhost:8080/api/v1/stats
```

## CI/CD

The project includes GitHub Actions workflow that:
- Builds C library
- Tests Rust backend
- Runs security scans
- Deploys on push to main

Configure secrets in GitHub:
- `DEPLOY_HOST` - Deployment server
- `DEPLOY_KEY` - SSH key for deployment

## Troubleshooting

### C Library Not Found

```bash
# Make sure library is in library path
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$(pwd)/c-lib
# Or install it
cd c-lib && sudo make install
```

### Redis Connection Failed

- Check Redis is running: `redis-cli ping`
- Verify connection string in `config.toml`
- Backend will fallback to in-memory cache

### API Not Responding

- Check backend logs: `docker-compose logs backend`
- Verify port 8080 is not in use
- Check firewall rules

### Frontend Can't Connect to API

- Check CORS settings in backend
- Verify API_BASE_URL in frontend
- Use browser dev tools to check network requests

## Security

- Enable HTTPS in production
- Use environment variables for secrets
- Keep dependencies updated
- Run security scans (included in CI/CD)
- Configure rate limiting (in nginx.conf)

## Support

- GitHub: [@Brionengine](https://github.com/Brionengine)
- Website: Brion.qt

## License

© 2024 Brion Quantum AI Lab. All rights reserved.

