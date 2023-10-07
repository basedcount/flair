# Use an official Rust runtime as a parent image
FROM rust:1.70.0 as builder

# Set the working directory in the builder stage
WORKDIR /usr/src/flair

# Copy the current directory contents into the container at /usr/src/flair
COPY . .

# Install any needed packages specified in Cargo.toml
RUN cargo install --path .

# Start a new stage from scratch
FROM debian:bullseye-slim

# Install libssl, sqlite3 and clean up apt cache to keep the image small
RUN apt-get update && apt-get install -y libssl1.1 libsqlite3-0 libc6 && rm -rf /var/lib/apt/lists/*

# Set the working directory in the container to /usr/local/bin
WORKDIR /usr/local/bin

# Copy the binary from builder to this new stage
COPY --from=builder /usr/local/cargo/bin/flair .

# Make port 6969 available to the world outside this container
EXPOSE 6969

# Define environment variable RUST_LOG to control the output of logs by the application
ENV RUST_LOG=info

# Run the binary when the container launches
CMD ["./flair", "serve"]