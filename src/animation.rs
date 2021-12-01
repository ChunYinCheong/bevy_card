use bevy::prelude::*;

pub struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AninationRequestEvent>()
            .insert_resource(AnimationTracker::default())
            .add_system(init_animation.system())
            .add_system(animation_system.system());
    }
}

pub struct EffectAnimation {
    timer: Timer,
    index: usize,
    textures: Vec<Handle<Texture>>,
    inited: bool,
}

impl EffectAnimation {
    fn new(textures: Vec<Handle<Texture>>) -> Self {
        Self {
            // timer: Timer::from_seconds(0.04, true),
            timer: Timer::from_seconds(0.08, true),
            index: 0,
            textures,
            inited: false,
        }
    }
}
#[derive(Default)]
pub struct AnimationTracker {
    pub entities: Vec<Entity>,
}
pub struct AninationRequestEvent {
    pub texture: Vec<&'static str>,
    pub character: Entity,
}

fn init_animation(
    mut ev: EventReader<AninationRequestEvent>,
    mut tracker: ResMut<AnimationTracker>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for e in ev.iter() {
        let entity = commands
            .spawn()
            .insert(EffectAnimation::new(
                e.texture.iter().map(|&s| asset_server.load(s)).collect(),
            ))
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    size: Vec2::new(90.0, 90.0),
                    resize_mode: SpriteResizeMode::Manual,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 2.0),
                ..Default::default()
            })
            .id();
        commands.entity(e.character).push_children(&[entity]);
        tracker.entities.push(entity);
    }
}
fn animation_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut EffectAnimation, &mut Handle<ColorMaterial>)>,
    mut tracker: ResMut<AnimationTracker>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for (entity, mut anim, mut material) in query.iter_mut() {
        if !anim.inited {
            if !anim.textures.is_empty() {
                *material = materials.add(anim.textures[0].clone().into());
            }
            anim.inited = true;
        }
        anim.timer.tick(time.delta());
        if anim.timer.finished() {
            anim.index += 1;
            if anim.index < anim.textures.len() {
                *material = materials.add(anim.textures[anim.index].clone().into());
            } else {
                commands.entity(entity).despawn();
                tracker.entities.retain(|e| *e != entity);
            }
        }
    }
}
