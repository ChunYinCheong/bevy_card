use bevy::prelude::Entity;

use crate::game::components::Position;

#[derive(Clone, Copy, Debug)]
pub enum BattleEvent {
    PreTurnStart(PreTurnStart),
    TurnStart(TurnStart),
    TurnEnd(TurnEnd),
    PostTurnEnd(PostTurnEnd),
    PlayerChange(PlayerChange),
    PlayerDraw(PlayerDraw),
    PlayerEndTurn(PlayerEndTurn),
    UnitHurt(UnitHurt),
    UnitDie(UnitDie),
    UnitStartAbility(UnitStartAbility),
    UnitCombat(UnitCombat),
    AbilityStart(AbilityStart),
    SummonUnit(SummonUnit),
}

#[derive(Clone, Copy, Debug)]
pub struct PreTurnStart {
    pub player: Entity,
}

#[derive(Clone, Copy, Debug)]
pub struct TurnStart {
    pub player: Entity,
}

#[derive(Clone, Copy, Debug)]
pub struct TurnEnd {
    pub player: Entity,
}
#[derive(Clone, Copy, Debug)]
pub struct PostTurnEnd {
    pub player: Entity,
}

#[derive(Clone, Copy, Debug)]
pub struct PlayerChange {
    pub next_player: Entity,
    pub next_index: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct PlayerDraw {
    pub player: Entity,
    pub c: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct PlayerEndTurn {
    pub player: Entity,
}

#[derive(Clone, Copy, Debug)]
pub struct UnitHurt {
    pub source: Entity,
    pub target: Entity,
    pub value: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct UnitDie {
    pub dead: Entity,
    pub killer: Entity,
}

#[derive(Clone, Copy, Debug)]
pub struct UnitStartAbility {
    pub ability: Entity,
    pub source: Entity,
    pub target: Entity,
}
#[derive(Clone, Copy, Debug)]
pub struct UnitCombat {
    pub source: Entity,
    pub target: Entity,
}

#[derive(Clone, Copy, Debug)]
pub struct AbilityStart {
    pub ability: Entity,
    pub source: Entity,
    pub target: Entity,
}

#[derive(Clone, Copy, Debug)]
pub struct SummonUnit {
    pub card: Entity,
    pub position: Position,
}
