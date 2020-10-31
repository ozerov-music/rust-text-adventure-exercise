// Room Data Structure
pub struct Room {
    pub description: String,
    pub points_of_interest: String,
    pub exits: Vec<String>,
}

// Room Traits
pub trait ExpositRoom {
    fn describe_room(&self);
    fn describe_poi(&self);
}
