# Email Newsletter Project - Zero to Production in Rust

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Postgres](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white)
![Actix](https://img.shields.io/badge/actix-%23CC0000.svg?style=for-the-badge)

This project is an implementation of an email newsletter service built while following the book ["Zero to Production in Rust"](https://www.zero2prod.com/) by Luca Palmieri. It's a complete backend service for managing newsletter subscriptions and sending emails.

## Project Journey

I built this project by systematically working through each chapter of the book, implementing concepts as they were introduced:

1. **Initial Setup**: Started with project scaffolding and basic health checks
2. **Subscription Handling**: Built endpoints to manage newsletter subscriptions
3. **Database Integration**: Added PostgreSQL for persistent storage
4. **Email Delivery**: Implemented email sending functionality
5. **Error Handling**: Added robust error management
6. **Testing**: Wrote integration and unit tests
7. **Deployment**: Configured for Docker deployment

Each chapter helped me understand not just the "how" but the "why" behind architectural decisions in production-ready Rust services.

## Technologies Used

### Core Stack
- **Rust** - Primary programming language
- **Actix Web** - Web framework for building the API
- **PostgreSQL** - Database for persistent storage
- **SQLx** - Async database toolkit for Rust
- **Serde** - Serialization/deserialization framework

### Email Functionality
- **reqwest** - HTTP client for email API integration
- **lettre** - Email sending library (alternative implementation)

### Configuration & Security
- **dotenv** - Environment variable management
- **secrecy** - Secret handling utilities
- **argon2** - Password hashing algorithm

### Testing & Development
- **cargo-edit** - Dependency management
- **sqlx-cli** - Database migration tool
- **tracing** - Application logging
- **wiremock** - HTTP mock server for testing

## Key Learnings

Through building this project, I've gained:

1. **Rust Web Development**:
   - Building REST APIs with Actix Web
   - Middleware implementation
   - Request/response handling

2. **Database Operations**:
   - SQLx for type-safe SQL queries
   - Database migrations
   - Connection pooling

3. **Error Handling**:
   - Custom error types
   - Error propagation
   - User-friendly error responses

4. **Testing**:
   - Integration testing patterns
   - Test isolation techniques
   - Mocking external services

5. **Production Considerations**:
   - Configuration management
   - Telemetry and logging
   - Security best practices
   - Deployment strategies

6. **Asynchronous Programming**:
   - Working with async/await
   - Future handling
   - Task spawning

## Getting Started

### Prerequisites
- Rust (latest stable version)
- PostgreSQL
- sqlx-cli (`cargo install sqlx-cli`)

### Installation
1. Clone the repository
2. Set up your `.env` file based on `.env.example`
3. Run database migrations: `sqlx migrate run`
4. Start the server: `cargo run`

## Project Structure
