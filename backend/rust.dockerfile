# Build stage
FROM rust:1.79-buster as builder

WORKDIR /app

# Copy the source code
COPY . .

# Build the Rust application
RUN cargo build --release

# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/backend .

# Run the application
CMD [ "./backend" ]
