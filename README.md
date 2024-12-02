# Rust API Project

This is a simple RESTful API project built with Rust, using the `axum` web framework for handling HTTP requests and `tokio` for asynchronous runtime.

## Features

- Uses `axum` to define routes and handle HTTP requests.
- Asynchronous server with `tokio` to handle concurrent operations.
- Organized project structure with separate modules for models, routes, repository, and constants.

## Project Structure

- `models/`: Contains data models for the application.
- `api/`: Includes API-related logic and handlers.
- `routes/`: Defines the API routes (currently including vaccine-related routes).
- `repository/`: Handles database interactions or data storage.
- `db/`: Contains database connection and setup logic.
- `constants/`: Stores constant values used throughout the project.
- `main.rs`: Entry point that starts the HTTP server and sets up routes.

## Requirements

- Rust 1.60 or higher
- Dependencies:
  - `axum`: Web framework for Rust.
  - `tokio`: Asynchronous runtime.
  - `axum-server`: For binding and running the server.

## Getting Started

### Clone the Repository

```bash
git clone https://github.com/yourusername/rust-api-project.git
cd rust-api-project
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
This will start the server on http://localhost:8080.
