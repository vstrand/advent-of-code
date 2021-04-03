use std::fs;

pub fn read_data() -> Vec<String> {
    let data = fs::read_to_string("data.txt")
        .expect("Unable to read data!");

    let rows: Vec<String> = data
        .split("\n")
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    rows
}
