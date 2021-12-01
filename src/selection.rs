use bevy::prelude::*;

use crate::{
    input::{HoverChanged, HoveringEntity, SelectedEntity, SelectionChanged},
    selection_box::SelectionBoxColor,
};

pub struct SelectionPlugin;
impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(selection_change.system())
            .add_system(hovering_system.system());
    }
}

fn selection_change(
    mut ev_selection_change: EventReader<SelectionChanged>,
    mut query: Query<&mut SelectionBoxColor>,
    hovering: Res<HoveringEntity>,
    selected: Res<SelectedEntity>,
) {
    for e in ev_selection_change.iter() {
        if let Some(entity) = e.0 {
            update_color(entity, &mut query, &hovering, &selected);
        }
        if let Some(entity) = e.1 {
            update_color(entity, &mut query, &hovering, &selected);
        }
    }
}

fn hovering_system(
    mut ev_hover_changed: EventReader<HoverChanged>,
    mut query: Query<&mut SelectionBoxColor>,
    hovering: Res<HoveringEntity>,
    selected: Res<SelectedEntity>,
) {
    for e in ev_hover_changed.iter() {
        if let Some(entity) = e.0 {
            update_color(entity, &mut query, &hovering, &selected);
        }
        if let Some(entity) = e.1 {
            update_color(entity, &mut query, &hovering, &selected);
        }
    }
}

fn update_color(
    entity: Entity,
    query: &mut Query<&mut SelectionBoxColor>,
    hovering: &Res<HoveringEntity>,
    selected: &Res<SelectedEntity>,
) {
    if let Ok(mut color) = query.get_mut(entity) {
        if hovering.0 == Some(entity) && selected.0 == Some(entity) {
            *color = SelectionBoxColor::GreenYellow;
        } else if hovering.0 == Some(entity) {
            *color = SelectionBoxColor::Yellow;
        } else if selected.0 == Some(entity) {
            *color = SelectionBoxColor::Green;
        } else {
            *color = SelectionBoxColor::None;
        }
    }
}
