# # Build stage
# FROM rust:1.84.0
# # Set the working directory
# WORKDIR /app
# # Copy the project files from your machine to the container
# COPY . .
# # Build your application for release, inside the container
# RUN cargo build --release
# EXPOSE 8081/tcp
# # Run the binary built inside the container
# CMD ["./target/release/kindred-auth-service"]


FROM rust:1.84.0 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev libpq5 && apt-get clean
WORKDIR /app
COPY --from=builder /app/target/release/ .
#export this actix web service to port 8080 and 0.0.0.0
EXPOSE 8080/tcp
# Run the binary built inside the container
CMD ["./auth"]