mod level;
mod lib;

use crate::level::ExpositRoom;
use crate::level::Room;

use termion::color;

fn main() {
    // Game Introduction
    lib::loading_screen();
    lib::pause_text(1500);
    lib::game_opening();

    // Init Living Room Data
    let living_room_exits = Vec::new();
    let living_room_description: String = "\nThe room appears sparse and forgotten. \
                                           A thin layer of dust lies on almost every \
                                           surface you lay eyes on.\n"
        .to_string();

    // Instantiate Room
    let living_room = Room {
        description: living_room_description,
        exits: living_room_exits,
    };

    living_room.describe_room();

    // Ask For Input
    let _first_move = lib::take_user_input().to_string();
    println!(
        "\n{}You {}! Game over!\n",
        color::Fg(color::Red),
        _first_move
    );
}

// Impl of Trait ExpositRoom
// for Struct Room
impl ExpositRoom for Room {
    fn describe_room(&self) {
        println!("{}{}", color::Fg(color::Blue), self.description);
    }
}
