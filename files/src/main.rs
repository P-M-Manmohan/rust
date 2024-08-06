// use std::fs::File;
use std::fs::OpenOptions;
// use std::io::Read;
use std::io::Write;
// use std::fs;

fn main() {
    // let mut data = File::open("data.txt").unwrap();
    // let mut content= String::new();
    // data.read_to_string(&mut content).unwrap();
    // println!("{:?}",content);
    // let mut data_file=File::create("first.txt").expect("error creating file");
    // data_file.write("hello world".as_bytes()).expect("error while writing");
    // fs::remove_file("data.txt").expect("error while removing file");

    let mut data_append=OpenOptions::new()
    .append(true)
    .open("first.txt")
    .expect("error opeing file");

    data_append
        .write("Learning to manipulate files in rust".as_bytes())
        .expect("Error appending to file");

}
