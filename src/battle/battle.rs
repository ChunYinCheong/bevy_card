use crate::{
    animation::{AnimationTracker, AninationRequestEvent},
    game::{
        AbilityType, Character, EffectEvent, EffectTarget, Game, Owner, Position, TriggerEvent,
        ENEMY_INDEX, PLAYER_INDEX,
    },
    ui_animation::{UiAnimationTracker, UiAninationRequestEvent},
};
use bevy::prelude::*;

pub struct BattlePlugin;
impl Plugin for BattlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(EventQuene::default())
            .insert_resource(EventState::Init)
            .add_event::<BattleEvent>()
            .add_system(battle_quene.system())
            .add_event::<StartTurnStatusEvent>()
            .add_system(start_turn_status_system.system())
            .add_event::<AttackEvent>()
            .add_system(attack_system.system())
            .add_event::<DeadEvent>()
            .add_system(dead_system.system())
            .add_event::<AbilityEvent>()
            .add_system(ability_system.system())
            .add_event::<EndTurnEvent>()
            .add_system(end_turn_system.system())
            .add_event::<ChangePlayerEvent>()
            .add_system(change_player_system.system())
            // Flow
            .insert_resource(BattleFlow::PlayerStartTurn(PlayerStartTurn))
            .add_system(flow_systme.system())
            .add_event::<PlayerStartTurn>()
            .add_system(start_turn.system())
            .add_event::<StartTurnStatus>()
            .add_system(start_turn_status.system())
            .add_event::<StartTurnAbility>()
            .add_system(start_turn_ability.system())
            .add_event::<AiAction>()
            .add_system(ai_action.system())
            .add_event::<EndTurnStatus>()
            .add_system(end_turn_status.system())
            .add_event::<EndTurnAbility>()
            .add_system(end_turn_ability.system())
            .add_event::<ChangePlayer>()
            .add_system(change_player.system());
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BattleEvent {
    StartTurnStatus(StartTurnStatusEvent),
    Attack(AttackEvent),
    Dead(DeadEvent),
    UseAbility(AbilityEvent),
    EndTurn(EndTurnEvent),
    ChangePlayer(ChangePlayerEvent),
}
#[derive(Debug)]
enum EventState {
    Init,
    Animtaion,
    End,
}
#[derive(Default)]
pub struct EventQuene {
    quene: Vec<BattleEvent>,
}

impl EventQuene {
    pub fn add(&mut self, e: BattleEvent) {
        self.quene.push(e);
    }

    pub fn is_empty(&self) -> bool {
        self.quene.is_empty()
    }
}

