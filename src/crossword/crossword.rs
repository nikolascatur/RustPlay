use std::collections::HashMap;
use std::io;

use rand::seq::index;

use crate::crossword::{collection::CollectionWord, word::Word};
use crate::menu::Menu;
use crate::util::helper::{self, clear_terminal};

#[derive(Clone)]
pub struct Crossword {
    size_dec: i32,
    questions: Option<CollectionWord>,
    map: Option<HashMap<i32, (char, i32)>>,
    answered_map: [Option<char>; 64],
}

impl Crossword {
    pub fn new(crossword: CollectionWord) -> Self {
        Self {
            size_dec: 8,
            questions: Some(crossword),
            map: None,
            answered_map: [None; 64],
        }
    }

    pub fn load_data(&mut self) {
        if let Some(n) = self.questions.clone() {
            self.arr_to_map(&n);
        }

        let count_min: i32 = 1;
        let map_borrow = self.map.clone();
        if let Some(ref value) = map_borrow {
            for (index, (ch, num)) in value {
                println!("{},{},{}", index, ch, num);
                if *num > count_min {
                    let to_usize = *index as usize;
                    self.fill_answer(to_usize, *ch);
                }
            }
        }
    }

    fn fill_answer(&mut self, index: usize, letter: char) {
        if index <= self.answered_map.len() {
            self.answered_map[index] = Some(letter);
            println!("{:?}", self.answered_map[index])
        }
    }

    fn arr_to_map(&mut self, crossword: &CollectionWord) {
        let mut map_tmp: HashMap<i32, (char, i32)> = HashMap::new();
        for word in &crossword.words {
            let text = &word.word;
            let chars: Vec<char> = text.chars().collect();
            for (i, index) in word.arr_index.iter().enumerate() {
                if let Some(&n) = chars.get(i) {
                    match map_tmp.get_mut(index) {
                        Some((_existing_char, count)) => {
                            *count += 1;
                        }
                        None => {
                            map_tmp.insert(*index, (n, 1));
                        }
                    }
                }
            }
        }
        self.map = Some(map_tmp);
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
                let index = (&i * self.size_dec) + &j;
                let char = self.answered_map[index as usize];
                if let Some(n) = char {
                    print!("[{}]", n)
                } else {
                    print!("   ")
                }
            }
            println!()
        }
    }
}
