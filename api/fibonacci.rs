use serde_json::json;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};
use num_bigint::BigUint;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    // Parse the request path to get the Fibonacci number
    let path = req.uri().path();
    let query = req.uri().query().unwrap_or("");
    
    println!("Full URI: {}", req.uri());
    println!("Path: {}", path);
    println!("Query: {}", query);
    
    // Try multiple ways to extract the number
    let n: u64 = extract_fibonacci_number(path);
    
    println!("Extracted number: {}", n);
    
    // Limit to prevent excessive computation
    let n = n.min(1000);
    
    let fibonacci_result = calculate_fibonacci(n);
    
    let response_body = json!({
        "fibonacci": fibonacci_result.to_string(),
        "n": n,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "status": "success",
        "debug": {
            "path": path,
            "query": query,
            "full_uri": req.uri().to_string(),
            "extraction_method": "path_analysis"
        },
        "usage": "To calculate Fibonacci of a different number, use: /api/20 (replace 20 with your desired number(integer))"
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
    
    if let Some(last_part) = path.split('/').last() {
        if let Ok(num) = last_part.parse::<u64>() {
            println!("Found number at end of path: {}", num);
            return num;
        }
    }
    
    println!("No number found, using default: 10");
    10 // Default fallback
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