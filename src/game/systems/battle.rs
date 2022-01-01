use bevy::prelude::*;

use crate::{
    game::{
        components::{
            AbilityDataId, AbilityEffect, AbilityInstance, AbilityType, CardInstance, EffectTarget,
            PlayerController, PlayerInstance, Position, PositionType, TriggerCondition,
            UnitInstance, UnitInstanceRef, UnitVar,
        },
        events::{
            AbilityStart, BattleEvent, PlayerChange, PlayerDraw, PlayerEndTurn, PreTurnStart,
            SummonUnit, TurnStart, UnitCombat, UnitDie, UnitHurt, UnitStartAbility,
        },
    },
    AppState,
};

#[derive(Debug)]
pub struct BattleFrame {
    pub event: BattleEvent,
    pub on_stacks: Vec<Entity>,
    pub started: bool,
}

pub fn update(
    mut battle: ResMut<Option<Battle>>,
    ability_query: Query<(Entity, &AbilityInstance, &UnitInstanceRef)>,
    mut unit_query: Query<(Entity, &mut UnitInstance)>,
    mut ev_pre_turn_start: EventWriter<PreTurnStart>,
    mut ev_unit_hurt: EventWriter<UnitHurt>,
    mut ev_player_draw: EventWriter<PlayerDraw>,
    mut ev_summon_unit: EventWriter<SummonUnit>,
    mut ev_unit_die: EventWriter<UnitDie>,
) {
    if let Some(battle) = battle.as_mut() {
        let stacks = &mut battle.stacks;
        'outer: while !stacks.is_empty() {
            // check animation...
            let s = stacks.last_mut().unwrap();
            if !s.started {
                let mut e: Vec<_> = ability_query
                    .iter()
                    .filter(|(_, a, _)| match &a.ability_type {
                        AbilityType::Active(_) => false,
                        AbilityType::Trigger(t) => t.event.match_event(&s.event),
                    })
                    .collect();
                e.sort_by_key(|(_, a, _)| a.priority);
                let mut ids = e.iter().map(|(id, _, _)| *id).collect();
                s.on_stacks.append(&mut ids);
                s.started = true;
                info!("Frame start: {:?}", s);
                // add animation
            }
            // check animation...
            // on event
            while !s.on_stacks.is_empty() {
                let ss = s.on_stacks.pop().unwrap();
                let (entity, ability, unit_ref) = ability_query.get(ss).unwrap();
                if let AbilityType::Trigger(t) = &ability.ability_type {
                    let ability_unit = unit_ref.0; // ability unit ref
                    let trigger_unit = match &s.event {
                        BattleEvent::PreTurnStart(_) => ability_unit,
                        BattleEvent::TurnStart(_) => ability_unit,
                        BattleEvent::TurnEnd(_) => ability_unit,
                        BattleEvent::PostTurnEnd(_) => ability_unit,
                        BattleEvent::PlayerChange(_) => todo!(),
                        BattleEvent::PlayerDraw(_) => todo!(),
                        BattleEvent::PlayerEndTurn(event) => event.player,
                        BattleEvent::UnitHurt(event) => event.target,
                        BattleEvent::UnitDie(event) => event.dead,
                        BattleEvent::UnitStartAbility(event) => event.source,
                        BattleEvent::UnitCombat(event) => event.source,
                        BattleEvent::AbilityStart(event) => event.source,
                        BattleEvent::SummonUnit(_) => todo!(),
                    };
                    let action_unit = match &s.event {
                        BattleEvent::PreTurnStart(_) => ability_unit,
                        BattleEvent::TurnStart(_) => ability_unit,
                        BattleEvent::TurnEnd(_) => ability_unit,
                        BattleEvent::PostTurnEnd(_) => ability_unit,
                        BattleEvent::PlayerChange(_) => todo!(),
                        BattleEvent::PlayerDraw(_) => todo!(),
                        BattleEvent::PlayerEndTurn(event) => event.player,
                        BattleEvent::UnitHurt(event) => event.source,
                        BattleEvent::UnitDie(event) => event.killer,
                        BattleEvent::UnitStartAbility(event) => event.source,
                        BattleEvent::UnitCombat(event) => event.source,
                        BattleEvent::AbilityStart(event) => event.source,
                        BattleEvent::SummonUnit(_) => todo!(),
                    };
                    if t.conditions.iter().all(|condition| match condition {
                        TriggerCondition::UnitEq(u1, u2) => {
                            let u1 = match u1 {
                                UnitVar::TriggerUnit => trigger_unit,
                                UnitVar::ActionUnit => action_unit,
                                UnitVar::AbilityUnit => ability_unit,
                            };
                            let u2 = match u2 {
                                UnitVar::TriggerUnit => trigger_unit,
                                UnitVar::ActionUnit => action_unit,
                                UnitVar::AbilityUnit => ability_unit,
                            };
                            u1 == u2
                        }
                    }) {
                        info!("Ability trigger: {:?}", ability);
                        stacks.push(BattleFrame {
                            event: BattleEvent::UnitStartAbility(UnitStartAbility {
                                ability: entity,
                                source: ability_unit,
                                target: trigger_unit,
                            }),
                            on_stacks: Default::default(),
                            started: false,
                        });
                        // handle new frame
                        continue 'outer;
                    }
                } else {
                    panic!("Not trigger");
                }
            }

            // start effect
            let s = stacks.pop().unwrap();
            let can_run = true;
            if can_run {
                info!("Run Frame: {:?}", s);
                // add animation
                match s.event {
                    BattleEvent::PreTurnStart(event) => {
                        // AP, status effect
                        ev_pre_turn_start.send(event);
                    }
                    BattleEvent::TurnStart(event) => {
                        // trigger ability
                        // Draw
                        stacks.push(BattleFrame {
                            event: BattleEvent::PlayerDraw(PlayerDraw {
                                player: event.player,
                                c: 1,
                            }),
                            on_stacks: Default::default(),
                            started: false,
                        });
                    }
                    BattleEvent::TurnEnd(_) => {
                        // trigger ability
                        // do nothing
                    }
                    BattleEvent::PostTurnEnd(_) => {
                        // status effect
                    }
                    BattleEvent::PlayerChange(event) => {
                        battle.current_player_index = event.next_index;
                    }
                    BattleEvent::PlayerDraw(event) => {
                        ev_player_draw.send(event);
                    }
                    BattleEvent::PlayerEndTurn(_) => {
                        battle.stage = BattleFlow::TurnEnd;
                    }
                    BattleEvent::UnitHurt(event) => {
                        // change hp
                        // check die
                        ev_unit_hurt.send(event);
                    }
                    BattleEvent::UnitDie(event) => {
                        // unit already die
                        // move it to grave
                        ev_unit_die.send(event);
                    }
                    BattleEvent::UnitCombat(event) => {
                        // reverse order
                        let (_, attacker) = unit_query.get_mut(event.source).unwrap();
                        let att_atk = attacker.atk;
                        let att_def = attacker.def;
                        let (_, victim) = unit_query.get_mut(event.target).unwrap();
                        let vic_atk = victim.atk;
                        let vic_def = victim.def;
                        stacks.push(BattleFrame {
                            event: BattleEvent::UnitHurt(UnitHurt {
                                source: event.target,
                                target: event.source,
                                value: (vic_atk - att_def).max(0),
                            }),
                            on_stacks: Default::default(),
                            started: false,
                        });
                        stacks.push(BattleFrame {
                            event: BattleEvent::UnitHurt(UnitHurt {
                                source: event.source,
                                target: event.target,
                                value: (att_atk - vic_def).max(0),
                            }),
                            on_stacks: Default::default(),
                            started: false,
                        });
                    }
                    BattleEvent::UnitStartAbility(event) => {
                        // cost AP
                        let (_, a, _) = ability_query.get(event.ability).unwrap();
                        let (_, mut u) = unit_query.get_mut(event.source).unwrap();
                        u.ap -= a.ap;
                        // to ability start
                        stacks.push(BattleFrame {
                            event: BattleEvent::AbilityStart(AbilityStart {
                                ability: event.ability,
                                source: event.source,
                                target: event.target,
                            }),
                            on_stacks: Default::default(),
                            started: false,
                        });
                    }
                    BattleEvent::AbilityStart(event) => {
                        let (_, ability, _) = ability_query.get(event.ability).unwrap();
                        let mut events: Vec<_> = ability
                            .effects
                            .iter()
                            .map(|(t, e)| match e {
                                AbilityEffect::Attack => BattleEvent::UnitCombat(UnitCombat {
                                    source: event.source,
                                    target: event.target,
                                }),
                                AbilityEffect::Damage(damage) => BattleEvent::UnitHurt(UnitHurt {
                                    source: match t {
                                        EffectTarget::Target => event.target,
                                        EffectTarget::Source => event.source,
                                        EffectTarget::AllEnemy => todo!(),
                                        EffectTarget::AllAlliance => todo!(),
                                        EffectTarget::AllEnemyExceptTarget => todo!(),
                                        EffectTarget::AllAllianceExceptTarget => todo!(),
                                    },
                                    target: match t {
                                        EffectTarget::Target => event.target,
                                        EffectTarget::Source => event.source,
                                        EffectTarget::AllEnemy => todo!(),
                                        EffectTarget::AllAlliance => todo!(),
                                        EffectTarget::AllEnemyExceptTarget => todo!(),
                                        EffectTarget::AllAllianceExceptTarget => todo!(),
                                    },
                                    value: *damage,
                                }),
                                AbilityEffect::Heal(_) => todo!(),
                                AbilityEffect::SoulDrain(_) => todo!(),
                                AbilityEffect::Reflection => todo!(),
                                AbilityEffect::Curse(_) => todo!(),
                                AbilityEffect::Charm(_) => todo!(),
                            })
                            .map(|event| BattleFrame {
                                event,
                                on_stacks: Default::default(),
                                started: false,
                            })
                            .collect();
                        events.reverse();
                        stacks.append(&mut events);
                    }
                    BattleEvent::SummonUnit(event) => {
                        ev_summon_unit.send(event);
                    }
                }
                // limit 1 event pre frame
                return;
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Battle {
    pub stage: BattleFlow,
    pub stacks: Vec<BattleFrame>,
    pub event_state: EventState,
    pub animations: Vec<Entity>,

    pub players: Vec<Entity>,
    pub boards: Vec<Entity>,
    pub current_player_index: usize,
}

#[derive(Debug)]
pub enum EventState {
    Init,
    Animtaion,
    End,
}

impl Default for EventState {
    fn default() -> Self {
        Self::Init
    }
}

pub fn summon_unit_system(
    mut ev: EventReader<SummonUnit>,
    mut query: Query<(Entity, &mut CardInstance, &mut Position)>,
) {
    for e in ev.iter() {
        let (_, card, mut pos) = query.get_mut(e.card).unwrap();
        let z = pos.z;
        *pos = e.position;
        query
            .iter_mut()
            .filter(|(_, _, pos)| {
                pos.player_id == e.position.player_id
                    && pos.position_type == PositionType::Hand
                    && pos.z > z
            })
            .for_each(|(_, _, mut pos)| pos.z -= 1);
    }
}

pub fn player_draw_system(
    mut ev: EventReader<PlayerDraw>,
    mut query: Query<(Entity, &mut CardInstance, &mut Position)>,
) {
    for e in ev.iter() {
        // player.hands.get(index)
        let mut index = query
            .iter_mut()
            .filter(|(_, _, pos)| {
                pos.player_id == e.player && pos.position_type == PositionType::Hand
            })
            .count();
        let mut decks = query
            .iter_mut()
            .filter(|(_, _, pos)| {
                pos.player_id == e.player && pos.position_type == PositionType::Deck
            })
            .collect::<Vec<_>>();
        decks.sort_by_key(|(_, _, pos)| pos.z);
        for (i, (_, _, mut pos)) in decks.into_iter().enumerate() {
            if (i as i32) < e.c {
                pos.position_type = PositionType::Hand;
                pos.z = index as i32;
                index += 1;
            } else {
                pos.z = i as i32 - e.c;
            }
        }
    }
}

pub fn unit_die_system(
    mut ev: EventReader<UnitDie>,
    mut query_set: QuerySet<(
        Query<(Entity, &UnitInstance, &Position)>,
        Query<(Entity, &mut UnitInstance, &mut Position)>,
        Query<(Entity, &mut CardInstance, &mut Position)>,
        Query<(Entity, &CardInstance, &Position)>,
    )>,
    player_query: Query<(Entity, &PlayerInstance, &PlayerController)>,
    mut battle: ResMut<Option<Battle>>,
) {
    for e in ev.iter() {
        let (_, unit, pos) = query_set.q1_mut().get_mut(e.dead).unwrap();
        let player_id = unit.owner;
        let pid = pos.player_id;
        let px = pos.x;
        let py = pos.y;
        let z = query_set
            .q3()
            .iter()
            .filter(|(_, _, pos)| {
                pos.player_id == player_id && pos.position_type == PositionType::Grave
            })
            .count();
        query_set
            .q2_mut()
            .iter_mut()
            .filter(|(_, _, p)| {
                p.position_type == PositionType::Board
                    && p.player_id == pid
                    && p.x == px
                    && p.y == py
            })
            .for_each(|(_, _, mut pos)| {
                pos.position_type = PositionType::Grave;
                pos.z = pos.z + z as i32;
            });

        let end = battle
            .as_mut()
            .unwrap()
            .players
            .iter()
            .map(|&id| player_query.get(id).unwrap())
            .map(|(_, player, _)| query_set.q3().get(player.player_card_instance_id).unwrap())
            .any(|(card, _, _)| card == e.dead);
        if end {
            battle.as_mut().unwrap().stage = BattleFlow::BattleEnd;
        }
    }
}

pub fn pre_turn_start_system(
    mut ev: EventReader<PreTurnStart>,
    mut query: Query<(Entity, &mut UnitInstance, &mut Position)>,
) {
    for e in ev.iter() {
        for (_, mut unit, _) in query.iter_mut().filter(|(_, _, pos)| {
            pos.position_type == PositionType::Board && pos.player_id == e.player
        }) {
            unit.ap += 1;
        }
    }
}

pub fn unit_hurt_system(
    mut ev: EventReader<UnitHurt>,
    mut query: Query<(Entity, &mut UnitInstance)>,
    mut battle: ResMut<Option<Battle>>,
) {
    for e in ev.iter() {
        let (_, mut unit) = query.get_mut(e.target).unwrap();
        unit.hp -= e.value;
        if unit.hp <= 0 {
            battle.as_mut().unwrap().stacks.push(BattleFrame {
                event: BattleEvent::UnitDie(UnitDie {
                    dead: e.target,
                    killer: e.source,
                }),
                on_stacks: Default::default(),
                started: false,
            });
        }
    }
}

fn ability_system(
    e: AbilityStart,
    query: &mut Query<(Entity, &mut UnitInstance, &mut Position)>,
    ability_query: &mut Query<(Entity, &mut AbilityInstance)>,
) {
    let result: Result<(), Box<dyn std::error::Error>> = (|| {
        let owner_id = {
            let (_, card, _) = query.get_mut(e.source)?;
            card.owner
        };

        let (_, ability) = ability_query.get_mut(e.ability)?;
        for (et, ee) in ability.effects.iter() {
            let targets: Vec<Entity> = match et {
                EffectTarget::Target => vec![e.target],
                EffectTarget::Source => vec![e.source],
                EffectTarget::AllEnemy => query
                    .iter_mut()
                    .filter(|(_, owner, _)| owner.owner != owner_id)
                    .map(|(id, _, _)| id)
                    .collect(),
                EffectTarget::AllAlliance => query
                    .iter_mut()
                    .filter(|(_, o, _)| o.owner == owner_id)
                    .map(|(id, _, _)| id)
                    .collect(),
                EffectTarget::AllEnemyExceptTarget => query
                    .iter_mut()
                    .filter(|(id, o, _)| o.owner != owner_id && *id != e.target)
                    .map(|(id, _, _)| id)
                    .collect(),
                EffectTarget::AllAllianceExceptTarget => query
                    .iter_mut()
                    .filter(|(id, o, _)| o.owner == owner_id && *id != e.target)
                    .map(|(id, _, _)| id)
                    .collect(),
            };

            match ee {
                AbilityEffect::Attack => {
                    //
                    todo!()
                }
                AbilityEffect::Damage(damage) => {
                    for entity in targets {
                        let (_, mut character, _) = query.get_mut(entity).unwrap();
                        character.hp -= damage;
                    }
                }
                AbilityEffect::Heal(value) => {
                    for entity in targets {
                        let (_, mut character, _) = query.get_mut(entity).unwrap();
                        character.hp += value;
                    }
                }
                AbilityEffect::SoulDrain(_) => {}
                AbilityEffect::Reflection => {}
                AbilityEffect::Curse(_) => {}
                AbilityEffect::Charm(_) => {}
            }
        }
        Ok(())
    })();
    match result {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BattleFlow {
    BattleStart,
    PreTurnStart,
    TurnStart,
    // PostTurnStart,
    PlayerAction,
    // PreTurnEnd,
    TurnEnd,
    PostTurnEnd,
    PlayerChange,
    BattleEnd,
}

impl Default for BattleFlow {
    fn default() -> Self {
        Self::BattleStart
    }
}

pub fn flow_systme(
    mut battle: ResMut<Option<Battle>>,
    unit_query: Query<(Entity, &UnitInstance, &Position)>,
    player_query: Query<(Entity, &PlayerInstance, &PlayerController)>,
    ability_query: Query<(Entity, &AbilityInstance, &UnitInstanceRef)>,
    mut app_state: ResMut<State<AppState>>,
) {
    if let Some(game) = battle.as_mut() {
        if !game.stacks.is_empty() {
            return;
        }
        let next_stage = match game.stage {
            BattleFlow::BattleStart => Some(BattleFlow::PreTurnStart),
            BattleFlow::PreTurnStart => {
                let &player = game.players.get(game.current_player_index).unwrap();
                game.stacks.push(BattleFrame {
                    event: BattleEvent::PreTurnStart(PreTurnStart { player }),
                    on_stacks: Default::default(),
                    started: false,
                });
                Some(BattleFlow::TurnStart)
            }
            BattleFlow::TurnStart => {
                let &player = game.players.get(game.current_player_index).unwrap();
                game.stacks.push(BattleFrame {
                    event: BattleEvent::TurnStart(TurnStart { player }),
                    on_stacks: Default::default(),
                    started: false,
                });
                Some(BattleFlow::PlayerAction)
            }
            BattleFlow::PlayerAction => {
                // warn!("BattleFlow::PlayerAction need TODO!");
                if let Some(&id) = game.players.get(game.current_player_index) {
                    // if player turn do nothing
                    if player_query
                        .get(id)
                        .map(|(_, _, controller)| match controller {
                            PlayerController::Player => false,
                            PlayerController::Ai => true,
                        })
                        .unwrap_or_default()
                    {
                        game.stacks.push(BattleFrame {
                            event: BattleEvent::PlayerEndTurn(PlayerEndTurn { player: id }),
                            on_stacks: Default::default(),
                            started: false,
                        });
                        for (enemy_unit, _, _) in unit_query
                            .iter()
                            .find(|(_, unit, _)| unit.owner == id && unit.ap > 0)
                        {
                            if let Some((target_unit, _, _)) =
                                unit_query.iter().find(|(_, unit, pos)| {
                                    unit.owner != id && pos.position_type == PositionType::Board
                                })
                            {
                                if let Some((ability, _, _)) =
                                    ability_query.iter().find(|(_, a, u)| {
                                        a.ability_data_id == AbilityDataId::Attack
                                            && u.0 == enemy_unit
                                    })
                                {
                                    game.stacks.push(BattleFrame {
                                        event: BattleEvent::UnitStartAbility(UnitStartAbility {
                                            ability,
                                            source: enemy_unit,
                                            target: target_unit,
                                        }),
                                        on_stacks: Default::default(),
                                        started: false,
                                    });
                                }
                            }
                        }
                    }
                }
                None
            }
            BattleFlow::TurnEnd => Some(BattleFlow::PostTurnEnd),
            BattleFlow::PostTurnEnd => Some(BattleFlow::PlayerChange),
            BattleFlow::PlayerChange => {
                let next_index = (game.current_player_index + 1) % game.players.len();
                let next_player = game.players[next_index];
                game.stacks.push(BattleFrame {
                    event: BattleEvent::PlayerChange(PlayerChange {
                        next_player,
                        next_index,
                    }),
                    on_stacks: Default::default(),
                    started: false,
                });
                Some(BattleFlow::PreTurnStart)
            }
            BattleFlow::BattleEnd => {
                println!("battle_end");
                app_state.set(AppState::BattleEnd).unwrap();
                None
            }
        };
        if let Some(next_stage) = next_stage {
            info!("Next Stage: {:?} => {:?}", game.stage, next_stage);
            game.stage = next_stage;
        }
    }
}
