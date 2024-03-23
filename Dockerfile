# Get started with a build env with Rust nightly
FROM ubuntu:latest

RUN apt update
RUN apt install -y ca-certificates

# Copy the server binary to the /app directory
COPY  target/debug/runtime-dispose-repro /app/
# /target/site contains our JS/WASM/CSS, etc.
COPY target/site /app/site
# Copy Cargo.toml if it’s needed at runtime
COPY Cargo.toml /app/
WORKDIR /app


RUN chmod +x runtime-dispose-repro

EXPOSE 3000
# Run the server
CMD ["/app/runtime-dispose-repro"]