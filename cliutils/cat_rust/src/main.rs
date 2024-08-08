use std::env;
use std::fs::File;
use std::io::Read;

// #[derive(Parser,Debug)]
// #[command(version, about, long_about=None)]

// struct Args{
//     file_name: String,
// }


fn main() {

    let args: Vec<String>=env::args().collect();
    let length=args.len();
    for num in 1..length{
        let file=&args[num];
        let mut data = File::open(file).unwrap();
        let mut content= String::new();
        data.read_to_string(&mut content).unwrap();
        print!("{}",content);
    }
}
