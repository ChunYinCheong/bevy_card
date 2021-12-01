use crate::game::{Character, Game, Owner, Player, Position, ENEMY_INDEX, PLAYER_INDEX};
use animation::AnimationPlugin;
use battle::BattleActionPlugin;
use battle::BattlePlugin;
use battle::BattleUiPlugin;
use bevy::prelude::*;
use game::Field;
use input::GameInputPlugin;
use selection::SelectionPlugin;
use selection_box::{SelectionBoxColor, SelectionBoxPlugin};
use ui_animation::UiAnimationPlugin;

mod animation;
mod battle;
mod game;
mod input;
mod selection;
mod selection_box;
mod ui_animation;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            // title: "Bevy".to_string(),
            // width: 1024.0,
            // height: 768.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GameInputPlugin)
        .add_plugin(SelectionPlugin)
        .add_plugin(BattlePlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(UiAnimationPlugin)
        .add_startup_system(setup_game.system())
        .add_startup_system(setup_2d.system())
        .add_system(update_transform_by_position.system())
        .add_system(update_character_text.system())
        // .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugin(BattleUiPlugin)
        .add_plugin(BattleActionPlugin)
        .add_plugin(SelectionBoxPlugin)
        .run();
}
pub struct MainCamera;
fn setup_2d(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    for player_index in 0..2 {
        for x in 0..3 {
            for y in 0..3 {
                let pos = Position { player_index, x, y };
                let (x, y, _z) = pos.xyz();
                let transform = Transform::from_xyz(x, y, 0.0);
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            size: Vec2::new(90.0, 90.0),
                            ..Default::default()
                        },
                        transform,
                        material: materials.add(Color::rgb(0.8, 0.0, 0.0).into()),
                        ..Default::default()
                    })
                    .insert(Field)
                    .insert(pos);
            }
        }
    }
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let c = Character {
        name: "Soldier".to_string(),
        description: "A soldier".to_string(),
        hp: 5,
        attack: 1,
        defence: 0,
        action_point: 0,
        abilities: vec![],
        status: Default::default(),
        is_dead: false,
    };
    // character 00
    spawn_character(
        &mut commands,
        &asset_server,
        &mut materials,
        c.clone(),
        0,
        0,
        PLAYER_INDEX,
    );
    // character 01
    spawn_character(
        &mut commands,
        &asset_server,
        &mut materials,
        c.clone(),
        1,
        0,
        PLAYER_INDEX,
    );
    // character 02
    spawn_character(
        &mut commands,
        &asset_server,
        &mut materials,
        c.clone(),
        2,
        0,
        PLAYER_INDEX,
    );
    // enemy character 01
    spawn_character(
        &mut commands,
        &asset_server,
        &mut materials,
        c.clone(),
        1,
        0,
        ENEMY_INDEX,
    );
    // Player
    let player = spawn_character(
        &mut commands,
        &asset_server,
        &mut materials,
        Character {
            name: "You".to_string(),
            description: "you".to_string(),
            hp: 5,
            attack: 1,
            defence: 0,
            action_point: 0,
            abilities: vec![],
            status: Default::default(),
            is_dead: false,
        },
        1,
        1,
        PLAYER_INDEX,
    );
    commands.entity(player).insert(Player {
        index: PLAYER_INDEX,
        name: "Player".to_string(),
    });

    let enemy = spawn_character(
        &mut commands,
        &asset_server,
        &mut materials,
        Character {
            name: "Demon".to_string(),
            description: "A demon".to_string(),
            hp: 5,
            attack: 1,
            defence: 0,
            action_point: 0,
            abilities: vec![],
            status: Default::default(),
            is_dead: false,
        },
        1,
        1,
        ENEMY_INDEX,
    );
    commands.entity(enemy).insert(Player {
        index: ENEMY_INDEX,
        name: "Enemy".to_string(),
    });

    // Game
    let game = Game {
        players: vec![player, enemy],
        current_index: 0,
    };

    commands.insert_resource(game);
}

struct CharacterChildren {
    pub name_entity: Entity,
    pub hp_entity: Entity,
    pub sprite_entity: Entity,
    pub ap_entity: Entity,
}
fn spawn_character(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    character: Character,
    x: usize,
    y: usize,
    player_index: usize,
) -> Entity {
    let position = Position { player_index, x, y };
    let (x, y, z) = position.xyz();
    let transform = Transform::from_xyz(x, y, 100.0);

    let name_entity = commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                format!("{}", character.name.clone()),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            transform: Transform::from_xyz(0.0, 0.0, 2.0),
            ..Default::default()
        })
        .id();
    let hp_entity = commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                format!("{}", character.hp),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::GREEN,
                },
                TextAlignment {
                    vertical: VerticalAlign::Bottom,
                    horizontal: HorizontalAlign::Right,
                },
            ),
            transform: Transform::from_xyz(-45.0, 45.0, 2.0),
            ..Default::default()
        })
        .id();
    let ap_entity = commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                format!("{}", character.action_point),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::GREEN,
                },
                TextAlignment {
                    vertical: VerticalAlign::Bottom,
                    horizontal: HorizontalAlign::Left,
                },
            ),
            transform: Transform::from_xyz(45.0, 45.0, 2.0),
            ..Default::default()
        })
        .id();
    let sprite_entity = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                size: Vec2::new(90.0, 90.0),
                resize_mode: SpriteResizeMode::Manual,
                ..Default::default()
            },
            material: materials.add(
                asset_server
                    .load("images/Soldier/Soldier1/Walking/1.png")
                    .into(),
            ),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .id();
    let entity = commands
        .spawn()
        .insert(character)
        .insert(position)
        .insert(Owner { player_index })
        .insert(transform)
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                size: Vec2::new(95.0, 95.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(transform)
        .insert(SelectionBoxColor::None)
        .insert(CharacterChildren {
            name_entity,
            hp_entity,
            sprite_entity,
            ap_entity,
        })
        .push_children(&[name_entity, hp_entity, sprite_entity, ap_entity])
        .id();
    entity
}

fn update_transform_by_position(mut query: Query<(&Position, &mut Transform), Changed<Position>>) {
    for (pos, mut tranform) in query.iter_mut() {
        let (x, y, z) = pos.xyz();
        tranform.translation.x = x;
        tranform.translation.y = y;
        // tranform.translation.z = z;
    }
}

fn update_character_text(
    mut character_query: Query<(&CharacterChildren, &Character), Changed<Character>>,
    mut text_query: Query<&mut Text>,
) {
    for (children, character) in character_query.iter_mut() {
        let mut text = text_query.get_mut(children.hp_entity).unwrap();
        text.sections[0].value = character.hp.to_string();
        let mut text = text_query.get_mut(children.ap_entity).unwrap();
        text.sections[0].value = character.action_point.to_string();
    }
}
