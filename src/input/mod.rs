mod action;
mod mapping;

use self::{
    action::{
        handle_hovering_system, handle_selection_change, input_event_system, InputState,
        SelectEntity,
    },
    mapping::{
        click_sprite, cursor_system, keyboard_system, mouse_click_system, HoveringEvent,
        InputMapping, PositionClickedEvent,
    },
};
use bevy::prelude::*;

pub use action::HoverChanged;
pub use action::HoveringEntity;
pub use action::PlayerActionEvent;
pub use action::SelectedEntity;
pub use action::SelectionChanged;

pub struct GameInputPlugin;
impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(InputState::InTurn)
            .add_event::<InputMapping>()
            .add_event::<PlayerActionEvent>()
            .add_system(mouse_click_system.system())
            .add_system(keyboard_system.system())
            .add_system(input_event_system.system())
            .add_event::<PositionClickedEvent>()
            .add_event::<HoverChanged>()
            .add_system(cursor_system.system())
            .add_system(click_sprite.system())
            .init_resource::<HoveringEntity>()
            .add_event::<HoveringEvent>()
            .add_system(handle_hovering_system.system())
            .add_event::<SelectionChanged>()
            .init_resource::<SelectedEntity>()
            .add_system(handle_selection_change.system())
            .add_event::<SelectEntity>();
    }
}
