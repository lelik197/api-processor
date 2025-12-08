pub fn calculate(a: f64, b: f64, op: &str) -> f64 {
    match op {
        "add" => a + b,
        "subtract" => a - b,
        "multiply" => a * b,
        "divide" => if b != 0.0 { a / b } else { 0.0 },
        _ => 0.0,
    }
}