mod ability;
mod character;
mod player;

pub use ability::Ability;
pub use ability::AbilityType;
pub use ability::EffectEvent;
pub use ability::EffectTarget;
pub use ability::TriggerEvent;
pub use character::Character;
pub use player::Player;

pub use player::ENEMY_INDEX;
pub use player::PLAYER_INDEX;

pub use bevy::ecs::entity::Entity;

pub struct Game {
    pub players: Vec<Entity>,
    pub current_index: usize,
}

#[derive(Clone)]
pub struct Owner {
    pub player_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub player_index: usize,
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn xyz(&self) -> (f32, f32, f32) {
        let offset: i32 = 100;
        (
            ((self.x as i32 - 1) * offset) as f32,
            (((self.y as i32 + 1) * offset - offset / 2)
                * if self.player_index == PLAYER_INDEX {
                    -1
                } else {
                    1
                }) as f32,
            1.0,
        )
    }
}

pub struct Field;
