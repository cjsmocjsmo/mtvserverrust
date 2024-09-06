# Stage 1: Build the Rust project
FROM rust:latest AS builder

# Set the working directory
WORKDIR /usr/src/mtvserverrust

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Copy the .env file
COPY .env ./

# Build the project
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:buster-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/local/bin

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/mtvserverrust/target/release/mtvserverrust .

# Copy the .env file
COPY --from=builder /usr/src/mtvserverrust/.env ./

# Expose the port
EXPOSE 8080

# Run the server
CMD ["./mtvserverrust"]