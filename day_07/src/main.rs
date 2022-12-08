use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DirectoryName(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
struct File {
    name: String,
    size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Directory {
    parent: Option<Box<Directory>>,
    name: DirectoryName,
    files: Option<Vec<File>>,
    children: Option<Vec<Directory>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Hierachy {
    head: Directory,
}


fn main() {
    let contents = std::fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut root = Hierachy {
        head: Directory { parent: None, name: DirectoryName("/".to_string()), files: None, children: None }
    };

    for line in contents.lines() {
        if line.starts_with("$") {
            if line
        }
    }
}
