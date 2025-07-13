#!/bin/bash

# Build and run script for Dioxus fullstack Docker application

# Source .env file if it exists
if [ -f .env ]; then
    source .env
fi

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_step() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
check_directory() {
    if [ ! -d "cicd" ] || [ ! -f "cicd/Dockerfile.debian" ]; then
        print_error "This script must be run from the project root directory"
        print_error "Make sure you're in the directory containing the 'cicd' folder"
        exit 1
    fi
}

# Function to build the production image
build_production() {
    check_directory
    #!/bin/bash

    if [[ -z "$GITHUB_TOKEN" ]] || [[ -z "$DATABASE_URL" ]]; then
        echo "Error: GITHUB_TOKEN and DATABASE_URL environment variables are not set." >&2
        exit 1
    fi

    print_step "Building production Docker image..."

    if
        docker build -f cicd/Dockerfile.debian \
        -t dioxus-web:latest \
        --secret type=env,id=GITHUB_TOKEN,env=GITHUB_TOKEN \
        --secret type=env,id=DATABASE_URL,env=DATABASE_URL  \
        --progress=plain .;
    then
        print_step "Production image built successfully!"
    else
        print_error "Failed to build production image"
        exit 1
    fi
}

# Function to run the production container
run_production() {
    print_step "Running production container..."
    if docker run -d \
        --name dioxus-web \
        -p 8080:8080 \
        -e PORT=8080 \
        -e IP=0.0.0.0 \
        --rm \
        dioxus-web:latest; then
        print_step "Production container is running at http://localhost:8080"
    else
        print_error "Failed to run production container"
        exit 1
    fi
}

# Function to stop containers
stop_containers() {
    print_step "Stopping containers..."
    docker stop dioxus-web 2>/dev/null || true
    cd cicd && docker-compose down 2>/dev/null || true
    print_step "Containers stopped."
}

# Function to clean up
clean() {
    print_step "Cleaning up Docker images and containers..."
    docker stop dioxus-web 2>/dev/null || true
    docker rm dioxus-web 2>/dev/null || true
    docker rmi dioxus-web:latest 2>/dev/null || true
    cd cicd && docker-compose down --rmi all 2>/dev/null || true
    print_step "Cleanup completed."
}

# Main script logic
case "${1:-}" in
"build")
    build_production
    ;;
"run")
    build_production
    run_production
    ;;
"stop")
    stop_containers
    ;;
"clean")
    clean
    ;;
"logs")
    docker logs -f dioxus-web 2>/dev/null || (cd cicd && docker-compose logs -f web-dev)
    ;;
*)
    echo "Usage: $0 {build|run|dev|stop|clean|logs}"
    echo ""
    echo "Commands:"
    echo "  build  - Build the production Docker image"
    echo "  run    - Build and run the production container"
    echo "  stop   - Stop all running containers"
    echo "  clean  - Remove all containers and images"
    echo "  logs   - Show container logs"
    echo ""
    echo "Note: Run this script from the project root directory"
    exit 1
    ;;
esac
