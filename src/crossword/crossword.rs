use std::io;

use crate::crossword::{collection::CollectionWord, word::Word};
use crate::menu::Menu;
use crate::util::helper::{self, clear_terminal};

#[derive(Clone)]
pub struct Crossword {
    size_dec: i32,
    questions: Option<CollectionWord>,
}

impl Crossword {
    pub fn new(crossword: CollectionWord) -> Self {
        Self {
            size_dec: 8,
            questions: Some(crossword),
        }
    }

    pub fn input(&mut self) -> Option<Menu> {
        helper::clear_terminal();
        self.board();
        println!("Type your guest word");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to readline");

        println!("type {}", user_input == "x");
        if user_input.trim() == "x" {
            return Some(Menu::MENU);
        } else {
            self.input();
            return Some(Menu::NONE);
        }
    }

    fn board(&self) {
        for i in 0..self.size_dec {
            for j in 0..self.size_dec {
                print!("[ ]")
            }
            println!()
        }
    }
}
