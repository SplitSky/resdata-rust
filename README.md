# resdataRust

## Overview

This project is an open-source, high-performance API built in **Rust** using the **Actix Web** framework. The API is designed for blazingly fast performance and efficient data handling, making it an ideal backend solution for research applications, real-time data processing, or other high-demand environments.

### Features
- **Actix Web**: Built using the Actix Web framework, optimized for speed and scalability.
- **Async API**: Asynchronous request handling using **Tokio** runtime.
- **JSON Handling**: Efficient serialization and deserialization using **Serde**.
- **Authentication**: JWT-based authentication for secure access to API endpoints.
- **Cross-Origin Resource Sharing (CORS)**: Configurable CORS support for secure cross-origin requests.
- **Database Integration**: Supports PostgreSQL or MongoDB for persistent storage.

## Getting Started

### Prerequisites

To build and run this project, you'll need the following installed:

- **Rust**: You can install Rust using [rustup](https://rustup.rs/):
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Cargo**: Rust's package manager (included with the Rust installation).
- **PostgreSQL or MongoDB**: Choose one for database integration.

### Installing Dependencies

Once you have Rust installed, clone this repository and navigate into the project directory:

```bash
git clone https://github.com/SplitSky/resdataRust.git
cd resdataRust
```

Then install the project dependencies:

```bash
cargo build
```

### Environment Configuration

Create a `.env` file in the project root to configure environment variables, such as database credentials:

```
DATABASE_URL=postgres://user:password@localhost/mydb
JWT_SECRET=your_secret_key
```

### Running the Project

To start the server, simply run:

```bash
cargo run
```

This will launch the API at `http://127.0.0.1:8080` (usually). You can configure the IP and port in the `main.rs` file if needed.

### API Endpoints

*(This section will be updated as the project progresses)*

### Authentication

This project uses **JWT (JSON Web Tokens)** for authentication. After successfully logging in, the API returns a token which should be included in the `Authorization` header for protected routes:

```http
Authorization: Bearer <your_token>
```

### Database Integration

This project supports both **PostgreSQL** and **MongoDB**. You can switch the database by modifying the connection in the `.env` file.

- **PostgreSQL**: Uses `sqlx` for async database queries.
- **MongoDB**: Uses the official MongoDB driver for Rust.

### Testing

To run the tests, use the following command:

```bash
cargo test
```

This project uses Rust's built-in testing framework.

### Formatting and Linting

- **Rustfmt**: The project is formatted using `rustfmt`. To format the codebase, run:
  ```bash
  cargo fmt
  ```
- **Clippy**: To run the linter:
  ```bash
  cargo clippy
  ```

## Contributing

Contributions are welcome! Feel free to submit a pull request or open an issue to report bugs, suggest features, or provide feedback.

### License

This project is licensed under the **GNU General Public License v3.0**. See the [LICENSE](./LICENSE) file for details.

---

## Contact

For any questions, feel free to reach out:

- Email: [tomaszneska523@gmail.com](mailto:tomaszneska523@gmail.com)
- GitHub: [@SplitSky](https://github.com/SplitSky)

