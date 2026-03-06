use std::io;
use rand::Rng;

fn main() {
println!("guess the number");
println!("please guess the input ");

let secret_number =rand::thread_rng().gen_range(1..=100);
println!("the secret number is: {secret_number}");

let mut guess =String::new();

io::stdin()
.read_line(&mut guess)
.expect("Failed to read line");

println!("you guessed the {guess}")
}
