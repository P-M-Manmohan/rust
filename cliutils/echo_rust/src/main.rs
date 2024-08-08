use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    word: String,
}

fn main() {
    let args= Args::parse();
    println!("{}",args.word);
}