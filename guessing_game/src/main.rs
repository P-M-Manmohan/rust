use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::thread_rng();

    let ran: u32 = rng.gen_range(0,11);

    let mut guesses:u32=0;
    while guesses < 3{
        let mut input = Default::default();
        println!("guess a number between 0 and 10 -> ");
        io::stdin()
        .read_line(&mut input)
        .expect("Error reading message");
        let input :u32= input.trim().parse().expect("Invalid value");
        if input==ran{
            println!("Your guessed right congrats!!");
            break;
        }else if input > ran{
            println!("lower");
        }else{
            println!("higher");
        }
        guesses+=1;
    }
}