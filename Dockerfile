FROM rust:latest

# Install dependencies
RUN apt-get update && apt-get install -y libssl-dev

# Copy the code into the container
COPY . /app

# Build the server
WORKDIR /app
RUN cargo build --release

EXPOSE 5000

# Run the server
CMD ["/app/target/release/websockets-server"]