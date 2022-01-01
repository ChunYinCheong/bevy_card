use bevy::prelude::*;

use crate::game::{
    components::{AbilityDataId, AbilityInstance, PlayerController, UnitInstance, UnitInstanceRef},
    events::{
        BattleEvent, PlayerAction, PlayerActionEvent, PlayerEndTurn, SummonUnit, UnitStartAbility,
    },
};

use super::{battle::BattleFrame, Battle, BattleFlow};

pub fn player_event_system(
    mut ev_player: EventReader<PlayerActionEvent>,
    mut game: ResMut<Option<Battle>>,
    query: Query<&UnitInstance>,
    player_query: Query<(Entity, &PlayerController)>,
    ability_query: Query<(Entity, &AbilityInstance, &UnitInstanceRef)>,
) {
    if let Some(game) = game.as_mut() {
        if !game.stacks.is_empty() {
            return;
        }
        for ev in ev_player.iter() {
            if game.players[game.current_player_index] != ev.player_id {
                continue;
            }
            match game.stage {
                BattleFlow::PlayerAction => (),
                _ => {
                    continue;
                }
            }
            match ev.action {
                PlayerAction::EndTurn => {
                    game.stacks.push(BattleFrame {
                        event: BattleEvent::PlayerEndTurn(PlayerEndTurn {
                            player: ev.player_id,
                        }),
                        on_stacks: Default::default(),
                        started: false,
                    });
                }
                PlayerAction::UseAbility {
                    ability,
                    source,
                    target,
                } => {
                    // add checking...
                    game.stacks.push(BattleFrame {
                        event: BattleEvent::UnitStartAbility(UnitStartAbility {
                            ability,
                            source,
                            target,
                        }),
                        on_stacks: Default::default(),
                        started: false,
                    });
                }
                PlayerAction::Attack { source, target } => {
                    // add checking...
                    let character = query.get(source).unwrap();
                    if character.ap > 0 {
                        if let Some((ability, _, _)) = ability_query.iter().find(|(_, a, u)| {
                            a.ability_data_id == AbilityDataId::Attack && u.0 == source
                        }) {
                            game.stacks.push(BattleFrame {
                                event: BattleEvent::UnitStartAbility(UnitStartAbility {
                                    ability,
                                    source,
                                    target,
                                }),
                                on_stacks: Default::default(),
                                started: false,
                            });
                        }
                    }
                }
                PlayerAction::SummonUnit { card, position } => {
                    // checking
                    game.stacks.push(BattleFrame {
                        event: BattleEvent::SummonUnit(SummonUnit { card, position }),
                        on_stacks: Default::default(),
                        started: false,
                    });
                }
            }
        }
    }
}
