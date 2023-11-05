# Stage 1: Build the Rust application
FROM rust:latest AS build

WORKDIR /usr/src/app
COPY . .

# Build the release version of the Rust application
RUN cargo build --release

# Stage 2: Create a minimal runtime image
FROM debian:bullseye-slim

WORKDIR /usr/src/app

# Copy the compiled binary from the build stage
COPY --from=build /usr/src/app/target/release/your_app_name .  # Replace "your_app_name" with your actual binary name

# Command to run your Rust application
CMD ["./your_app_name"]  # Replace "your_app_name" with your actual binary name