fn battle_quene(
    mut quene: ResMut<EventQuene>,
    mut ev_animation: EventWriter<AninationRequestEvent>,
    mut ev_uianimation: EventWriter<UiAninationRequestEvent>,
    tracker: Res<AnimationTracker>,
    ui_tracker: Res<UiAnimationTracker>,
    mut es: ResMut<EventState>,
    mut ev_attack: EventWriter<AttackEvent>,
    mut ev_dead: EventWriter<DeadEvent>,
    mut ev_ability: EventWriter<AbilityEvent>,
    mut ev_end_turn: EventWriter<EndTurnEvent>,
    mut ev_start_turn: EventWriter<StartTurnStatusEvent>,
    mut ev_cp: EventWriter<ChangePlayerEvent>,
) {
    if let Some(e) = quene.quene.get(0) {
        match *es {
            EventState::Init => {
                // add anination...
                match e {
                    BattleEvent::StartTurnStatus(e) => {
                        info!("StartTurnStatus animation... {:?}", e);
                    }
                    BattleEvent::Attack(e) => {
                        ev_animation.send(AninationRequestEvent {
                            texture: vec![
                                "images/Classic/4/Classic_19.png",
                                "images/Classic/4/Classic_20.png",
                                "images/Classic/4/Classic_21.png",
                                "images/Classic/4/Classic_22.png",
                                "images/Classic/4/Classic_23.png",
                                "images/Classic/4/Classic_24.png",
                            ],
                            character: e.target,
                        });
                    }
                    BattleEvent::Dead(e) => {
                        ev_animation.send(AninationRequestEvent {
                            texture: vec![
                                "images/particlePack_1.1/PNG (Transparent)/smoke_01.png",
                                "images/particlePack_1.1/PNG (Transparent)/smoke_02.png",
                                "images/particlePack_1.1/PNG (Transparent)/smoke_03.png",
                                "images/particlePack_1.1/PNG (Transparent)/smoke_04.png",
                                "images/particlePack_1.1/PNG (Transparent)/smoke_05.png",
                                "images/particlePack_1.1/PNG (Transparent)/smoke_06.png",
                                "images/particlePack_1.1/PNG (Transparent)/smoke_07.png",
                                "images/particlePack_1.1/PNG (Transparent)/smoke_08.png",
                                "images/particlePack_1.1/PNG (Transparent)/smoke_09.png",
                                "images/particlePack_1.1/PNG (Transparent)/smoke_10.png",
                            ],
                            character: e.dead,
                        });
                    }
                    BattleEvent::UseAbility(_) => todo!(),
                    BattleEvent::EndTurn(e) => {
                        info!("EndTurn animation... {:?}", e);
                    }
                    BattleEvent::ChangePlayer(e) => {
                        info!("ChangePlayer animation... {:?}", e);
                        ev_uianimation.send(UiAninationRequestEvent);
                    }
                }
                *es = EventState::Animtaion;
            }
            EventState::Animtaion => {
                // wait for animation end
                if tracker.entities.is_empty() && ui_tracker.entities.is_empty() {
                    *es = EventState::End;
                }
            }
            EventState::End => {
                *es = EventState::Init;
                let e = quene.quene.remove(0);
                debug!("EventState::End, {:?}", e);
                match e {
                    BattleEvent::StartTurnStatus(e) => ev_start_turn.send(e),
                    BattleEvent::Attack(e) => ev_attack.send(e),
                    BattleEvent::Dead(e) => ev_dead.send(e),
                    BattleEvent::UseAbility(e) => ev_ability.send(e),
                    BattleEvent::EndTurn(e) => ev_end_turn.send(e),
                    BattleEvent::ChangePlayer(e) => ev_cp.send(e),
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct StartTurnStatusEvent {
    pub character: Entity,
}
fn start_turn_status_system(
    mut ev: EventReader<StartTurnStatusEvent>,
    mut character_query: Query<&mut Character>,
) {
    for e in ev.iter() {
        let result: Result<(), Box<dyn std::error::Error>> = (|| {
            let mut character = character_query.get_mut(e.character).unwrap();
            character.action_point = 1;

            Ok(())
        })();
        match result {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AttackEvent {
    pub source: Entity,
    pub target: Entity,
}
fn attack_system(
    mut ev: EventReader<AttackEvent>,
    mut quene: ResMut<EventQuene>,
    mut character_query: Query<&mut Character>,
) {
    for e in ev.iter() {
        let result: Result<(), Box<dyn std::error::Error>> = (|| {
            let mut attacker = character_query.get_mut(e.source)?;
            attacker.action_point -= 1;
            let attack = attacker.attack;
            let mut character = character_query.get_mut(e.target)?;
            character.hp -= attack;
            if character.hp <= 0 {
                quene.add(BattleEvent::Dead(DeadEvent {
                    dead: e.target,
                    killer: e.source,
                }));
            }

            Ok(())
        })();
        match result {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DeadEvent {
    pub dead: Entity,
    pub killer: Entity,
}
fn dead_system(
    mut ev: EventReader<DeadEvent>,
    // mut quene: ResMut<EventQuene>,
    // game: Res<Game>,
    mut query: Query<(Entity, &mut Character)>,
    mut visible_query: Query<&mut Visible>,
    children_query: Query<&Children>,
    // mut player_query: Query<&mut Player>,
    // mut position_query: Query<&mut CharacterPosition>,
) {
    for e in ev.iter() {
        let result: Result<(), Box<dyn std::error::Error>> = (|| {
            let (entity, mut character) = query.get_mut(e.dead)?;
            character.is_dead = true;
            set_visible_recursive(false, entity, &mut visible_query, &children_query);
            Ok(())
        })();
        match result {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn set_visible_recursive(
    is_visible: bool,
    entity: Entity,
    visible_query: &mut Query<&mut Visible>,
    children_query: &Query<&Children>,
) {
    if let Ok(mut visible) = visible_query.get_mut(entity) {
        visible.is_visible = is_visible;
    }

    if let Ok(children) = children_query.get(entity) {
        for child in children.iter() {
            set_visible_recursive(is_visible, *child, visible_query, children_query);
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AbilityEvent {
    pub ability: usize,
    pub source: Entity,
    pub target: Entity,
}
fn ability_system(
    mut ev: EventReader<AbilityEvent>,
    // mut quene: ResMut<EventQuene>,
    mut character_query: Query<(&mut Character, &Owner, Entity)>,
) {
    for e in ev.iter() {
        let result: Result<(), Box<dyn std::error::Error>> = (|| {
            let (ability, owner_player_index) = {
                let (character, o, _) = character_query.get_mut(e.source)?;
                let ability = character.abilities.get(e.ability).ok_or("No ability!")?;
                (ability.clone(), o.player_index)
            };

            for (et, ee) in ability.effects.iter() {
                let targets: Vec<Entity> = match et {
                    EffectTarget::Target => vec![e.target],
                    EffectTarget::Source => vec![e.source],
                    EffectTarget::AllEnemy => character_query
                        .iter_mut()
                        .filter(|(_, o, _)| o.player_index != owner_player_index)
                        .map(|(_, _, id)| id)
                        .collect(),
                    EffectTarget::AllAlliance => character_query
                        .iter_mut()
                        .filter(|(_, o, _)| o.player_index == owner_player_index)
                        .map(|(_, _, id)| id)
                        .collect(),
                    EffectTarget::AllEnemyExceptTarget => character_query
                        .iter_mut()
                        .filter(|(_, o, id)| {
                            o.player_index != owner_player_index && *id != e.target
                        })
                        .map(|(_, _, id)| id)
                        .collect(),
                    EffectTarget::AllAllianceExceptTarget => character_query
                        .iter_mut()
                        .filter(|(_, o, id)| {
                            o.player_index == owner_player_index && *id != e.target
                        })
                        .map(|(_, _, id)| id)
                        .collect(),
                };

                match ee {
                    EffectEvent::Heal(value) => {
                        for entity in targets {
                            let mut character = character_query.get_mut(entity).unwrap().0;
                            character.hp += value;
                        }
                    }
                    EffectEvent::SoulDrain(_) => {}
                    EffectEvent::Reflection => {}
                    EffectEvent::Curse(_) => {}
                    EffectEvent::Charm(_) => {}
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EndTurnEvent {
    pub player: Entity,
}
fn end_turn_system(mut ev: EventReader<EndTurnEvent>, mut flow: ResMut<BattleFlow>) {
    for _e in ev.iter() {
        let result: Result<(), Box<dyn std::error::Error>> = (|| {
            *flow = BattleFlow::EndTurn;
            Ok(())
        })();
        match result {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub struct ChangePlayerEvent;
fn change_player_system(mut ev: EventReader<ChangePlayerEvent>, mut game: ResMut<Game>) {
    for _e in ev.iter() {
        let result: Result<(), Box<dyn std::error::Error>> = (|| {
            game.current_index = (game.current_index + 1) % game.players.len();
            Ok(())
        })();
        match result {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum BattleFlow {
    PlayerStartTurn(PlayerStartTurn),
    StartTurnStatus(StartTurnStatus),
    StartTurnAbility(StartTurnAbility),
    PlayerAction,
    EndTurn,
    EndTurnStatus(EndTurnStatus),
    EndTurnAbility(EndTurnAbility),
    ChangePlayer,
}

fn flow_systme(
    game: Res<Game>,
    ev_q: Res<EventQuene>,
    mut flow: ResMut<BattleFlow>,
    mut ev_st: EventWriter<PlayerStartTurn>,
    mut ev_ss: EventWriter<StartTurnStatus>,
    mut ev_sa: EventWriter<StartTurnAbility>,
    mut ev_ai: EventWriter<AiAction>,
    mut ev_es: EventWriter<EndTurnStatus>,
    mut ev_ea: EventWriter<EndTurnAbility>,
    mut ev_cp: EventWriter<ChangePlayer>,
) {
    if !ev_q.quene.is_empty() {
        return;
    }
    debug!("Flow: {:?}", *flow);
    match *flow {
        BattleFlow::PlayerStartTurn(st) => {
            ev_st.send(st);
            *flow = BattleFlow::StartTurnStatus(StartTurnStatus {
                position: Position {
                    player_index: game.current_index,
                    x: 0,
                    y: 0,
                },
            });
        }
        BattleFlow::StartTurnStatus(mut cs) => {
            ev_ss.send(cs.clone());
            cs.position.x += 1;
            if cs.position.x >= 3 {
                cs.position.x = 0;
                cs.position.y += 1;
            }
            if cs.position.y < 3 {
                *flow = BattleFlow::StartTurnStatus(cs);
            } else {
                *flow = BattleFlow::StartTurnAbility(StartTurnAbility {
                    position: Position {
                        player_index: game.current_index,
                        x: 0,
                        y: 0,
                    },
                });
            }
        }
        BattleFlow::StartTurnAbility(mut ca) => {
            ev_sa.send(ca.clone());
            ca.position.x += 1;
            if ca.position.x >= 3 {
                ca.position.x = 0;
                ca.position.y += 1;
            }
            if ca.position.y < 3 {
                *flow = BattleFlow::StartTurnAbility(ca);
            } else {
                *flow = BattleFlow::PlayerAction;
            }
        }
        BattleFlow::PlayerAction => {
            if game.current_index == ENEMY_INDEX {
                ev_ai.send(AiAction);
            }
            // if player turn do nothing
        }
        BattleFlow::EndTurn => {
            *flow = BattleFlow::EndTurnStatus(EndTurnStatus {
                position: Position {
                    player_index: game.current_index,
                    x: 0,
                    y: 0,
                },
            });
        }
        BattleFlow::EndTurnStatus(mut e) => {
            ev_es.send(e.clone());
            e.position.x += 1;
            if e.position.x >= 3 {
                e.position.x = 0;
                e.position.y += 1;
            }
            if e.position.y < 3 {
                *flow = BattleFlow::EndTurnStatus(e);
            } else {
                *flow = BattleFlow::EndTurnAbility(EndTurnAbility {
                    position: Position {
                        player_index: game.current_index,
                        x: 0,
                        y: 0,
                    },
                });
            }
        }
        BattleFlow::EndTurnAbility(mut e) => {
            ev_ea.send(e.clone());
            e.position.x += 1;
            if e.position.x >= 3 {
                e.position.x = 0;
                e.position.y += 1;
            }
            if e.position.y < 3 {
                *flow = BattleFlow::EndTurnAbility(e);
            } else {
                *flow = BattleFlow::ChangePlayer;
            }
        }
        BattleFlow::ChangePlayer => {
            ev_cp.send(ChangePlayer);
            *flow = BattleFlow::PlayerStartTurn(PlayerStartTurn);
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct PlayerStartTurn;
fn start_turn(mut ev: EventReader<PlayerStartTurn>) {
    for _e in ev.iter() {
        let result: Result<(), Box<dyn std::error::Error>> = (|| {
            // todo...
            Ok(())
        })();
        match result {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct StartTurnStatus {
    position: Position,
}
fn start_turn_status(
    mut ev: EventReader<StartTurnStatus>,
    mut quene: ResMut<EventQuene>,
    character_query: Query<(Entity, &Character, &Position)>,
) {
    for e in ev.iter() {
        let result: Result<(), Box<dyn std::error::Error>> = (|| {
            // Status, AP
            if let Some((entity, _, _)) = character_query
                .iter()
                .find(|(_, _, &pos)| pos == e.position)
            {
                quene.add(BattleEvent::StartTurnStatus(StartTurnStatusEvent {
                    character: entity,
                }));
            }
            Ok(())
        })();
        match result {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct StartTurnAbility {
    position: Position,
}
fn start_turn_ability(
    mut ev: EventReader<StartTurnAbility>,
    mut quene: ResMut<EventQuene>,
    character_query: Query<(Entity, &Character, &Position)>,
) {
    for e in ev.iter() {
        let result: Result<(), Box<dyn std::error::Error>> = (|| {
            // Ability
            if let Some((entity, character, _)) = character_query
                .iter()
                .find(|(_, _, &pos)| pos == e.position)
            {
                let ability: Vec<usize> = character
                    .abilities
                    .iter()
                    .enumerate()
                    .filter_map(|(i, a)| match &a.ability_type {
                        AbilityType::Active(_) => None,
                        AbilityType::Passive(trigger) => match trigger {
                            TriggerEvent::TrunStart => Some(i),
                            _ => None,
                        },
                    })
                    .collect();
                for i in ability {
                    quene.quene.push(BattleEvent::UseAbility(AbilityEvent {
                        ability: i,
                        source: entity,
                        target: entity,
                    }));
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}

struct AiAction;
fn ai_action(mut ev: EventReader<AiAction>, mut quene: ResMut<EventQuene>, game: Res<Game>) {
    for _e in ev.iter() {
        let result: Result<(), Box<dyn std::error::Error>> = (|| {
            // Ai action...
            // Just end the turn now
            quene.add(BattleEvent::EndTurn(EndTurnEvent {
                player: game.players[ENEMY_INDEX],
            }));
            Ok(())
        })();
        match result {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct EndTurnStatus {
    position: Position,
}
fn end_turn_status(mut ev: EventReader<EndTurnStatus>, mut quene: ResMut<EventQuene>) {
    for e in ev.iter() {
        let result: Result<(), Box<dyn std::error::Error>> = (|| {
            // Status, AP
            Ok(())
        })();
        match result {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct EndTurnAbility {
    position: Position,
}
fn end_turn_ability(
    mut ev: EventReader<EndTurnAbility>,
    mut quene: ResMut<EventQuene>,
    character_query: Query<(Entity, &Character, &Position)>,
) {
    for e in ev.iter() {
        let result: Result<(), Box<dyn std::error::Error>> = (|| {
            // Ability
            if let Some((entity, character, _)) = character_query
                .iter()
                .find(|(_, _, &pos)| pos == e.position)
            {
                let ability: Vec<usize> = character
                    .abilities
                    .iter()
                    .enumerate()
                    .filter_map(|(i, a)| match &a.ability_type {
                        AbilityType::Active(_) => None,
                        AbilityType::Passive(trigger) => match trigger {
                            TriggerEvent::TurnEnd => Some(i),
                            _ => None,
                        },
                    })
                    .collect();
                for i in ability {
                    quene.quene.push(BattleEvent::UseAbility(AbilityEvent {
                        ability: i,
                        source: entity,
                        target: entity,
                    }));
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ChangePlayer;
fn change_player(mut ev: EventReader<ChangePlayer>, mut quene: ResMut<EventQuene>) {
    for _e in ev.iter() {
        let result: Result<(), Box<dyn std::error::Error>> = (|| {
            quene.add(BattleEvent::ChangePlayer(ChangePlayerEvent));
            Ok(())
        })();
        match result {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
    }
}
