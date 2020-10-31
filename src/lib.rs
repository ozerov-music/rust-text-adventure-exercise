use ferris_says::say;
use std::io::{stdout, BufWriter};
use std::thread;
use std::time::Duration;
use termion::color;
use text_io::read;

// Ferris Appearance ASCII Art
pub fn loading_screen() {
    // Setup Ferris Says ASCII Art
    let stdout = stdout();
    let message = String::from("Welcome to the game.");
    let width = message.chars().count();

    // Write ASCII Art
    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

// Trigger Opening
pub fn game_opening() {
    // Game Exposition
    println!(
        "\n\n\n{blue}You open your eyes, the rough strands of the carpet pressing in against your face.{reset}",
        blue = color::Fg(color::Blue),
        reset = color::Fg(color::Reset)
    );

    println!(
        "\n{blue}You have a {red}splitting {blue}headache.{reset}",
        red = color::Fg(color::Red),
        blue = color::Fg(color::Blue),
        reset = color::Fg(color::Reset)
    );
}

// Pause Text Print
pub fn pause_text(wait_length: u64) {
    thread::sleep(Duration::from_millis(wait_length));
}

// Get User Input
pub fn take_user_input() -> String {
    println!(
        "{green}What do you do?{reset}\n",
        green = color::Fg(color::Green),
        reset = color::Fg(color::Reset)
    );
    let line: String = read!("{}\n");
    return line;
}
