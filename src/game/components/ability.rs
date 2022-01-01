use bevy::prelude::Entity;
use bevy_inspector_egui::Inspectable;

use crate::game::events::BattleEvent;

#[derive(Clone, Debug, Copy, PartialEq, Eq, Inspectable)]
pub enum AbilityDataId {
    Attack,
    Regeneration,
    Killer,
    Vampire,
    Phoenix,
}

impl Default for AbilityDataId {
    fn default() -> Self {
        Self::Attack
    }
}

impl AbilityDataId {
    pub fn ability_data(&self) -> AbilityData {
        match self {
            AbilityDataId::Attack => AbilityData {
                id: AbilityDataId::Attack,
                priority: 0,
                name: "Attack".into(),
                description: "Basic Attack".into(),
                ability_type: AbilityType::Active(TargetSelection::Any),
                effects: vec![(EffectTarget::Target, AbilityEffect::Attack)],
                ap: 1,
            },
            AbilityDataId::Regeneration => AbilityData {
                id: AbilityDataId::Regeneration,
                priority: 0,
                name: "Regeneration".into(),
                description: "Regeneration".into(),
                ability_type: AbilityType::Trigger(Trigger {
                    event: OnBattleEvent::OnTurnStart,
                    conditions: Default::default(),
                }),
                effects: vec![(EffectTarget::Target, AbilityEffect::Heal(1))],
                ap: 0,
            },
            AbilityDataId::Killer => AbilityData {
                id: AbilityDataId::Killer,
                priority: 0,
                name: "Killer".into(),
                description: "Killer".into(),
                ability_type: AbilityType::Trigger(Trigger {
                    event: OnBattleEvent::OnUnitDie,
                    conditions: vec![TriggerCondition::UnitEq(
                        UnitVar::ActionUnit,
                        UnitVar::AbilityUnit,
                    )],
                }),
                effects: vec![(EffectTarget::Target, AbilityEffect::Heal(1))],
                ap: 0,
            },
            AbilityDataId::Vampire => AbilityData {
                id: AbilityDataId::Vampire,
                priority: 0,
                name: "Vampire".into(),
                description: "Vampire".into(),
                ability_type: AbilityType::Trigger(Trigger {
                    event: OnBattleEvent::OnUnitDie,
                    conditions: vec![TriggerCondition::UnitEq(
                        UnitVar::ActionUnit,
                        UnitVar::AbilityUnit,
                    )],
                }),
                effects: vec![(EffectTarget::Target, AbilityEffect::Heal(1))],
                ap: 0,
            },
            AbilityDataId::Phoenix => AbilityData {
                id: AbilityDataId::Phoenix,
                priority: 0,
                name: "Phoenix".into(),
                description: "Phoenix".into(),
                ability_type: AbilityType::Trigger(Trigger {
                    event: OnBattleEvent::OnUnitDie,
                    conditions: vec![TriggerCondition::UnitEq(
                        UnitVar::TriggerUnit,
                        UnitVar::AbilityUnit,
                    )],
                }),
                effects: vec![(EffectTarget::Target, AbilityEffect::Heal(1))],
                ap: 0,
            },
        }
    }
}

#[derive(Clone, Debug, Default, Inspectable)]
pub struct AbilityData {
    pub id: AbilityDataId,
    pub priority: i32,
    pub name: String,
    pub description: String,
    pub ability_type: AbilityType,
    pub effects: Vec<(EffectTarget, AbilityEffect)>,
    pub ap: i32,
}

#[derive(Clone, Debug, Default, Inspectable)]
pub struct Ability {
    // Ability Data
    pub priority: i32,
    pub name: String,
    pub description: String,
    pub ability_type: AbilityType,
    pub effects: Vec<(EffectTarget, AbilityEffect)>,
    pub ap: i32,
    // Ability
    pub ability_data_id: AbilityDataId,
}

impl Ability {
    pub fn new(ability_data_id: AbilityDataId) -> Self {
        let data = ability_data_id.ability_data();
        Self {
            priority: data.priority,
            name: data.name,
            description: data.description,
            ability_type: data.ability_type,
            effects: data.effects,
            ap: data.ap,
            ability_data_id,
        }
    }
}

