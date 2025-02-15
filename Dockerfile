# Builder Stage
FROM rust:latest AS builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release

# Final Stage
FROM ubuntu:22.04
RUN apt-get update && apt-get install -y openssl libssl-dev
COPY --from=builder /usr/src/myapp/target/release/newbicycle_backend /usr/local/bin/newbicycle_backend
CMD ["newbicycle_backend"]
