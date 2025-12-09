pub fn calculate(deviceCount: u64, devicePrice: f64, serversCount: u64, serverPrice: f64, op: &str) -> f64 {
    match op {
        "calculate" => {
            (deviceCount as f64) * devicePrice + (serversCount as f64) * serverPrice
        },
        _ => 0.0,
    }
}

// pub fn calculate(a: f64, b: f64, op: &str) -> f64 {
//     match op {
//         "add" => a + b,
//         "subtract" => a - b,
//         "multiply" => a * b,
//         "divide" => if b != 0.0 { a / b } else { 0.0 },
//         _ => 0.0,
//     }
// }