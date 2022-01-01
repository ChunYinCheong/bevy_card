use bevy::prelude::*;

use crate::game::{
    components::{PlayerController, Position, PositionType},
    events::{
        InputMappingEvent, PlayerAction, PlayerActionEvent, SelectEntityEvent,
        SelectionChangedEvent,
    },
};

#[derive(Debug)]
pub enum InputState {
    InTurn,
    SelectedCell(Entity),
    SelectedUnitCard(Entity),
    SelectedUnitCardInHand(Entity),
}

pub fn input_event_system(
    mut input_state: ResMut<InputState>,
    mut ev_input: EventReader<InputMappingEvent>,
    mut ev_player_action: EventWriter<PlayerActionEvent>,
    mut ev_select_entity: EventWriter<SelectEntityEvent>,
    pos_q: Query<&Position>,
    controller_query: Query<(Entity, &PlayerController)>,
) {
    for ev in ev_input.iter() {
        info!("Input Event: {:?}", ev);
        match ev {
            InputMappingEvent::EndTurn => {
                let (player_id, _) = controller_query
                    .iter()
                    .find(|(_, &p)| p == PlayerController::Player)
                    .unwrap();
                ev_player_action.send(PlayerActionEvent {
                    player_id,
                    action: PlayerAction::EndTurn,
                })
            }
            InputMappingEvent::ClickCell(entity) => match *input_state {
                InputState::InTurn => {
                    *input_state = InputState::SelectedCell(*entity);
                    ev_select_entity.send(SelectEntityEvent(Some(*entity)));
                }
                InputState::SelectedCell(_) | InputState::SelectedUnitCard(_) => {
                    *input_state = InputState::SelectedCell(*entity);
                    ev_select_entity.send(SelectEntityEvent(Some(*entity)));
                }
                InputState::SelectedUnitCardInHand(e) => {
                    let (player_id, _) = controller_query
                        .iter()
                        .find(|(_, &p)| p == PlayerController::Player)
                        .unwrap();
                    let pos = pos_q.get(*entity).unwrap();
                    ev_player_action.send(PlayerActionEvent {
                        player_id,
                        action: PlayerAction::SummonUnit {
                            card: e,
                            position: pos.clone(),
                        },
                    });
                    *input_state = InputState::InTurn;
                    ev_select_entity.send(SelectEntityEvent(None));
                }
            },
            InputMappingEvent::ClickCard(entity) => match *input_state {
                InputState::InTurn | InputState::SelectedCell(_) => {
                    let pos = pos_q.get(*entity).unwrap();
                    match pos.position_type {
                        PositionType::Hand => {
                            *input_state = InputState::SelectedUnitCardInHand(*entity);
                            ev_select_entity.send(SelectEntityEvent(Some(*entity)));
                        }
                        PositionType::Deck | PositionType::Board | PositionType::Grave => {
                            *input_state = InputState::SelectedUnitCard(*entity);
                            ev_select_entity.send(SelectEntityEvent(Some(*entity)));
                        }
                    }
                }
                InputState::SelectedUnitCard(old_entity) => {
                    let old_pos = pos_q.get(old_entity).unwrap();
                    match old_pos.position_type {
                        PositionType::Board => {
                            let new_pos = pos_q.get(*entity).unwrap();
                            match new_pos.position_type {
                                PositionType::Hand => {
                                    *input_state = InputState::SelectedUnitCardInHand(*entity);
                                    ev_select_entity.send(SelectEntityEvent(Some(*entity)));
                                }
                                PositionType::Deck | PositionType::Grave => {
                                    *input_state = InputState::SelectedUnitCard(*entity);
                                    ev_select_entity.send(SelectEntityEvent(Some(*entity)));
                                }
                                PositionType::Board => {
                                    let (player_id, _) = controller_query
                                        .iter()
                                        .find(|(_, &p)| p == PlayerController::Player)
                                        .unwrap();
                                    ev_player_action.send(PlayerActionEvent {
                                        player_id,
                                        action: PlayerAction::Attack {
                                            source: old_entity,
                                            target: *entity,
                                        },
                                    });
                                }
                            }
                        }
                        PositionType::Deck | PositionType::Hand | PositionType::Grave => {
                            let new_pos = pos_q.get(*entity).unwrap();
                            match new_pos.position_type {
                                PositionType::Hand => {
                                    *input_state = InputState::SelectedUnitCardInHand(*entity);
                                    ev_select_entity.send(SelectEntityEvent(Some(*entity)));
                                }
                                PositionType::Deck | PositionType::Grave | PositionType::Board => {
                                    *input_state = InputState::SelectedUnitCard(*entity);
                                    ev_select_entity.send(SelectEntityEvent(Some(*entity)));
                                }
                            }
                        }
                    }
                }
                InputState::SelectedUnitCardInHand(_) => {
                    let pos = pos_q.get(*entity).unwrap();
                    match pos.position_type {
                        PositionType::Hand => {
                            *input_state = InputState::SelectedUnitCardInHand(*entity);
                            ev_select_entity.send(SelectEntityEvent(Some(*entity)));
                        }
                        PositionType::Deck | PositionType::Board | PositionType::Grave => {
                            *input_state = InputState::SelectedUnitCard(*entity);
                            ev_select_entity.send(SelectEntityEvent(Some(*entity)));
                        }
                    }
                }
            },
            InputMappingEvent::Cancel => match *input_state {
                InputState::InTurn => (),
                InputState::SelectedCell(_)
                | InputState::SelectedUnitCard(_)
                | InputState::SelectedUnitCardInHand(_) => {
                    *input_state = InputState::InTurn;
                    ev_select_entity.send(SelectEntityEvent(None));
                }
            },
        }
        info!("Input State: {:?}", *input_state);
    }
}

#[derive(Default)]
pub struct SelectedEntity(pub Option<Entity>);

pub fn handle_selection_change(
    mut ev_select: EventReader<SelectEntityEvent>,
    mut selected: ResMut<SelectedEntity>,
    mut ev_selection_changed: EventWriter<SelectionChangedEvent>,
) {
    for e in ev_select.iter() {
        if selected.0 != e.0 {
            ev_selection_changed.send(SelectionChangedEvent(selected.0, e.0));
            selected.0 = e.0;
        }
    }
}
