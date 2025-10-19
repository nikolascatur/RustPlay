use crate::{guess::GuestNumber, util::validation};
use std::io;

pub enum Menu {
    NONE,
    GUESS,
    EXIT,
}

pub struct MenuUI {
    input_menu: Menu,
}

impl MenuUI {
    pub fn new() -> Self {
        Self {
            input_menu: Menu::NONE,
        }
    }

    // fn number_to_menu(&self, number: &i32) -> Menu { // klo mau ngakses pake self.number_to_menu(&n)
    fn number_to_menu(&self, number: &i32) -> Menu {
        match number {
            1 => Menu::GUESS,
            2 => Menu::EXIT,
            _ => Menu::NONE,
        }
    }
    pub fn show_menu(&mut self) {
        println!("Input Your number to Choose Your Menu");
        println!("1 GUESS GAME");
        println!("2 Exit Menu");
        let mut select_menu = String::new();
        io::stdin()
            .read_line(&mut select_menu)
            .expect("Failed to read line");
        let validation_input = validation::number_validation(&select_menu);
        if let Some(n) = validation_input {
            let result = self.number_to_menu(&n);
            self.input_menu = result;
            self.go_next();
        } else {
            println!("Wrong Select Menu");
            self.show_menu();
        }
    }

    fn go_guess_game(&self) {
        let mut guess = GuestNumber::new();
        guess.run_game_guest();
    }

    fn go_next(&mut self) {
        match self.input_menu {
            Menu::GUESS => self.go_guess_game(),
            Menu::NONE => self.show_menu(),
            _ => {}
        }
    }
}
