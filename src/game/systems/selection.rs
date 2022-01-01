use bevy::prelude::*;

use crate::{
    game::systems::HighlightColor,
    game::{
        events::{HoverChangedEvent, SelectionChangedEvent},
        HoveringEntity, SelectedEntity,
    },
};

pub fn selection_change(
    mut ev_selection_change: EventReader<SelectionChangedEvent>,
    mut query: Query<&mut HighlightColor>,
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

pub fn hovering_system(
    mut ev_hover_changed: EventReader<HoverChangedEvent>,
    mut query: Query<&mut HighlightColor>,
    hovering: Res<HoveringEntity>,
    selected: Res<SelectedEntity>,
) {
    for e in ev_hover_changed.iter() {
        if let Some(entity) = e.old {
            update_color(entity, &mut query, &hovering, &selected);
        }
        if let Some(entity) = e.new {
            update_color(entity, &mut query, &hovering, &selected);
        }
    }
}

fn update_color(
    entity: Entity,
    query: &mut Query<&mut HighlightColor>,
    hovering: &Res<HoveringEntity>,
    selected: &Res<SelectedEntity>,
) {
    if let Ok(mut color) = query.get_mut(entity) {
        if hovering.0 == Some(entity) && selected.0 == Some(entity) {
            *color = HighlightColor::GreenYellow;
        } else if hovering.0 == Some(entity) {
            *color = HighlightColor::Yellow;
        } else if selected.0 == Some(entity) {
            *color = HighlightColor::Green;
        } else {
            *color = HighlightColor::None;
        }
    }
}
