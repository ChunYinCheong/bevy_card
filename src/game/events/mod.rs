mod battle_event;

use bevy::prelude::Entity;

use super::components::Position;

#[derive(Debug, Clone, Default)]
pub struct HoverChangedEvent {
    pub old: Option<Entity>,
    pub new: Option<Entity>,
}

#[derive(Debug)]
pub struct PlayerActionEvent {
    pub player_id: Entity,
    pub action: PlayerAction,
}
#[derive(Debug)]
pub enum PlayerAction {
    EndTurn,
    UseAbility {
        ability: Entity,
        source: Entity,
        target: Entity,
    },
    Attack {
        source: Entity,
        target: Entity,
    },
    SummonUnit {
        card: Entity,
        position: Position,
    },
    // Use Item
    // Use Magic
}

// target: unit/ cell/ card?/player?
// avaiable target?

#[derive(Debug)]
pub enum InputMappingEvent {
    EndTurn,
    ClickCell(Entity),
    ClickCard(Entity),
    // ClickAbility(Entity),
    Cancel,
}

#[derive(Debug)]
pub struct SelectEntityEvent(pub Option<Entity>);

#[derive(Debug)]
pub struct SelectionChangedEvent(pub Option<Entity>, pub Option<Entity>);

pub use battle_event::AbilityStart;
pub use battle_event::BattleEvent;
pub use battle_event::PlayerChange;
pub use battle_event::PlayerDraw;
pub use battle_event::PlayerEndTurn;
pub use battle_event::PreTurnStart;
pub use battle_event::SummonUnit;
pub use battle_event::TurnEnd;
pub use battle_event::TurnStart;
pub use battle_event::UnitCombat;
pub use battle_event::UnitDie;
pub use battle_event::UnitHurt;
pub use battle_event::UnitStartAbility;
