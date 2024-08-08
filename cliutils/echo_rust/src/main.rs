use std::env;

// #[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
// struct Args {
//     word: String,
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let word=&args[1];
    println!("{word}");
}