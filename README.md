# Rust Order Service

A robust order management service built with Rust and Axum.

## Features

- User authentication and authorization
- JWT-based authentication
- Admin and user roles
- Secure password hashing
- RESTful API endpoints

## Getting Started

1. Clone the repository
2. Install dependencies
3. Run the service

## API Endpoints

- `/api/auth/register` - Register a new user
- `/api/auth/login` - Login and get JWT token
- `/api/admin/users` - Admin-only endpoint to list users

## Technologies Used

- Rust
- Axum web framework
- SQLx for database operations
- JWT for authentication
- Bcrypt for password hashing