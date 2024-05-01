pub mod guessing_game {
    use std::io;
    pub fn guessing_game() {
        println!("Guess the Number");
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        print!("You guessed: {}", guess);
    }
}