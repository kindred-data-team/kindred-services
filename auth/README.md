# Kindred Microservice

This microservice is built with Rust, using the actix-web framework for handling HTTP requests and it connects to a Postgresql database using Diesel.

## Features

- Actix-Web Framework: A powerful web framework for building scalable APIs.
- Database Integration: Uses Diesel ORM for interacting with a Postgresql database.
- Middleware Support: Includes custom middleware for logging and error handling.
- Organized Architecture: Clean separation of concerns across modules.

## Project Structure
```
├── migrations/
├── src/ 
│   ├── api/   
│   ├── config/  
│   ├── db/
│   ├── helper/  
│   ├── middleware/  
│   ├── models/  
│   ├── repository/  
│   ├── routes/  
│   └── main.rs 
│   └── schema.rs 
├── Cargo.toml 
└── diesel.toml 
```
- `api/`: Includes API-related logic and handlers.
- `config/`: Manages application configuration.
- `db/`: Contains database connection and setup logic.
- `helper/`: Contains helper functions for specific tasks (password hashing, request validation, etc).
- `middleware/`: Custom middleware for logging.
- `models/`: Contains data models for the application.
- `repository/`: Handles database interactions or data storage.
- `routes/`: Defines the API routes (currently including vaccine-related routes).
- `main.rs`: Entry point that starts the HTTP server and sets up routes.
- `schema.rs`: Contains database table schema.
- `Cargo.toml`: Contains the list of dependencies used by the application.
- `diesel.toml`: Config file used by diesel.

## Requirements

- Rust 1.70 or higher
- Diesel 2.2 or higher

## Getting Started

### Clone the Repository

```bash
https://github.com/kindred-data-team/kindred-services.git
cd kindred-services/auth
```

### Install Dependencies

Make sure you have Rust installed on your system. If not, follow the installation guide.

Run the following command to install the project dependencies:
```
cargo build
```

### Setup Diesel ORM

Run the following command to setup diesel and run migration:
```
diesel setup
```

### Run the Project

Start the server with the following command:
```
cd .. (you need to be in the root folder kindred-services)
cargo run -p auth
```
This will start the server on http://127.0.0.1:8080.
