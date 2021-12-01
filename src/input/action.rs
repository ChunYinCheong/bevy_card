use bevy::prelude::*;

use crate::game::Position;

use super::mapping::{HoveringEvent, InputMapping};

pub struct HoverChanged(pub Option<Entity>, pub Option<Entity>);
#[derive(Default)]
pub struct HoveringEntity(pub Option<Entity>);

pub fn handle_hovering_system(
    mut ev_hovering: EventReader<HoveringEvent>,
    mut hovering: ResMut<HoveringEntity>,
    mut ev_hover_changed: EventWriter<HoverChanged>,
) {
    for e in ev_hovering.iter() {
        if hovering.0 != e.0 {
            ev_hover_changed.send(HoverChanged(hovering.0, e.0));
            hovering.0 = e.0;
        }
    }
}

pub struct SelectEntity(pub Option<Entity>);
pub struct SelectionChanged(pub Option<Entity>, pub Option<Entity>);
#[derive(Default)]
pub struct SelectedEntity(pub Option<Entity>);

pub fn handle_selection_change(
    mut ev_select: EventReader<SelectEntity>,
    mut selected: ResMut<SelectedEntity>,
    mut ev_selection_changed: EventWriter<SelectionChanged>,
) {
    for e in ev_select.iter() {
        if selected.0 != e.0 {
            ev_selection_changed.send(SelectionChanged(selected.0, e.0));
            selected.0 = e.0;
        }
    }
}

pub enum InputState {
    InTurn,
    SelectedField(Entity),
    SelectedCharacter(Entity),
    SelectingAbilityTarget(Position, usize),
}

pub enum PlayerActionEvent {
    EndTurn,
    UseAbility {
        ability: usize,
        source: Entity,
        target: Entity,
    },
    Attack {
        source: Entity,
        target: Entity,
    },
}

pub fn input_event_system(
    mut input_state: ResMut<InputState>,
    mut ev_input: EventReader<InputMapping>,
    mut ev_player_action: EventWriter<PlayerActionEvent>,
    mut ev_select_entity: EventWriter<SelectEntity>,
) {
    for ev in ev_input.iter() {
        info!("Input Event: {:?}", ev);
        match ev {
            InputMapping::EndTurn => ev_player_action.send(PlayerActionEvent::EndTurn),
            InputMapping::SelectField(entity) => match *input_state {
                InputState::InTurn => {
                    *input_state = InputState::SelectedField(*entity);
                    ev_select_entity.send(SelectEntity(Some(*entity)));
                }
                InputState::SelectedField(_) | InputState::SelectedCharacter(_) => {
                    *input_state = InputState::SelectedField(*entity);
                    ev_select_entity.send(SelectEntity(Some(*entity)));
                }
                InputState::SelectingAbilityTarget(source_pos, ability) => {
                    todo!();
                }
            },
            InputMapping::SelectCharacter(entity) => match *input_state {
                InputState::InTurn => {
                    *input_state = InputState::SelectedCharacter(*entity);
                    ev_select_entity.send(SelectEntity(Some(*entity)));
                }
                InputState::SelectedField(_) => {
                    *input_state = InputState::SelectedCharacter(*entity);
                    ev_select_entity.send(SelectEntity(Some(*entity)));
                }
                InputState::SelectedCharacter(old_entity) => {
                    ev_player_action.send(PlayerActionEvent::Attack {
                        source: old_entity,
                        target: *entity,
                    });
                }
                InputState::SelectingAbilityTarget(source_pos, ability) => {
                    todo!();
                }
            },
            InputMapping::Cancel => match *input_state {
                InputState::InTurn => (),
                InputState::SelectedField(_) | InputState::SelectedCharacter(_) => {
                    *input_state = InputState::InTurn;
                    ev_select_entity.send(SelectEntity(None));
                }
                InputState::SelectingAbilityTarget(source_pos, ability) => {
                    todo!();
                }
            },
        }
    }
}
