mod level;
mod lib;
mod logic;

use crate::level::ExpositRoom;
use crate::level::Room;
use crate::logic::ManageState;
use crate::logic::PlayerState;

use termion::color;

fn main() {
    // The current player state
    // 0 = Unconsious
    // 1 = Anxious
    // 2 = Confident
    // 3 = Neutral
    let mut current_state;

    // A verbal representation of the character's state of mind
    let mut feeling_condition: &str;

    // Game Introduction
    lib::loading_screen();
    lib::pause_text(1500);
    lib::game_opening();

    // Init Living Room Data
    let living_room_exits = Vec::new();
    let living_room_description = "\nThe room appears sparse and forgotten. \
                                           A thin layer of dust lies on almost every \
                                           surface you lay eyes on. There is an underlying \
                                           familiarity to the room you can't quite put your \
                                           finger on.\n";
    let living_room_poi = "There appears to be a large wooden cabinet in the corner, \
                                  a mantlepiece lined with old photographs by the fireplace, \
                                  and a doorway to the East, past the worn armchair.\n";

    // Instantiate Room
    let living_room = Room {
        description: &living_room_description,
        points_of_interest: &living_room_poi,
        exits: living_room_exits,
    };

    // Instantiate PlayerState
    let player_state = PlayerState {
        anxiety_or_confidence: 5u8,
    };

    // Describe the room & points of interest
    living_room.describe_room();
    living_room.describe_poi();

    // Ask For Input from Player
    let _first_move = lib::take_user_input().to_string();
    println!(
        "\n{}You {}! Game over!\n",
        color::Fg(color::Red),
        _first_move
    );

    // Calculate Player State, Get Feeling Descriptor, Print Test Descriptor
    current_state = player_state.update_state();
    feeling_condition = player_state.current_feeling(current_state);
    println!("{}{}", color::Fg(color::Yellow), feeling_condition);
}

// Impl of Trait ExpositRoom For Struct Room
impl<'a> ExpositRoom for Room<'_> {
    fn describe_room(&self) {
        println!("{}{}", color::Fg(color::Blue), self.description);
    }

    fn describe_poi(&self) {
        println!("{}{}", color::Fg(color::Blue), self.points_of_interest);
    }
}

// Impl Trait Manage State for Struct PlayerState
impl ManageState for PlayerState {
    fn update_state(&self) -> u8 {
        let current_state: u8;

        if self.anxiety_or_confidence == 5 {
            current_state = 3u8;
            return current_state;
        } else if self.anxiety_or_confidence == 10 {
            current_state = 0u8;
            return current_state;
        } else if self.anxiety_or_confidence > 5 {
            current_state = 2u8;
            return current_state;
        } else {
            current_state = 1u8;
            return current_state;
        }
    }

    fn current_feeling(&self, player_state: u8) -> &str {
        let feelings: &str;

        if player_state == 3u8 {
            feelings = "My head is a little dizzy, but I feel alright.";
            return &feelings;
        } else {
            feelings = "I don't know how I feel ...\nI should consult my developer.";
            return &feelings;
        }
    }
}
