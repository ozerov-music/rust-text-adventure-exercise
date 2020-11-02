// Data Scruct Representing Player State
pub struct PlayerState {
    // 0-4 Anxious, 5 Neutral, 6-10 Confident
    pub anxiety_or_confidence: u8,
}

// Traits For Managing Player State
pub trait ManageState {
    fn update_state(&self) -> u8;
    fn current_feeling(&self, player_state: u8) -> &str;
    // fn increase_state_score(&self);
    // fn decrease_state_score(&self);
    // fn anxious_response(&self, message: String);
    // fn confident_response(&self, message: String);
    // fn neutral_response(&self, message: String);
}
