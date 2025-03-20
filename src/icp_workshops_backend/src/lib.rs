#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn calculate(a: i32, b: i32, operator: String) -> String {
    let result = match operator.as_str() {
        "+" => Some(a + b),
        "-" => Some(a - b),
        "*" => Some(a * b),  // poprawka dla mnożenia
        "/" => if b != 0 { Some(a / b) } else { None },
        _ => None, // dodanie domyślnej obsługi dla nieznanych operatorów
    };

    match result {
        Some(value) => format!("Result: {}", value),
        None => "Invalid operator or division by zero".to_string(),
    }
}
