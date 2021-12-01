use crate::{
    game::{Character, Field, Position},
    MainCamera,
};
use bevy::prelude::*;

#[derive(Debug)]
pub enum InputMapping {
    EndTurn,
    SelectField(Entity),
    SelectCharacter(Entity),
    Cancel,
}

pub struct HoveringEvent(pub Option<Entity>);
pub struct PositionClickedEvent(Entity);

pub fn cursor_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut ev_click: EventWriter<PositionClickedEvent>,
    mut ev_hovering: EventWriter<HoveringEvent>,
    q_camera: Query<&Transform, With<MainCamera>>,
    sprite_query: Query<(Entity, &GlobalTransform, &Sprite, &Visible), With<Position>>,
) {
    // get the primary window
    let wnd = wnds.get_primary().unwrap();

    // check if the cursor is in the primary window
    if let Some(pos) = wnd.cursor_position() {
        // get the size of the window
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // assuming there is exactly one main camera entity, so this is OK
        let camera_transform = q_camera.single().unwrap();

        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        // eprintln!("World coords: {}/{}", pos_wld.x, pos_wld.y);

        let s = sprite_query
            .iter()
            .filter(|(_, gt, sprite, visible)| {
                let size = sprite.size;
                let x1 = gt.translation.x - size.x / 2.0;
                let x2 = gt.translation.x + size.x / 2.0;
                let y1 = gt.translation.y - size.y / 2.0;
                let y2 = gt.translation.y + size.y / 2.0;
                visible.is_visible
                    && x1 <= pos_wld.x
                    && pos_wld.x <= x2
                    && y1 <= pos_wld.y
                    && pos_wld.y <= y2
            })
            .max_by(|(_, a, _, _), (_, b, _, _)| {
                a.translation.z.partial_cmp(&b.translation.z).unwrap()
            })
            // .max_by_key(|(gt, _)| gt.translation.z);
            .map(|(e, _, _, _)| e);

        // Send Event
        ev_hovering.send(HoveringEvent(s));
        if let Some(e) = s {
            if mouse_button_input.just_pressed(MouseButton::Left) {
                ev_click.send(PositionClickedEvent(e));
            }
        }
    }
}

pub fn click_sprite(
    mut ev_click: EventReader<PositionClickedEvent>,
    mut ev_input: EventWriter<InputMapping>,
    query: Query<(Option<&Character>, Option<&Field>)>,
) {
    for e in ev_click.iter() {
        let entity = e.0;
        let (character, field) = query.get(entity).unwrap();
        if character.is_some() {
            ev_input.send(InputMapping::SelectCharacter(entity));
        } else if field.is_some() {
            ev_input.send(InputMapping::SelectField(entity));
        } else {
            error!("Entity do not have Chatacter or Field");
        }
    }
}

pub fn mouse_click_system(
    mouse_button_input: Res<Input<MouseButton>>,
    mut ev_input: EventWriter<InputMapping>,
) {
    if mouse_button_input.just_pressed(MouseButton::Right) {
        ev_input.send(InputMapping::Cancel);
    }
}

pub fn keyboard_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut ev_input: EventWriter<InputMapping>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        ev_input.send(InputMapping::EndTurn);
    }
}
