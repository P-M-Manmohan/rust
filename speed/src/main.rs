use std::io;

fn main() {
    let mut speed=Default::default();
    println!("Enter the speed of the vehicle -> ");
    io::stdin()
    .read_line(&mut speed)
    .expect("Failed to read line");
    let speed: u32 = speed.trim().parse().expect("Invalid input");
    if speed > 90 {
        let points: u32=(speed-90)/5;
        if points > 5 {
            println!("You're dead to us")
        }else{
        println!("You are overspeeding. current points -> {points}");
        }
    }else{
        println!("Good job following the rules");
    }
}
