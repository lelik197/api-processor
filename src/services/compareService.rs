pub fn compare(a: f64, b: f64) -> String {
    if a > b {
        format!("{} больше {}", a, b)
    } else if b > a {
        format!("{} больше {}", b, a)
    } else {
        "Числа равны".to_string()
    }
}