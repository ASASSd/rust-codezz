use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let sec_num = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Now lets guess some numbers idk...\nPlease input your shit:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("E: Failed to read line!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Dude, are fuckin kiddin me? type a number...");
                continue;
            }
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&sec_num) {
            Ordering::Less => println!("Too small dude. try again"),
            Ordering::Equal => {
                println!("Yep! that's right shit! cool nigga");
                break;
            }
            Ordering::Greater => println!("Man, it's too big. stop it."),
        }
    }
}