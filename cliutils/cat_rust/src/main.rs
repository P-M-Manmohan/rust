use clap::Parser;
use std::fs::File;
use std::io::Read;

#[derive(Parser,Debug)]
#[command(version, about, long_about=None)]

struct Args{
    file_name: Vec<String>,
}


fn main() {

    let args=Args::parse();
    let length=args.file_name.len();
    for num in 0..length{
        let file=&args.file_name[num];
        let mut data = File::open(file).unwrap();
        let mut content= String::new();
        data.read_to_string(&mut content).unwrap();
        print!("{}",content);
    }
}
