use crate::{
    crossword::{
        collection::CollectionWord,
        crossword::{self, Crossword},
        word::Word,
    },
    guess::GuestNumber,
    util::{helper, validation},
};
use std::io;

pub enum Menu {
    NONE,
    MENU,
    GUESS,
    CROSSWORD,
    EXIT,
}

pub struct MenuUI {
    input_menu: Menu,
}

impl MenuUI {
    pub const MENU_LIST: [&str; 4] = [
        "Input Your number to Choose Your Menu",
        "1 GUESS Game",
        "2 Crossword",
        "3 EXIT Menu",
    ];
    pub fn new() -> Self {
        Self {
            input_menu: Menu::NONE,
        }
    }

    // fn number_to_menu(&self, number: &i32) -> Menu { // klo mau ngakses pake self.number_to_menu(&n)
    fn number_to_menu(&self, number: &i32) -> Menu {
        match number {
            1 => Menu::GUESS,
            2 => Menu::CROSSWORD,
            3 => Menu::EXIT,
            _ => Menu::NONE,
        }
    }
    pub fn show_menu(&mut self) {
        helper::clear_terminal();
        for menu in MenuUI::MENU_LIST {
            println!("{}", menu)
        }
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

    fn go_guess_game(&mut self) {
        let mut guess = GuestNumber::new();
        let result = guess.run_game_guest();

        if let Some(n) = result {
            match n {
                Menu::MENU => self.show_menu(),
                Menu::EXIT => println!("Bye Bye ....."),
                _ => {}
            }
        }
    }

    fn go_crossword_game(&mut self) {
        let word1 = Word::new(&[0, 1, 2, 3, 4, 5, 6], "PEMILIK");
        let word2 = Word::new(&[6, 14, 22, 30, 38, 46, 54], "KINERJA");
        let word3 = Word::new(&[28, 36, 44, 52, 60], "SERAP");
        let collection = CollectionWord::new(&[word1, word2, word3]);
        let mut crossword = Crossword::new(collection);
        let result = crossword.input();
        if let Some(n) = result {
            match n {
                Menu::MENU => {
                    println!("presss Menu Harusnya",);
                    self.show_menu()
                }
                _ => {}
            }
        }
    }

    fn go_next(&mut self) {
        match self.input_menu {
            Menu::GUESS => self.go_guess_game(),
            Menu::CROSSWORD => self.go_crossword_game(),
            Menu::NONE => self.show_menu(),
            _ => println!("Bye Bye ....... "),
        }
    }
}
