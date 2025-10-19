use std::io;

use rand::Rng;
use std::cmp::Ordering;

pub struct GuestNumber {
    secret_number: i32,
    answer_number: i32,
}

impl GuestNumber {
    pub fn new() -> Self {
        Self {
            secret_number: 0,
            answer_number: 0,
        }
    }
    fn generate_secret_number(&mut self) {
        let number = rand::rng().random_range(0..=100);
        self.secret_number = number;
    }

    fn input_answer_number(&mut self) -> bool {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        match guess.trim().parse() {
            Ok(num) => {
                self.answer_number = num;
                true
            }
            Err(_) => false,
        }
    }

    fn checker_answer(&self) -> bool {
        match self.answer_number.cmp(&self.secret_number) {
            Ordering::Less => {
                println!("Is To Small");
                false
            }
            Ordering::Equal => {
                println!("Your Correct");
                true
            }
            Ordering::Greater => {
                println!("Is To Big");
                false
            }
        }
    }

    pub fn run_game_guest(&mut self) {
        self.generate_secret_number();
        println!("Input Your Guess Number");
        loop {
            if !self.input_answer_number() {
                continue;
            }
            if self.checker_answer() {
                break;
            }
        }
    }
}
