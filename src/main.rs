mod crossword;
mod guess;
mod menu;
mod util;

fn main() {
    // let mut game = guess::GuestNumber::new();
    // game.run_game_guest();
    let mut menu = menu::MenuUI::new();
    menu.show_menu();
}
