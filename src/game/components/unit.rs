use bevy::prelude::Entity;
use bevy_inspector_egui::Inspectable;

use super::ability::AbilityDataId;

#[derive(Debug, Clone, Inspectable)]
pub enum UnitDataId {
    Solider,
    Spare,
    You,
    Demon,
    Enemy,
}

impl UnitDataId {
    pub fn unit_data(&self) -> UnitData {
        match self {
            UnitDataId::Solider => UnitData {
                id: UnitDataId::Solider,
                name: "Soldier".into(),
                description: "A Soldier".into(),
                hp: 10,
                atk: 2,
                def: 0,
                abilities: vec![AbilityDataId::Attack],
            },
            UnitDataId::Spare => UnitData {
                id: UnitDataId::Spare,
                name: "Spare".into(),
                description: "Some spare card".into(),
                hp: 0,
                atk: 0,
                def: 0,
                abilities: vec![AbilityDataId::Attack],
            },
            UnitDataId::You => UnitData {
                id: UnitDataId::You,
                name: "You".into(),
                description: "You, the player".into(),
                hp: 10,
                atk: 5,
                def: 0,
                abilities: vec![AbilityDataId::Attack],
            },
            UnitDataId::Demon => UnitData {
                id: UnitDataId::Demon,
                name: "Demon".into(),
                description: "A Demon".into(),
                hp: 10,
                atk: 2,
                def: 0,
                abilities: vec![AbilityDataId::Attack],
            },
            UnitDataId::Enemy => UnitData {
                id: UnitDataId::Enemy,
                name: "Enemy".into(),
                description: "Your Enemy!".into(),
                hp: 10,
                atk: 2,
                def: 0,
                abilities: vec![AbilityDataId::Attack],
            },
        }
    }
}

#[derive(Debug, Clone, Inspectable)]
pub struct UnitData {
    pub id: UnitDataId,
    pub name: String,
    pub description: String,
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub abilities: Vec<AbilityDataId>,
}

#[derive(Debug, Clone)]
pub struct Unit {
    // Unit Data
    pub name: String,
    pub description: String,
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub abilities: Vec<Entity>,
    // Unit
    pub unit_data_id: UnitDataId,
    // pub exp: i32,
    // pub level: i32,
    // pub dead_count: i32,
}

impl Unit {
    pub fn new(unit_data_id: UnitDataId, abilities: Vec<Entity>) -> Self {
        let data = unit_data_id.unit_data();
        Self {
            name: data.name,
            description: data.description,
            hp: data.hp,
            atk: data.atk,
            def: data.def,
            abilities,
            unit_data_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnitInstance {
    // Unit Data
    pub name: String,
    pub description: String,
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub abilities: Vec<Entity>,
    // Unit
    pub unit_data_id: UnitDataId,
    // pub exp: i32,
    // pub level: i32,
    // pub dead_count: i32,
    // Battle
    pub unit_id: Entity,
    pub ap: i32,
    pub states: Status,
    // pub is_dead: bool,
    pub owner: Entity,
}

impl UnitInstance {
    pub fn new(
        unit_id: Entity,
        unit: Unit,
        player_instance_id: Entity,
        abilities: Vec<Entity>,
    ) -> Self {
        Self {
            name: unit.name,
            description: unit.description,
            hp: unit.hp,
            atk: unit.atk,
            def: unit.def,
            abilities,
            unit_data_id: unit.unit_data_id,
            // Battle
            unit_id,
            ap: 0,
            states: Default::default(),
            owner: player_instance_id,
        }
    }
}

#[derive(Debug, Default, Clone)]
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
