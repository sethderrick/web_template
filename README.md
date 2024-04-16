# Rust Web Server Template for AI Service

This repository contains a template for creating a web server in Rust, specifically designed to be integrated with an AI service. The server is built using Actix-web, a powerful, pragmatic, and extremely fast web framework for Rust.

## Features

- CORS support: The server is configured to allow requests from `http://localhost` and to support various HTTP methods like `GET`, `POST`, `PUT`, `DELETE`.
- Route handling: The server is set up with routes for creating, reading, updating, and deleting tasks.
- Actix-web: The server uses Actix-web, which is a simple, pragmatic and extremely fast web framework for Rust.

## Getting Started

1. Clone the repository
2. Navigate to the project directory
3. Run `cargo build` to build the project
4. Run `cargo run` to start the server

The server will start running at `http://127.0.0.1:8080`.

## Routes

- `POST /task`: Create a new task
- `GET /task`: Read all tasks
- `PUT /task`: Update a task
- `GET /task/{id}`: Read a specific task by ID
- `DELETE /task/{id}`: Delete a specific task by ID

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)