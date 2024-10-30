use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    let mut secret_number = rand::thread_rng();
    let secret_number: u32 = secret_number.gen_range(1..100);

    let mut chances: u8 = 10;

    loop {
        if chances == 0 {
            break;
        }

        let mut user_input = String::new();
        let _ = io::stdin().read_line(&mut user_input);

        let user_input : u32 = user_input.trim().parse()
            .expect("Please type a number! under 255");

        match user_input.cmp(&secret_number) {
            Ordering::Less => println!("Too small! chances remaining {}", chances - 1),
            Ordering::Greater => println!("Too big! chances remaining {}", chances - 1),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        };

        chances -= 1;
    }
}
