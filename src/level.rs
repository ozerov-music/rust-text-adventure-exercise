pub struct Room {
    pub description: String,
    pub exits: Vec<String>,
}

pub trait ExpositRoom {
    fn describe_room(&self);
}
