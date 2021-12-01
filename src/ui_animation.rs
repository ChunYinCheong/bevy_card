use bevy::prelude::*;

pub struct UiAnimationPlugin;
impl Plugin for UiAnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<UiAninationRequestEvent>()
            .insert_resource(UiAnimationTracker::default())
            .add_system(init_animation.system())
            .add_system(animation_system.system());
    }
}

struct EffectAnimation {
    timer: Timer,
    index: usize,
    inited: bool,
}

impl EffectAnimation {
    fn new() -> Self {
        Self {
            // timer: Timer::from_seconds(0.04, true),
            timer: Timer::from_seconds(0.08, true),
            index: 0,
            inited: false,
        }
    }
}
#[derive(Default)]
pub struct UiAnimationTracker {
    pub entities: Vec<Entity>,
}
pub struct UiAninationRequestEvent;

fn init_animation(
    mut ev: EventReader<UiAninationRequestEvent>,
    mut tracker: ResMut<UiAnimationTracker>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for e in ev.iter() {
        let entity = commands
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    align_content: AlignContent::Center,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        // bottom: Val::Px(0.0),
                        // right: Val::Px(0.0),
                        // left: Val::Px(0.0),
                        // top: Val::Px(0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                // Use the `Text::with_section` constructor
                text: Text::with_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 100.0,
                        color: Color::WHITE,
                    },
                    // Note: You can use `Default::default()` in place of the `TextAlignment`
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..Default::default()
                    },
                ),
                transform: Transform::from_xyz(0.0, 0.0, 100.0),
                ..Default::default()
            })
            .insert(EffectAnimation::new())
            .id();

        tracker.entities.push(entity);
    }
}

fn animation_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut EffectAnimation, &mut Text)>,
    mut tracker: ResMut<UiAnimationTracker>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for (entity, mut anim, mut text) in query.iter_mut() {
        anim.timer.tick(time.delta());
        if anim.timer.finished() {
            anim.index += 1;
            if anim.index < 10 {
                // *material = materials.add(anim.textures[anim.index].clone().into());
                text.sections[0].value = format!("Change {}", anim.index);
            } else {
                commands.entity(entity).despawn();
                tracker.entities.retain(|e| *e != entity);
            }
        }
    }
}
