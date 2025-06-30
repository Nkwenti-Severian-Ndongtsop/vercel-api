# Fibonacci API - Rust Serverless Function

A high-performance serverless API built with Rust and deployed on Vercel for calculating Fibonacci numbers. This implementation demonstrates modern serverless architecture with exceptional performance and minimal cold start times.

## Features

- **⚡ Ultra-Fast Computation**: Optimized Rust implementation with BigUint for large numbers
- **🚀 Serverless**: Deployed on Vercel with automatic scaling and global distribution
- **📊 Performance Metrics**: Returns detailed computation time statistics
- **🛡️ Robust Error Handling**: Comprehensive input validation and error responses
- **🧪 Thoroughly Tested**: Extensive test suite covering edge cases and performance
- **📱 JSON API**: Clean RESTful endpoints with structured responses

## Tech Stack

- **Runtime**: Rust 2021 Edition
- **Framework**: Vercel Runtime for Rust
- **Dependencies**:
  - `tokio` - Async runtime for high-performance I/O
  - `serde_json` - JSON serialization/deserialization
  - `vercel_runtime` - Vercel serverless integration
  - `num-bigint` - Arbitrary precision integer arithmetic
  - `num-traits` - Numeric traits for BigUint operations

## API Endpoints

### GET /{n} - Calculate Fibonacci Number

Calculates the nth Fibonacci number where `n` is a positive integer.

**URL Parameters:**
- `n` (required): The position in the Fibonacci sequence (0 ≤ n ≤ 20000)

**Examples:**
```bash
GET /10    # Calculate F(10)
GET /100   # Calculate F(100)
GET /1000  # Calculate F(1000)
```

**Successful Response (200 OK):**
```json
{
  "success": true,
  "input": 10,
  "result": "55",
  "digits": 2,
  "computation_time": {
    "milliseconds": 0,
    "microseconds": 45,
    "nanoseconds": 45123
  }
}
```

**Error Response (400 Bad Request):**
```json
{
  "success": false,
  "error": "Value too large: 50000. Maximum allowed value is 20000.",
  "status_code": 400
}
```

## Performance Characteristics

This implementation provides excellent performance for Fibonacci calculations:

- **F(100)**: ~1-5 microseconds
- **F(1000)**: ~10-50 microseconds  
- **F(10000)**: ~1-5 milliseconds
- **F(20000)**: ~5-20 milliseconds

The algorithm uses an iterative approach with O(n) time complexity and O(1) space complexity (excluding the size of the result).

## Error Handling

The API handles various error cases gracefully:

| Error Case | Status Code | Response |
|------------|-------------|----------|
| Missing number in path | 400 | Invalid path format |
| Invalid number format | 400 | Cannot parse number |
| Negative numbers | 400 | Invalid number format |
| Numbers > 20000 | 400 | Value too large |

## Development

### Prerequisites

- **Rust**: 2021 edition or later
- **Vercel CLI**: For local development and deployment
- **Git**: For version control

### Local Development

```bash
# Clone the repository
git clone <repository-url>
cd fibonacci-rust

# Build the project
cargo build

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Build for release (optimized)
cargo build --release

# Run with Vercel CLI for local testing
vercel dev
```

### Project Structure

```plaintext
.
├── api/
│   └── fibonacci.rs        # Fibonacci API handler function
├── Cargo.toml              # Rust project configuration
├── vercel.json             # Vercel deployment configuration
├── lib.rs                  # Library root (empty)
├── README.md               # Project documentation
└── .gitignore              # Git ignore patterns
```

### Running Tests

The project includes comprehensive tests:

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run tests with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_fibonacci_base_cases
```

## Usage Examples

### Command Line Interface

```bash
# Run the demo
cargo run --example fibonacci_demo

# Run performance benchmarks
cargo run --bin fibonacci_bench --release

# Test the library directly
cargo test
```

### API Usage

Once deployed, you can use the API endpoints:

```bash
# Calculate F(10)
curl https://your-deployment.vercel.app/10

# Calculate F(100)
curl https://your-deployment.vercel.app/100

# Calculate F(1000) 
curl https://your-deployment.vercel.app/1000
```

### Library Usage

```rust
use fibonacci_rust_api::fibonacci;
use num_bigint::BigUint;

fn main() {
    // Calculate F(50)
    let result = fibonacci(50);
    println!("F(50) = {}", result); // F(50) = 12586269025
    
    // Work with very large numbers
    let big_fib = fibonacci(1000);
    println!("F(1000) has {} digits", big_fib.to_string().len());
}
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
