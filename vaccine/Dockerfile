FROM rust:1.84.0 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev && apt-get clean
WORKDIR /app
COPY --from=builder /app/target/release/ .
#export this actix web service to port 8080 and 0.0.0.0
EXPOSE 8081/tcp
# Run the binary built inside the container
CMD ["./vaccine"]