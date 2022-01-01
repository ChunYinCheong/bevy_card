use bevy::math::Vec2;
use bevy_inspector_egui::Inspectable;

#[derive(Debug, Clone, Inspectable)]
pub struct Selectable {
    pub size: Vec2,
    pub select_type: SelectType,
}

#[derive(Debug, Clone, Inspectable)]
pub enum SelectType {
    Cell,
    Card,
}
