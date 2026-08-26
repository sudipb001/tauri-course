fn greet_user(name: String) {
    println!("Hello, {}!", name);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    greet_user(String::from("Tauri Developer"));
    let total = add_numbers(5, 7);
    println!("Total: {}", total);
}
