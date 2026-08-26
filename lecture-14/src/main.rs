use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct FileInfo {
    name: String,
    size_in_kb: f64,
}

fn main() {
    let report = FileInfo {
        name: String::from("report.csv"),
        size_in_kb: 245.5,
    };

    let json = serde_json::to_string(&report).unwrap();
    println!("Serialized: {}", json);

    let parsed: FileInfo = serde_json::from_str(&json).unwrap();
    println!("Deserialized: {:?}", parsed);
}
