#!/bin/bash

# Source .env file if it exists
# this is a bit hacky but check for either cicd/.env or .env path since we may be in either location
if [ -f cicd/.env ]; then
    source .env
fi

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

    # Build the production image using secrets
    docker build \
        --secret id=github_token,src=<(echo "$GITHUB_TOKEN") \
        --secret id=database_url,src=<(echo "$DATABASE_URL") \
        -f cicd/Dockerfile.debian.optimized \
        -t nicojudgedotcom:latest \
        .

    if [ $? -eq 0 ]; then
        print_step "Production image built successfully!"
    else
        print_error "Failed to build production image"
        exit 1
    fi
}

# Function to run the production container
run_production() {
    check_directory
    print_step "Running production container..."

    # Stop and remove existing container if it exists
    docker stop nicojudgedotcom 2>/dev/null || true
    docker rm nicojudgedotcom 2>/dev/null || true

    # Run the production container
    docker run -d \
        --name nicojudgedotcom \
        -p 8080:8080 \
        nicojudgedotcom:latest

    if docker ps | grep -q nicojudgedotcom:latest; then
        print_step "Production container is running!"
    else
        print_error "Failed to start production container"
        exit 1
    fi
}

# Function to clean up Docker resources
cleanup() {
    print_step "Cleaning up Docker resources..."
    docker stop nicojudgedotcom 2>/dev/null || true
    docker rm nicojudgedotcom 2>/dev/null || true
    docker rmi nicojudgedotcom:latest 2>/dev/null || true
    print_step "Cleanup completed!"
}

# Function to deploy to DigitalOcean Container Registry
deploy() {
    check_directory

    # Default values
    REGISTRY_NAME=${DO_REGISTRY_NAME:-""}
    IMAGE_NAME=${DO_IMAGE_NAME:-"nicojudgedotcom"}

    # Validate prerequisites
    if ! command -v doctl &> /dev/null; then
        print_error "doctl is not installed. Please install it first:"
        print_error "  https://docs.digitalocean.com/reference/doctl/how-to/install/"
        exit 1
    fi

    if [ -z "$REGISTRY_NAME" ]; then
        print_error "DO_REGISTRY_NAME environment variable is not set"
        print_error "Please set it to your DigitalOcean Container Registry name"
        exit 1
    fi

    # Check if user is authenticated
    if ! doctl auth list | grep -q "current"; then
        print_error "Not authenticated with doctl. Please run:"
        print_error "  doctl auth init"
        exit 1
    fi

    # Build production image first
    print_step "Building production image for deployment..."
    build_production

    # Authenticate Docker with DO Container Registry
    print_step "Authenticating Docker with DigitalOcean Container Registry..."
    if ! doctl registry login; then
        print_error "Failed to authenticate with DigitalOcean Container Registry"
        exit 1
    fi

    # Generate tag (use git commit hash if available, otherwise timestamp)
    if command -v git &> /dev/null && git rev-parse --git-dir > /dev/null 2>&1; then
        TAG=$(git rev-parse --short HEAD)
        print_step "Using git commit hash as tag: $TAG"
    else
        TAG=$(date +%Y%m%d-%H%M%S)
        print_step "Using timestamp as tag: $TAG"
    fi

    REGISTRY_PATH="registry.digitalocean.com/$REGISTRY_NAME/$IMAGE_NAME"

    # Tag the image
    print_step "Tagging image for registry..."
    if ! docker tag nicojudgedotcom:latest "$REGISTRY_PATH:$TAG"; then
        print_error "Failed to tag image with version tag"
        exit 1
    fi

    if ! docker tag nicojudgedotcom:latest "$REGISTRY_PATH:latest"; then
        print_error "Failed to tag image with latest tag"
        exit 1
    fi

    # Push the images
    print_step "Pushing images to registry..."
    if ! docker push "$REGISTRY_PATH:$TAG"; then
        print_error "Failed to push versioned image"
        exit 1
    fi

    if ! docker push "$REGISTRY_PATH:latest"; then
        print_error "Failed to push latest image"
        exit 1
    fi

    print_step "Deployment completed successfully!"
    print_step "Registry: $REGISTRY_NAME"
    print_step "Image: $IMAGE_NAME"
    print_step "Tag: $TAG"
    print_step ""
    print_step "Image pushed to: $REGISTRY_PATH:$TAG"
    print_step "Image pushed to: $REGISTRY_PATH:latest"
    print_step ""
    print_step "To deploy to DigitalOcean App Platform:"
    print_step "  1. Go to https://cloud.digitalocean.com/apps"
    print_step "  2. Create a new app or edit existing one"
    print_step "  3. Choose 'Container Registry' as source"
    print_step "  4. Select your registry and image"
    print_step ""
    print_step "To deploy to Kubernetes:"
    print_step "kubectl create deployment nicojudgedotcom --image=$REGISTRY_PATH:$TAG"
    print_step "kubectl expose deployment nicojudgedotcom --type=LoadBalancer --port=80 --target-port=8080"
}

# Function to show logs
logs() {
    print_step "Showing container logs..."
    # Try production container first, then development
    docker logs -f nicojudgedotcom 2>/dev/null || (cd cicd && docker-compose logs -f web-dev)
}

# Function to show usage
usage() {
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  build       Build the production Docker image"
    echo "  run         Run the production container"
    echo "  deploy      Deploy to DigitalOcean Container Registry"
    echo "  logs        Show container logs"
    echo "  cleanup     Clean up Docker resources"
    echo "  help        Show this help message"
    echo ""
    echo " Mandatory environment variables:"
    echo "  GITHUB_TOKEN     - GitHub personal access token (required for build)"
    echo "  DATABASE_URL     - Database connection string (required for build)"
    echo "  DO_REGISTRY_NAME - DigitalOcean Container Registry name (required for deploy)"
    echo "  DO_IMAGE_NAME    - Image name (optional, defaults to 'nicojudgedotcom')"
}

# Main script logic
case "${1:-help}" in
    "build")
        build_production
        ;;
    "run")
        run_production
        ;;
    "deploy")
        build_production
        deploy
        ;;
    "logs")
        logs
        ;;
    "cleanup")
        cleanup
        ;;
    "help"|*)
        usage
        ;;
esac
