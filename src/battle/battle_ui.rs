use crate::{
    game::{Character, Position},
    input::{HoverChanged, PlayerActionEvent},
};
use bevy::prelude::*;

pub struct BattleUiPlugin;
impl Plugin for BattleUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_ui.system())
            .add_system(update_ui_system.system())
            .init_resource::<ButtonMaterials>()
            .add_system(button_system.system());
    }
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
    mut ev_player: EventWriter<PlayerActionEvent>,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                // text.sections[0].value = "Press".to_string();
                *material = button_materials.pressed.clone();
                ev_player.send(PlayerActionEvent::EndTurn);
            }
            Interaction::Hovered => {
                // text.sections[0].value = "Hover".to_string();
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                // text.sections[0].value = "Button".to_string();
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
) {
    // ui camera
    commands.spawn_bundle(UiCameraBundle::default());

    // Show detail
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|builder| {
            builder
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "Text",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.1, 0.1),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                })
                .insert(DetailUi);
        });

    // Button
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "End Turn",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });
}

struct DetailUi;
fn update_ui_system(
    mut ev_hover_changed: EventReader<HoverChanged>,
    mut query: Query<(&mut Text, &DetailUi)>,
    character_query: Query<(Option<&Character>, Option<&Position>)>,
) {
    let (mut text, _) = query.single_mut().unwrap();
    for e in ev_hover_changed.iter() {
        match &e.1 {
            Some(e) => {
                let (character, field) = character_query.get(*e).unwrap();
                match character {
                    Some(character) => {
                        text.sections[0].value = format!(
                            "{} {} {}",
                            character.name, character.hp, character.action_point
                        );
                    }
                    None => match field {
                        Some(field) => {
                            text.sections[0].value = format!(
                                "Field - player_index: {}, x: {}, y: {}",
                                field.player_index, field.x, field.y
                            )
                        }
                        None => {
                            error!("No Character or Field");
                        }
                    },
                }
            }
            None => {
                text.sections[0].value = "Nothing".to_string();
            }
        }
    }
}
