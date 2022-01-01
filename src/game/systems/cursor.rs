use bevy::{prelude::*, render::camera::Camera};

use crate::game::{components::Selectable, events::HoverChangedEvent};

#[derive(Debug, Clone, Default)]
pub struct HoveringEntity(pub Option<Entity>);

pub fn cursor_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    mut hovering: ResMut<HoveringEntity>,
    mut ev_hovering: EventWriter<HoverChangedEvent>,
    q_camera: Query<(&Transform, &Camera)>,
    sprite_query: Query<(Entity, &GlobalTransform, &Selectable)>,
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
        // let camera_transform = q_camera.single().unwrap();
        if let Some((camera_transform, _)) = q_camera
            .iter()
            .find(|(_, camera)| camera.name == Some("Camera2d".into()))
        {
            // apply the camera transform
            let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
            // eprintln!("World coords: {}/{}", pos_wld.x, pos_wld.y);

            let s = sprite_query
                .iter()
                .filter(|(_, gt, sprite)| {
                    let size = sprite.size;
                    let x1 = gt.translation.x - size.x / 2.0;
                    let x2 = gt.translation.x + size.x / 2.0;
                    let y1 = gt.translation.y - size.y / 2.0;
                    let y2 = gt.translation.y + size.y / 2.0;
                    x1 <= pos_wld.x && pos_wld.x <= x2 && y1 <= pos_wld.y && pos_wld.y <= y2
                })
                .max_by(|(_, a, _), (_, b, _)| {
                    a.translation.z.partial_cmp(&b.translation.z).unwrap()
                })
                // .max_by_key(|(gt, _)| gt.translation.z);
                .map(|(e, _, _)| e);

            // Event / Res update
            if hovering.0 != s {
                ev_hovering.send(HoverChangedEvent {
                    old: hovering.0,
                    new: s,
                });
            }
            hovering.0 = s;
        }
    }
}
