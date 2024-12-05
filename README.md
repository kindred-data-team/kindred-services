# Kindred Microservice

This microservice is built with Rust, using the actix-web framework for handling HTTP requests and tokio for asynchronous runtime. It connects to a MySQL database using sqlx.

## Features

- Actix-Web Framework: A powerful web framework for building scalable APIs.
- Database Integration: Uses sqlx for interacting with a MySQL database.
- Middleware Support: Includes custom middleware for logging and error handling.
- Organized Architecture: Clean separation of concerns across modules.

## Project Structure
```
├── src/ 
│   ├── models/   
│   ├── api/  
│   ├── routes/  
│   ├── repository/
│   ├── middleware/  
│   ├── db/  
│   ├── config/  
│   └── main.rs 
└── Cargo.toml 
```
- `models/`: Contains data models for the application.
- `api/`: Includes API-related logic and handlers.
- `routes/`: Defines the API routes (currently including vaccine-related routes).
- `repository/`: Handles database interactions or data storage.
- `db/`: Contains database connection and setup logic.
- `config/`: Manages application configuration.
- `config/`: Custom middleware for logging.
- `main.rs`: Entry point that starts the HTTP server and sets up routes.

## Requirements

- Rust 1.70 or higher
- Dependencies:
  - `actix-web`: Web framework for Rust.
  - `tokio`: Asynchronous runtime.
  - `sqlx`: Async database toolkit with compile-time query checking.

## Getting Started

### Clone the Repository

```bash
https://github.com/kindred-data-team/kindred-services.git
cd kindred-services
```

### Install Dependencies

Make sure you have Rust installed on your system. If not, follow the installation guide.

Run the following command to install the project dependencies:
```
cargo build
```

### Run the Project

Start the server with the following command:
```
cargo run
```
This will start the server on http://127.0.0.1:8080.
