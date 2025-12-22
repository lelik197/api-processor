pub fn calculate(
    deviceCount: u64,
    devicePrice: f64,
    serversCount: u64,
    serverPrice: f64,
    developerCount: u64,
    developerSalary: f64,
    qaCount: u64,
    qaSalary: f64,
    op: &str,
) -> f64 {
    match op {
        "calculate" => {
            (deviceCount as f64) * devicePrice
                + (serversCount as f64) * serverPrice
                + (developerCount as f64) * developerSalary
                + (qaCount as f64) * qaSalary
        }
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