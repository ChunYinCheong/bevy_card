use bevy::prelude::Entity;
use bevy_inspector_egui::Inspectable;

mod ability;
mod card;
mod player;
mod position;
mod selectable;
mod unit;

pub use ability::Ability;
pub use ability::AbilityDataId;
pub use ability::AbilityEffect;
pub use ability::AbilityInstance;
pub use ability::AbilityType;
pub use ability::EffectTarget;
pub use ability::OnBattleEvent;
pub use ability::TriggerCondition;
pub use ability::UnitInstanceRef;
pub use ability::UnitVar;
pub use card::Card;
pub use card::CardData;
pub use card::CardDataId;
pub use card::CardDataType;
pub use card::CardInstance;
pub use card::CardInstanceType;
pub use card::CardType;
pub use player::Player;
pub use player::PlayerController;
pub use player::PlayerInstance;
pub use position::Position;
pub use position::PositionType;
pub use selectable::SelectType;
pub use selectable::Selectable;
pub use unit::Status;
pub use unit::Unit;
pub use unit::UnitData;
pub use unit::UnitDataId;
pub use unit::UnitInstance;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspectable)]
pub struct CardName(pub Entity);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspectable)]
pub struct HpText(pub Entity);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspectable)]
pub struct ApText(pub Entity);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspectable)]
pub struct AtkText(pub Entity);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspectable)]
pub struct DefText(pub Entity);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspectable)]
pub struct CardSprite(pub Entity);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspectable)]
pub struct HighlightSprite(pub Entity);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspectable)]
pub struct Cell;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspectable)]
pub struct Board;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Inspectable)]
pub struct PlayerInstanceRef(pub Entity);
