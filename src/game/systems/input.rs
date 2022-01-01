use bevy::prelude::*;

use crate::game::{
    components::{SelectType, Selectable},
    events::InputMappingEvent,
};

use super::HoveringEntity;

pub fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut ev_input: EventWriter<InputMappingEvent>,
    hovering: Res<HoveringEntity>,
    query: Query<&Selectable>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        ev_input.send(InputMappingEvent::Cancel);
    }
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(entity) = hovering.0 {
            let selectable = query.get(entity).unwrap();
            match selectable.select_type {
                SelectType::Cell => {
                    ev_input.send(InputMappingEvent::ClickCell(entity));
                }
                SelectType::Card => {
                    ev_input.send(InputMappingEvent::ClickCard(entity));
                }
            }
        }
    }
}

pub fn keyboard_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut ev_input: EventWriter<InputMappingEvent>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        ev_input.send(InputMappingEvent::EndTurn);
    }
}
