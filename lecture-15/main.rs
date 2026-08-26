fn print_name(name: &String) {
    println!("Name: {}", name);
}

fn add_suffix(name: &mut String) {
    name.push_str(" App");
}

fn main() {
    let name = String::from("FileFlow");
    print_name(&name);
    println!("Still usable: {}", name);

    let mut mutable_name = String::from("FileFlow");
    add_suffix(&mut mutable_name);
    println!("{}", mutable_name);
}
