use bevy::math::Vec3;
use bevy_inspector_egui::Inspectable;

use super::Entity;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspectable)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub face_up: bool,
    pub player_id: Entity,
    pub position_type: PositionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspectable)]
pub enum PositionType {
    Deck,
    Hand,
    Board,
    Grave,
}

impl Position {
    pub fn xyz(&self) -> Vec3 {
        let offset = 100.0;
        match &self.position_type {
            PositionType::Deck => Vec3::new(-3.0 * offset, 0.0, (self.z + 1) as f32),
            PositionType::Hand => Vec3::new(
                (self.z - 2) as f32 * offset,
                2.0 * -offset,
                (self.z + 1) as f32,
            ),
            PositionType::Board => Vec3::new(
                (self.x - 1) as f32 * offset,
                (self.y - 1) as f32 * -offset,
                (self.z + 1) as f32,
            ),
            PositionType::Grave => Vec3::new(3.0 * offset, 0.0, (self.z + 1) as f32),
        }
    }
}
