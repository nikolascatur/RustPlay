mod guess;
fn main() {
    let mut game = guess::GuestNumber::new();
    game.run_game_guest();
}
