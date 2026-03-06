use::std::io;
fn main() {
println!("guess the number");
println!("please guess the input ");

let mut  guess =String::new();

io::stdin()
.read_line(&mut guess)
.expect("Failed to read line");

println!("you guessed the {guess}")
}
