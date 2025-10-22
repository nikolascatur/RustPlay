#[derive(Clone)]
pub struct Word {
    arr_index: Vec<i32>,
    word: String,
}

impl Word {
    pub fn new(arr: &[i32], word: &str) -> Self {
        Self {
            arr_index: arr.to_vec(),
            word: word.to_string(),
        }
    }
}
