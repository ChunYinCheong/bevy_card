use crate::{
    battle::{AbilityEvent, AttackEvent, BattleEvent, EndTurnEvent, EventQuene},
    game::{Character, Game, PLAYER_INDEX},
    input::PlayerActionEvent,
};
use bevy::prelude::*;

pub struct BattleActionPlugin;
impl Plugin for BattleActionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(player_event_system.system());
    }
}

fn player_event_system(
    mut ev_player: EventReader<PlayerActionEvent>,
    game: Res<Game>,
    mut quene: ResMut<EventQuene>,
    query: Query<&Character>,
) {
    if !quene.is_empty() {
        return;
    }
    for ev in ev_player.iter() {
        match ev {
            PlayerActionEvent::EndTurn => {
                let player_can_end_turn = true;
                if player_can_end_turn {
                    quene.add(BattleEvent::EndTurn(EndTurnEvent {
                        player: game.players[PLAYER_INDEX],
                    }));
                } else {
                    // Log error...
                }
            }
            PlayerActionEvent::UseAbility {
                ability,
                source,
                target,
            } => {
                // add checking...
                quene.add(BattleEvent::UseAbility(AbilityEvent {
                    ability: *ability,
                    source: *source,
                    target: *target,
                }));
            }
            PlayerActionEvent::Attack { source, target } => {
                // add checking...
                let character = query.get(*source).unwrap();
                if character.action_point > 0 {
                    quene.add(BattleEvent::Attack(AttackEvent {
                        source: *source,
                        target: *target,
                    }));
                }
            }
        }
    }
}
