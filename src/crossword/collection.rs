use crate::crossword::word::Word;

#[derive(Clone)]
pub struct CollectionWord {
    words: Vec<Word>,
}

impl CollectionWord {
    pub fn new(words: &[Word]) -> Self {
        Self {
            words: words.to_vec(),
        }
    }
}
