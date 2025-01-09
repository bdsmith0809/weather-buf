use std::fs::File;
use std::io::{self, Read, Write};

fn write_string_to_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn read_string_from_file(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {
    let filename = "my_file.txt";
    let content = "Hello, world!";

    write_string_to_file(filename, content)?;
    let read_content = read_string_from_file(filename)?;

    println!("Read from file: {}", read_content);

    Ok(())
}