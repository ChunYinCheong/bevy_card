use bevy::prelude::*;

use crate::{
    game::components::{
        ApText, CardInstance, CardName, CardSprite, Cell, HighlightSprite, HpText, Position,
        SelectType, Selectable, UnitInstance,
    },
    game::{
        components::{AtkText, Board, DefText, PlayerInstance, PlayerInstanceRef},
        systems::HighlightColor,
    },
};

// pub fn update_hp_text(
//     card_query: Query<&Card, Changed<Card>>,
//     mut text_query: Query<(&mut Text, &Parent), With<HpText>>,
// ) {
//     for (mut text, parent) in text_query.iter_mut() {
//         if let Ok(card) = card_query.get(parent.0) {
//             if let CardType::Monster(character) = &card.card_type {
//                 text.sections[0].value = character.hp.to_string();
//             }
//         }
//     }
// }

pub fn update_hp_text(
    card_query: Query<(&UnitInstance, &HpText), Changed<UnitInstance>>,
    mut text_query: Query<&mut Text>,
) {
    for (unit, text_id) in card_query.iter() {
        if let Ok(mut text) = text_query.get_mut(text_id.0) {
            text.sections[0].value = unit.hp.to_string();
        }
    }
}
pub fn update_ap_text(
    card_query: Query<(&UnitInstance, &ApText), Changed<UnitInstance>>,
    mut text_query: Query<&mut Text>,
) {
    for (unit, text_id) in card_query.iter() {
        if let Ok(mut text) = text_query.get_mut(text_id.0) {
            text.sections[0].value = unit.ap.to_string();
        }
    }
}
pub fn update_atk_text(
    card_query: Query<(&UnitInstance, &AtkText), Changed<UnitInstance>>,
    mut text_query: Query<&mut Text>,
) {
    for (unit, text_id) in card_query.iter() {
        if let Ok(mut text) = text_query.get_mut(text_id.0) {
            text.sections[0].value = unit.atk.to_string();
        }
    }
}
pub fn update_def_text(
    card_query: Query<(&UnitInstance, &DefText), Changed<UnitInstance>>,
    mut text_query: Query<&mut Text>,
) {
    for (unit, text_id) in card_query.iter() {
        if let Ok(mut text) = text_query.get_mut(text_id.0) {
            text.sections[0].value = unit.def.to_string();
        }
    }
}

pub fn update_transform_by_position(
    mut query: Query<(&Position, &mut Transform), Changed<Position>>,
) {
    for (pos, mut tranform) in query.iter_mut() {
        tranform.translation = pos.xyz();
    }
}

pub fn attach_graphic_to_card(
    mut commamds: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &CardInstance, &Position), Added<CardInstance>>,
) {
    let size = Vec2::new(95.0, 95.0);
    for (entity, card, position) in query.iter() {
        {
            let child_id = commamds
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        format!("{}", card.name.clone()),
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
                    transform: Transform::from_xyz(0.0, 0.0, 0.2),
                    ..Default::default()
                })
                .id();
            commamds
                .entity(entity)
                .insert(CardName(child_id))
                .push_children(&[child_id]);
        }
        {
            let child_id = commamds
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        size,
                        resize_mode: SpriteResizeMode::Manual,
                        ..Default::default()
                    },
                    material: materials.add(
                        asset_server
                            .load("images/Soldier/Soldier1/Walking/1.png")
                            // .load("images/particlePack_1.1/PNG (Transparent)/dirt_03.png")
                            .into(),
                    ),
                    transform: Transform::from_xyz(0.0, 0.0, 0.1),
                    ..Default::default()
                })
                .id();
            commamds
                .entity(entity)
                .insert(CardSprite(child_id))
                .push_children(&[child_id]);
        }
        {
            let child_id = commamds
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        size,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 0.3),
                    ..Default::default()
                })
                .id();
            commamds
                .entity(entity)
                .insert(HighlightSprite(child_id))
                .push_children(&[child_id]);
        }
        commamds
            .entity(entity)
            .insert_bundle((
                Transform {
                    translation: position.xyz(),
                    ..Default::default()
                },
                GlobalTransform::default(),
            ))
            .insert(Selectable {
                size,
                select_type: SelectType::Card,
            })
            .insert(HighlightColor::None);
    }
}

