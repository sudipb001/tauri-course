#[derive(Debug)]
struct FileInfo {
    name: String,
    size_in_kb: f64,
}

fn create_file_info(name: String, size_in_kb: f64) -> FileInfo {
    FileInfo { name, size_in_kb }
}

fn main() {
    let report = create_file_info(String::from("report.csv"), 245.5);
    println!("File name: {}", report.name);
    println!("File size: {} KB", report.size_in_kb);
    println!("{:?}", report);
}
