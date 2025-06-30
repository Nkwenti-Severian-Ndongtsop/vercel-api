use serde_json::json;
use vercel_runtime::{Body, Error, Request, Response, StatusCode};
use num_bigint::BigUint;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    vercel_runtime::run(handler).await?;
    Ok(())
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let path = req.uri().path();
    
    let n: u64 = extract_fibonacci_number(path);
    
    let n = n.min(1000);
    
    let fibonacci_result = calculate_fibonacci(n);
    
    let response_body = json!({
        "n": n,
        "fibonacci": fibonacci_result.to_string(),
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type")
        .body(response_body.to_string().into())?)
}

fn extract_fibonacci_number(path: &str) -> u64 {
    let parts: Vec<&str> = path.trim_matches('/').split('/').collect();
    if parts.len() >= 3 && parts[0] == "api" && parts[1] == "fib" {
        if let Ok(num) = parts[2].parse::<u64>() {
            println!("Found number in /api/fib/:number: {}", num);
            return num;
        }
    }
    0
}

fn calculate_fibonacci(n: u64) -> BigUint {
    if n == 0 {
        return BigUint::from(0u32);
    }
    if n == 1 {
        return BigUint::from(1u32);
    }
    
    let mut a = BigUint::from(0u32);
    let mut b = BigUint::from(1u32);
    
    for _ in 2..=n {
        let next = &a + &b;
        a = b;
        b = next;
    }
    
    b
}