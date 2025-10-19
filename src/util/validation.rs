pub fn number_validation(input: &str) -> Option<i32> {
    let number_input = input.trim();
    match number_input.parse::<i32>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}
