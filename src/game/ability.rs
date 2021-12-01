#[derive(Clone)]
pub struct Ability {
    pub name: String,
    pub description: String,
    pub ability_type: AbilityType,
    pub effects: Vec<(EffectTarget, EffectEvent)>,
}

#[derive(Clone)]
pub enum AbilityType {
    Active(TargetSelection),
    Passive(TriggerEvent),
}

#[derive(Clone)]
pub enum TargetSelection {
    None,
    Alliance,
    Enemy,
    Any,
}

#[derive(Clone)]
pub enum TriggerEvent {
    TrunStart,
    TurnEnd,
    OnAttack,
    OnAttacked,
}

#[derive(Clone)]
pub enum EffectTarget {
    Target,
    Source,
    AllEnemy,
    AllAlliance,
    AllEnemyExceptTarget,
    AllAllianceExceptTarget,
}

#[derive(Clone)]
pub enum EffectEvent {
    Heal(i8),
    SoulDrain(i8),
    Reflection,
    Curse(i8),
    Charm(i8),
}
