use clap::Parser;
use std::fs::File;
use std::io::Read;
use thiserror::Error;

#[derive(Parser,Debug)]
#[command(version, about, long_about=None)]

struct Args{
    file_name: Vec<String>,
}

#[derive(Debug, Error)]
enum MyError{
    #[error("This is an error: {0}")]
    FileNotFound(String),
}


fn main()->Result<(), MyError> {

    let args=Args::parse();
    let length=args.file_name.len();
    for num in 0..length{
        let file=&args.file_name[num];
        let data: Result<File, _> = File::open(file);
        let mut data = if let Ok(x) = data{
            x
        }
        else{
            return Err(MyError::FileNotFound(format!("File '{}' not found in the current directory",file)));
        };
        let mut content= String::new();
        let _=data.read_to_string(&mut content);
        println!("{}",content);
    }
    Ok(())
}
