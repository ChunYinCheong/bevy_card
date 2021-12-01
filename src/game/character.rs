use super::Ability;

#[derive(Clone)]
pub struct Character {
    pub name: String,
    pub description: String,
    pub hp: i8,
    pub attack: i8,
    pub defence: i8,
    pub action_point: i8,
    pub abilities: Vec<Ability>,
    pub status: Status,
    pub is_dead: bool,
}

#[derive(Default, Clone)]
pub struct Status {
    /// Cannot do anything, awake when taking damage
    pub sleep: i8,
    /// Cannot do anything
    pub stun: i8,
    /// Control by enemy
    pub charm: i8,
    /// Lose control, random action
    pub fear: i8,
    /// Mad
    pub curse: i8,
}