pub fn attach_graphic_to_unit(
    mut commamds: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &UnitInstance), Added<UnitInstance>>,
) {
    for (entity, unit) in query.iter() {
        {
            let child_id = commamds
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        format!("{}", unit.hp),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::GREEN,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    transform: Transform::from_xyz(0.0, 40.0, 0.2),
                    ..Default::default()
                })
                .id();
            commamds
                .entity(entity)
                .insert(HpText(child_id))
                .push_children(&[child_id]);
        }
        {
            let child_id = commamds
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        format!("{}", unit.ap),
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
                    transform: Transform::from_xyz(0.0, -40.0, 0.2),
                    ..Default::default()
                })
                .id();
            commamds
                .entity(entity)
                .insert(ApText(child_id))
                .push_children(&[child_id]);
        }
        {
            let child_id = commamds
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        format!("{}", unit.atk),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::RED,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    transform: Transform::from_xyz(-40.0, -40.0, 0.2),
                    ..Default::default()
                })
                .id();
            commamds
                .entity(entity)
                .insert(AtkText(child_id))
                .push_children(&[child_id]);
        }
        {
            let child_id = commamds
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        format!("{}", unit.def),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::BLUE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    transform: Transform::from_xyz(40.0, -40.0, 0.2),
                    ..Default::default()
                })
                .id();
            commamds
                .entity(entity)
                .insert(DefText(child_id))
                .push_children(&[child_id]);
        }
        // {
        //     let child_id = commamds
        //         .spawn_bundle(SpriteBundle {
        //             sprite: Sprite {
        //                 size: Vec2::new(75.0, 75.0),
        //                 ..Default::default()
        //             },
        //             transform: Transform::from_xyz(0.0, 0.0, 0.3),
        //             ..Default::default()
        //         })
        //         .id();
        //     commamds
        //         .entity(entity)
        //         .insert(HighlightSprite(child_id))
        //         .push_children(&[child_id]);
        // }
        // commamds
        //     .entity(entity)
        //     .insert_bundle((
        //         Transform {
        //             translation: {
        //                 let mut t = position.xyz();
        //                 t.z += 1.0;
        //                 t
        //             },
        //             ..Default::default()
        //         },
        //         GlobalTransform::default(),
        //     ))
        //     .insert(Selectable {
        //         size: Vec2::new(75.0, 75.0),
        //         select_type: SelectType::Unit,
        //     })
        //     .insert(HighlightColor::None);
    }
}

pub fn attach_graphic_to_cell(
    mut commamds: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &Cell, &Position), Added<Cell>>,
) {
    for (entity, _, position) in query.iter() {
        {
            let child_id = commamds
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        size: Vec2::new(95.0, 95.0),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 0.3),
                    ..Default::default()
                })
                .id();
            commamds
                .entity(entity)
                .insert(HighlightSprite(child_id))
                .push_children(&[child_id]);
        }
        commamds
            .entity(entity)
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    size: Vec2::new(95.0, 95.0),
                    resize_mode: SpriteResizeMode::Manual,
                    ..Default::default()
                },
                material: materials.add(Color::rgb(0.5, 0.5, 0.9).into()),
                transform: Transform {
                    translation: {
                        let mut pos = position.xyz();
                        pos.z = 0.0;
                        pos
                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            // .insert_bundle((
            //     Transform {
            //         translation: position.xyz(),
            //         ..Default::default()
            //     },
            //     GlobalTransform::default(),
            // ))
            .insert(Selectable {
                size: Vec2::new(95.0, 95.0),
                select_type: SelectType::Cell,
            })
            .insert(HighlightColor::None);
    }
}

pub fn attach_graphic_to_board(
    mut commamds: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Board, &PlayerInstanceRef), Added<Board>>,
    player_query: Query<&PlayerInstance>,
) {
    for (_, _, player) in query.iter() {
        let player = player_query.get(player.0).unwrap();
        {
            let _child_id = commamds
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(
                        format!("MP: {}", player.mp),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::GREEN,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    transform: Transform::from_xyz(-300.0, -100.0, 0.2),
                    ..Default::default()
                })
                .id();
            // commamds
            //     .entity(entity)
            //     .insert(ApText(child_id))
            //     .push_children(&[child_id]);
        }
    }
}
