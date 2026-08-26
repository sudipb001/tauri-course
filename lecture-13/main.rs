fn get_file_names() -> Vec<String> {
    let mut file_names: Vec<String> = Vec::new();
    file_names.push("report.csv".to_string());
    file_names.push("invoice.csv".to_string());
    file_names
}

fn main() {
    let file_names = get_file_names();

    println!("File count: {}", file_names.len());

    for file_name in &file_names {
        println!("File: {}", file_name);
    }

    if file_names.is_empty() {
        println!("No files found");
    } else {
        println!("Files were found");
    }

    let first_file = &file_names[0];
    println!("First file: {}", first_file);
}
