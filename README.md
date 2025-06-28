# My Vercel API

A serverless API built with Rust and deployed on Vercel, demonstrating modern serverless architecture with high performance and minimal cold start times.

## Features

- **Fast & Efficient**: Built with Rust for optimal performance
- **Serverless**: Deployed on Vercel with automatic scaling
- **JSON API**: RESTful endpoints returning JSON responses
- **Cross-platform**: Works across different regions with global deployment

## Tech Stack

- **Runtime**: Rust 2021 Edition
- **Framework**: Vercel Runtime for Rust
- **Dependencies**:
  - `tokio` - Async runtime
  - `serde_json` - JSON serialization
  - `vercel_runtime` - Vercel integration

## API Endpoints

### GET /api/handler

Returns a simple JSON greeting message.

**Response:**

```json
{
  "message": "你好，世界"
}
```

## Development

### Prerequisites

- Rust (2021 edition or later)
- Vercel CLI (optional, for local development)

### Local Development

```bash
# Clone the repository
git clone <repository-url>
cd my-vercel-api

# Build the project
cargo build

# Run locally (requires Vercel CLI)
vercel dev
```

### Project Structure

```plaintext
.
├── api/
│   └── handler.rs          # API handler function
├── Cargo.toml              # Rust project configuration
├── vercel.json             # Vercel deployment configuration
└── README.md               # Project documentation
```

## Deployment

This project is configured for automatic deployment on Vercel:

1. Connect your repository to Vercel
2. Vercel will automatically detect the Rust runtime
3. Deploy with zero configuration required

### Manual Deployment

```bash
# Install Vercel CLI
npm i -g vercel

# Deploy
vercel --prod
```

## Configuration

The project uses `vercel.json` to configure the Rust runtime:

- Runtime: `vercel-rust@4.0.9`
- Functions: All `.rs` files in the `api/` directory

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test locally
5. Submit a pull request

## License

This project is open source and available under the [MIT License](LICENSE).
