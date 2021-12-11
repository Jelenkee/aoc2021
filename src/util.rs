use std::fs;

pub fn read_file(filename: &str) -> Vec<String> {
    let content: String = fs::read_to_string(format!("txt/{}", filename))
        .expect("Error");
    let vec = content
        .as_str()
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| String::from(s))
        .collect::<Vec<String>>();
    return vec;
}