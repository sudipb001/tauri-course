fn find_file(name: &str) -> Option<String> {
    if name == "report.csv" {
        Some(String::from("report.csv"))
    } else {
        None
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let found = find_file("report.csv");
    match found {
        Some(file_name) => println!("Found file: {}", file_name),
        None => println!("File not found"),
    }

    let missing = find_file("invoice.csv");
    match missing {
        Some(file_name) => println!("Found file: {}", file_name),
        None => println!("File not found"),
    }

    let division_result = divide(10.0, 2.0);
    match division_result {
        Ok(value) => println!("Result: {}", value),
        Err(message) => println!("Error: {}", message),
    }

    let failed_division = divide(10.0, 0.0);
    match failed_division {
        Ok(value) => println!("Result: {}", value),
        Err(message) => println!("Error: {}", message),
    }
}
