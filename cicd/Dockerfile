FROM nixos/nix:2.18.3 AS builder

# Update Nix channels and enable experimental features for flakes
RUN nix-channel --update
RUN echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf

# Set the working directory for the application
WORKDIR /app

# Copy the entire source code (the flake needs access to the source directory)
COPY . .

# Build the application using the Nix flake
RUN nix build

# Create a minimal runtime image
FROM nixos/nix:2.18.3

# Enable flakes in the runtime image
RUN echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf

# Copy the built application from the builder stage
COPY --from=builder /app/result /app/result

# Set up environment variables
ENV DATABASE_URL="sqlite:/app/result/share/dioxus-web/main.db"
ENV RUST_LOG="info"

# Expose port 8080
EXPOSE 8080

# Set the working directory to where the assets are located
WORKDIR /app/result/share/dioxus-web

# Run the server binary
CMD ["/app/result/bin/dioxus-web"]
