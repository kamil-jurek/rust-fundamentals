use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

fn read_from_file(filename: &str) {
    let file = File::open(filename);
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            std::io::ErrorKind::PermissionDenied => {
                panic!("Permission denied: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}

fn write_to_file(filename: &str, content: &str) {
    // Attempt to create or open the file
    let mut file = match File::create(filename) {
        Ok(file) => file, // If successful, return the file handle
        Err(e) => {
            println!("Error creating or opening the file: {}", e);
            return;
        }
    };

    // Attempt to write to the file
    match file.write_all(content.as_bytes()) {
        Ok(_) => println!("Successfully wrote to the file."),
        Err(e) => println!("Error writing to the file: {}", e),
    }
}

fn append_to_file(filename: &str, content: &str) {
    // Attempt to open the file in append mode
    let mut file = match OpenOptions::new()
        .write(true)
        .append(true)
        .open(filename) {
            Ok(file) => file,
            Err(e) => {
                println!("Error opening the file: {}", e);
                return;
            }
        };

    // Attempt to write to the file
    match file.write_all(content.as_bytes()) {
        Ok(_) => println!("Successfully appended to the file."),
        Err(e) => println!("Error writing to the file: {}", e),
    }
}

fn main() {
    let filename = "output.txt";
    
    write_to_file(filename, "First Hello, world!");
    append_to_file(filename, " Second Hello, world! ");
    append_to_file(filename, " Third Hello, world! ");
    read_from_file(filename);
}
