# resdataRust

## Overview

This project is an open-source, API built in **Rust** using the **Actix Web** framework. The API is designed for efficient data handling. The use case is real-time data processing for high-throughput lasers

### Features
- **Actix Web**: Built using the Actix Web framework, optimized for speed and scalability.
- **Async API**: Asynchronous request handling using **Tokio** runtime.
- **JSON Handling**: Efficient serialization and deserialization using **Serde**.
- **Authentication**: JWT-based authentication for secure access to API endpoints.
- **Cross-Origin Resource Sharing (CORS)**: Configurable CORS support for secure cross-origin requests.
- **Database Integration**: Supports PostgreSQL or MongoDB for persistent storage.

## Getting Started

### Environment Configuration

Create a `.env` file in the project root to configure environment variables, such as database credentials:

```
MONGO_URI=<mongo uri>
JWT_SECRET=your_secret_key
```

### Testing

To run the tests, use the following command:

```bash
cargo test
```

This project uses Rust's built-in testing framework.

## Contact

For any questions, feel free to reach out:

- Email: [tomaszneska523@gmail.com](mailto:tomaszneska523@gmail.com)
- GitHub: [@SplitSky](https://github.com/SplitSky)

