use bevy::prelude::Entity;
use bevy_inspector_egui::Inspectable;

use super::UnitDataId;

#[derive(Debug, Clone, Inspectable)]
pub enum CardDataId {
    Solider,
    Spare,
    You,
    Demon,
    Enemy,
}

impl CardDataId {
    pub fn card_data(&self) -> CardData {
        match self {
            CardDataId::Solider => CardData {
                id: CardDataId::Solider,
                name: "Soldier".into(),
                cost: 1,
                card_type: CardDataType::Unit(UnitDataId::Solider),
            },
            CardDataId::Spare => CardData {
                id: CardDataId::Spare,
                name: "Spare".into(),
                cost: 1,
                card_type: CardDataType::Unit(UnitDataId::Spare),
            },
            CardDataId::You => CardData {
                id: CardDataId::You,
                name: "You".into(),
                cost: 1,
                card_type: CardDataType::Unit(UnitDataId::You),
            },
            CardDataId::Demon => CardData {
                id: CardDataId::Demon,
                name: "Demon".into(),
                cost: 1,
                card_type: CardDataType::Unit(UnitDataId::Demon),
            },
            CardDataId::Enemy => CardData {
                id: CardDataId::Enemy,
                name: "Enemy".into(),
                cost: 1,
                card_type: CardDataType::Unit(UnitDataId::Enemy),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct CardData {
    pub id: CardDataId,
    pub name: String,
    pub cost: i32,
    pub card_type: CardDataType,
}

#[derive(Debug, Clone)]
pub enum CardDataType {
    Unit(UnitDataId),
    // Magic(MagicData),
    // Item(ItemData),
}

// #[derive(Debug, Default, Clone)]
// pub struct MagicData {}

// #[derive(Debug, Clone, Default)]
// pub struct ItemData {}

#[derive(Debug, Clone)]
pub struct Card {
    // Card Data
    pub name: String,
    pub cost: i32,
    pub card_type: CardType,
    // Card
    pub card_data_id: CardDataId,
}

impl Card {
    pub fn new(card_data_id: CardDataId, card_type: CardType) -> Self {
        let card_data = card_data_id.card_data();
        Self {
            card_data_id,
            name: card_data.name,
            cost: card_data.cost,
            card_type,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CardType {
    Unit(Entity),
    // Magic(MagicData),
    // Item(ItemData),
}

#[derive(Debug, Clone)]
pub struct CardInstance {
    // Card Data
    pub name: String,
    pub cost: i32,
    pub card_type: CardInstanceType,
    // Card
    pub card_data_id: CardDataId,
    // Battle
    pub card_id: Entity,
}

impl CardInstance {
    pub fn new(card_id: Entity, card: Card, card_type: CardInstanceType) -> Self {
        Self {
            name: card.name,
            cost: card.cost,
            card_type,
            card_data_id: card.card_data_id,
            card_id,
        }
    }
}

#[derive(Debug, Clone)]
pub enum CardInstanceType {
    UnitInstance(Entity),
    // Magic(MagicData),
    // Item(ItemData),
}
