FROM alpine:3.15

WORKDIR /app

# Copy the Rust binary from the host into the container
COPY ./server /app/rust_server

# Set the binary as executable
RUN chmod +x rust_server

# Expose the port that the Rust server listens on
EXPOSE 8000

# Start the Rust server
CMD ["./rust_server"]

