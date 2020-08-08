use std::fs::File;
use std::io::{self, Write};
fn main() {
    write_to_file();
}

fn write_to_file() {
    // 1. get sentence from ternimal
    io::stdout()
        .write_all(b"Please input your sentence:\n")
        .expect("Write tips error");
    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("Some Error when read sentence from terminal");
    println!("Got sentence from terminal: {}", sentence);
    // 2. write to file
    // 2.1 supply a file path
    io::stdout()
        .write("Please supply a file path:".as_bytes())
        .expect("Write tips of file path failed");
    // write is just place data to buf and wait to write, so there nedd to flush it
    io::stdout().flush().expect("Flush data");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Get file path failed");
    let file_path = file_path.trim();
    let mut f = File::create(file_path).expect("Create file failed");
    f.write_all(sentence.as_bytes())
        .expect("Write to file failed");
}
