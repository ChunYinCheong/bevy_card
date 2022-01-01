use bevy::prelude::*;

use crate::game::Battle;

pub struct AnimationPlugin;
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AninationRequestEvent>()
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

pub struct AninationRequestEvent {
    pub texture: Vec<&'static str>,
    pub character: Entity,
}

fn init_animation(
    mut ev: EventReader<AninationRequestEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut battle: ResMut<Option<Battle>>,
) {
    if let Some(battle) = battle.as_mut() {
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
            battle.animations.push(entity);
        }
    }
}

fn animation_system(
    time: Res<Time>,
    mut query: Query<(Entity, &mut EffectAnimation, &mut Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    mut battle: ResMut<Option<Battle>>,
) {
    if let Some(battle) = battle.as_mut() {
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
                    battle.animations.retain(|e| *e != entity);
                }
            }
        }
    }
}
