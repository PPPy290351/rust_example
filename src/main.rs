use std::io;

fn main() {
    
    println!("--- start the game ---");

    guess_the_number();
}

fn guess_the_number() {
    println!("Please input your guess:");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}