# Dev Note:
Cursor auto generated this documentation, I have 

# CI/CD Setup for NicoJudge Website

This directory contains the CI/CD configuration for building and deploying the NicoJudge website Docker container.

## Quick Start

1. **Create environment file**: Create a `.env` file in this directory or the project root:
   ```bash
   # Copy and modify the example
   cp .env.example .env
   # Edit with your actual values
   nano .env
   ```

2. **Build the production image**:
   ```bash
   ./build.sh build
   ```

3. **Run locally**:
   ```bash
   ./build.sh run
   ```

4. **Deploy to DigitalOcean**:
   ```bash
   ./build.sh deploy
   ```

## Environment Variables

Create a `.env` file with the following variables:

```bash
# Required for building
GITHUB_TOKEN=your_github_personal_access_token
DATABASE_URL=sqlite:main.db

# Required for deployment (optional for local builds)
DO_REGISTRY_NAME=your_digitalocean_registry_name
DO_IMAGE_NAME=nicojudgedotcom  # optional, defaults to nicojudgedotcom
```

### Environment Variable Details

- **GITHUB_TOKEN**: Personal access token for GitHub API access
  - Generate at: https://github.com/settings/tokens
  - **Build time**: Required for SQLx query preparation and dependency resolution
  - **Runtime**: Required for GitHub API calls in the web application
  - If not provided at runtime, GitHub-related features will not work

- **DATABASE_URL**: Connection string for database access
  - For SQLite: `sqlite:path/to/database.db`
  - **Build time**: Required for SQLx query preparation
  - **Runtime**: Required for database connections (defaults to `sqlite:main.db`)

- **DO_REGISTRY_NAME**: Your DigitalOcean Container Registry name (for deployment)
- **DO_IMAGE_NAME**: Custom image name (defaults to `nicojudgedotcom`)

## Build Script Commands

The `build.sh` script supports the following commands:

- `build` - Build the production Docker image
- `run` - Run the production container locally
- `deploy` - Build and deploy to DigitalOcean Container Registry
- `logs` - Show container logs
- `cleanup` - Clean up Docker resources
- `help` - Show usage information

## Architecture

### Multi-stage Docker Build

The `Dockerfile.debian.optimized` uses a multi-stage build approach:

1. **Builder Stage** (`rust:1.81`):
   - Installs build dependencies (pkg-config, libssl-dev, libsqlite3-dev)
   - Installs Dioxus CLI and SQLx CLI
   - Adds wasm32 target for client-side builds
   - Builds Rust dependencies first (for better caching)
   - Compiles the full application
   - Prepares SQLx queries
   - Builds the frontend with Dioxus

2. **Runtime Stage** (`debian:bookworm-slim`):
   - Minimal runtime environment
   - Only includes necessary runtime dependencies
   - Creates non-root user for security
   - Copies only the compiled binaries and assets
   - Exposes port 8080

### Build Process

1. **Environment Validation**: Checks for required files and environment variables
2. **Dependency Caching**: Builds dependencies first for faster subsequent builds
3. **Source Compilation**: Compiles the Rust application with server features
4. **Query Preparation**: Runs `cargo sqlx prepare` to validate database queries
5. **Frontend Build**: Uses Dioxus to build the web frontend
6. **Asset Copying**: Copies only necessary files to the minimal runtime image

## Troubleshooting

### Common Issues

1. **"GITHUB_TOKEN not set"**: Make sure your `.env` file exists and contains the token
2. **"DATABASE_URL not set"**: Ensure the database URL is properly configured in `.env`
3. **SQLx preparation fails**: Verify your database is accessible and the URL is correct
4. **Build fails on dependencies**: Try cleaning Docker cache with `docker system prune`
5. **GitHub API returns 401 in container**: The `GITHUB_TOKEN` environment variable is not available at runtime
   - **Solution**: Ensure your `.env` file is properly sourced before running the container
   - **Check**: Run `echo $GITHUB_TOKEN` to verify the token is available in your shell
   - **Docker run**: The build script automatically passes the token to the container
   - **Docker Compose**: Make sure to run `docker-compose up` from the directory containing `.env`

### Build Arguments vs Secrets

The system now uses Docker build arguments instead of BuildKit secrets:
- Build arguments are passed via `--build-arg` during `docker build`
- Environment variables are set in the builder stage using `ARG` and `ENV`
- This approach is simpler and more reliable for CI/CD environments

### Security Considerations

- Environment variables are only used during build time
- The final runtime image does not contain build-time secrets
- The application runs as a non-root user
- Only necessary runtime dependencies are included

## Development Workflow

1. Make changes to your Rust code in the `source/` directory
2. Test locally: `./build.sh build && ./build.sh run`
3. Check logs: `./build.sh logs`
4. Deploy when ready: `./build.sh deploy`
5. Clean up: `./build.sh cleanup`

## DigitalOcean Deployment

The deploy command will:
1. Build the production image
2. Authenticate with DO Container Registry
3. Tag the image with git commit hash (or timestamp)
4. Push both versioned and latest tags
5. Provide instructions for App Platform or Kubernetes deployment

After deployment, you can use the image in:
- DigitalOcean App Platform
- Kubernetes clusters
- Any Docker-compatible hosting service 