#[derive(Clone, Debug, Inspectable)]
pub struct AbilityInstance {
    // Ability Data
    pub priority: i32,
    pub name: String,
    pub description: String,
    pub ability_type: AbilityType,
    pub effects: Vec<(EffectTarget, AbilityEffect)>,
    pub ap: i32,
    // Ability
    pub ability_data_id: AbilityDataId,
    // Battle
    pub ability_id: Entity,
}

impl AbilityInstance {
    pub fn new(ability_id: Entity, ability: Ability) -> Self {
        Self {
            priority: ability.priority,
            name: ability.name,
            description: ability.description,
            ability_type: ability.ability_type,
            effects: ability.effects,
            ap: ability.ap,
            ability_data_id: ability.ability_data_id,
            ability_id,
        }
    }
}

#[derive(Clone, Debug, Inspectable)]
pub struct UnitInstanceRef(pub Entity);

#[derive(Clone, Debug, Inspectable)]
pub enum AbilityType {
    Active(TargetSelection),
    Trigger(Trigger),
}

impl Default for AbilityType {
    fn default() -> Self {
        Self::Active(TargetSelection::None)
    }
}

#[derive(Clone, Debug, Inspectable)]
pub enum TargetSelection {
    None,
    Alliance,
    Enemy,
    Any,
}

impl Default for TargetSelection {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Debug, Default, Inspectable)]
pub struct Trigger {
    pub event: OnBattleEvent,
    pub conditions: Vec<TriggerCondition>,
}

#[derive(Clone, Debug, Inspectable)]
pub enum OnBattleEvent {
    OnTurnStart,
    OnTurnEnd,
    OnPlayerChange,
    OnPlayerDraw,
    OnUnitHurt,
    OnUnitDie,
    OnAbilityStart,
    // SummonUnit,s
}

impl Default for OnBattleEvent {
    fn default() -> Self {
        Self::OnTurnStart
    }
}

impl OnBattleEvent {
    pub fn match_event(&self, event: &BattleEvent) -> bool {
        match self {
            OnBattleEvent::OnTurnStart => match event {
                BattleEvent::TurnStart(_) => true,
                _ => false,
            },
            OnBattleEvent::OnTurnEnd => match event {
                BattleEvent::TurnEnd(_) => true,
                _ => false,
            },
            OnBattleEvent::OnPlayerChange => match event {
                BattleEvent::PlayerChange(_) => true,
                _ => false,
            },
            OnBattleEvent::OnPlayerDraw => match event {
                BattleEvent::PlayerDraw(_) => true,
                _ => false,
            },
            OnBattleEvent::OnUnitHurt => match event {
                BattleEvent::UnitHurt(_) => true,
                _ => false,
            },
            OnBattleEvent::OnUnitDie => match event {
                BattleEvent::UnitDie(_) => true,
                _ => false,
            },
            OnBattleEvent::OnAbilityStart => match event {
                BattleEvent::AbilityStart(_) => true,
                _ => false,
            },
        }
    }
}

#[derive(Clone, Debug, Inspectable)]
pub enum TriggerCondition {
    UnitEq(UnitVar, UnitVar),
}

impl Default for TriggerCondition {
    fn default() -> Self {
        Self::UnitEq(UnitVar::TriggerUnit, UnitVar::TriggerUnit)
    }
}

#[derive(Clone, Debug, Inspectable)]
pub enum UnitVar {
    TriggerUnit,
    ActionUnit,
    AbilityUnit,
}

impl Default for UnitVar {
    fn default() -> Self {
        Self::TriggerUnit
    }
}

#[derive(Clone, Debug, Inspectable)]
pub enum EffectTarget {
    Target,
    Source,
    AllEnemy,
    AllAlliance,
    AllEnemyExceptTarget,
    AllAllianceExceptTarget,
}

impl Default for EffectTarget {
    fn default() -> Self {
        Self::Target
    }
}

#[derive(Clone, Debug, Inspectable)]
pub enum AbilityEffect {
    Attack,
    Damage(i32),
    Heal(i32),
    SoulDrain(i32),
    Reflection,
    Curse(i32),
    Charm(i32),
}

impl Default for AbilityEffect {
    fn default() -> Self {
        Self::Attack
    }
}

#[derive(Clone, Debug, Default, Inspectable)]
pub struct Damage {
    pub damage_type: DamageType,
    pub damage: i32,
}

#[derive(Clone, Debug, Inspectable)]
pub enum DamageType {
    Attack,
    Spell,
}

impl Default for DamageType {
    fn default() -> Self {
        Self::Attack
    }
}
