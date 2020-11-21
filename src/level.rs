// Room Data Structure
pub struct Room<'a> {
    pub description: &'a str,
    pub points_of_interest: &'a str,
    pub exits: Vec<String>,
}

// Room Traits
pub trait ExpositRoom {
    fn describe_room(&self);
    fn describe_poi(&self);
    fn describe_exits(&self);
}
