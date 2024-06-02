# Default value for Rust version
ARG RUST_VERSION=1.78.0

# FIRST STAGE : BUILDER

# Use the Rust image with the specified version and slim-bookworm variant as 
# the base image for the builder stage
FROM rust:${RUST_VERSION}-slim-bookworm AS builder

# Set the working directory inside the container to /app
# grosso modo ça te crée un nouveau dossier dans l'image 
# c'est un nom de dossier classique pour mettre ton app dans un container
WORKDIR /app

# Copy all files from the host directory to the container's working directory
COPY . .

# Run the build command with caching for dependencies and build artifacts
# 1 - Cache build artifacts to speed up subsequent builds
# 2 - Cache downloaded dependencies
# 3 - Build the Rust project in release mode
# 4 - Copy the built executable to the root directory
RUN \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \ 
  cargo build --release && \ 
  cp ./target/release/api /

# SECOND STAGE : FINAL

# Use a minimal Debian image as the base image for the final stage
FROM debian:bookworm-slim AS final

# Add a new user named 'appuser' with specified options
# 1 - Disable password login for the user
# 2 - Set empty GECOS fields (user information fields)
# 3 - Set the home directory to a non-existent path
# 4 - Set the shell to /sbin/nologin to prevent the user from logging in
# 5 - Prevent the creation of the home directory
# 6 - Assign a specific user ID (10001) to the new user
# 7 - Name of the user being created
RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \ 
  --no-create-home \ 
  --uid "10001" \
  appuser

# Copy the built executable from the builder stage 
# to /usr/local/bin in the final stage
COPY --from=builder /api /usr/local/bin
# Change ownership of the executable to the new user
RUN chown appuser /usr/local/bin/api
# Copy the configuration directory from the builder 
# stage to /opt/api/config in the final stage
COPY --from=builder /app/config /opt/api/config
# Recursively change ownership of /opt/api to the new user
RUN chown -R appuser /opt/api
# Switch to the new user for subsequent commands and at runtime
USER appuser

# Set an environment variable for logging configuration
ENV RUST_LOG="api=debug,info"
# Set the working directory to /opt/api
WORKDIR /opt/api
# Specify the command to run when the container starts
ENTRYPOINT ["api"]

# Document that the container will listen on port 8080 over TCP
EXPOSE 8080/tcp