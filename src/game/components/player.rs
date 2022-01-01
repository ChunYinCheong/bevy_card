use bevy::prelude::Entity;
use bevy_inspector_egui::Inspectable;

// pub struct PlayerData{
//     pub name: String,
// }

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub money: i32,
    pub decks: Vec<Entity>,
    pub cards: Vec<Entity>,
    pub character_card: Entity,
    // pub exp: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspectable)]
pub enum PlayerController {
    Player,
    Ai,
}

#[derive(Debug, Clone)]
pub struct PlayerInstance {
    pub name: String,
    pub mp: i32,
    // pub hands: Vec<Entity>,
    // pub decks: Vec<Entity>,
    // pub graves: Vec<Entity>,
    // pub boards: Vec<Vec<Vec<Entity>>>,

    // pub units: Vec<Vec<Option<Entity>>>,
    // pub player_unit: Entity,
    pub player_card_instance_id: Entity,
}
