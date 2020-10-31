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
                                           surface you lay eyes on. There is an underlying \
                                           familiarity to the room you can't quite put your \
                                           finger on.\n"
        .to_string();
    let living_room_poi: String = "There appears to be a large wooden cabinet in the corner, \
                                  a mantlepiece lined with old photographs by the fireplace, \
                                  and a doorway to the East, past the worn armchair.\n"
        .to_string();

    // Instantiate Room
    let living_room = Room {
        description: living_room_description,
        points_of_interest: living_room_poi,
        exits: living_room_exits,
    };

    living_room.describe_room();
    living_room.describe_poi();

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

    fn describe_poi(&self) {
        println!("{}{}", color::Fg(color::Blue), self.points_of_interest);
    }
}
