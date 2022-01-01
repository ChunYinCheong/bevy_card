use bevy::ecs::archetype::Archetypes;
use bevy::ecs::component::Components;
use bevy::ecs::entity::Entities;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::{InspectableRegistry, WorldInspectorPlugin};

use animation::animation::AnimationPlugin;
use animation::ui_animation::UiAnimationPlugin;
use game::GamePlugin;

mod animation;
mod game;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Lobby,
    Battle,
    BattleEnd,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Card Game".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        // Ui
        .add_plugin(EguiPlugin)
        // Inspector
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(
            InspectableRegistry::default()
                .with::<game::components::Ability>()
                .with::<game::components::AbilityInstance>()
                .with::<game::components::UnitInstanceRef>()
                .with::<game::components::CardDataId>()
                .with::<game::components::UnitDataId>()
                .with::<game::components::Position>()
                .with::<game::components::PlayerController>()
                .with::<game::components::CardName>()
                .with::<game::components::HpText>()
                .with::<game::components::ApText>()
                .with::<game::components::CardSprite>(),
        )
        .add_state(AppState::MainMenu)
        .add_startup_system(game::load_game.system())
        .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(main_menu.system()))
        .add_system_set(SystemSet::on_update(AppState::Lobby).with_system(lobby_menu.system()))
        .add_system_set(
            SystemSet::on_enter(AppState::Battle).with_system(game::start_battle.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::Battle).with_system(game::cleanup_battle.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::BattleEnd).with_system(back_to_lobby.system()),
        )
        .add_plugin(AnimationPlugin)
        .add_plugin(UiAnimationPlugin)
        .add_plugin(GamePlugin)
        .add_system(inspect.system())
        .run();
}

fn inspect(
    keyboard: Res<Input<KeyCode>>,
    all_entities: Query<Entity>,
    entities: &Entities,
    archetypes: &Archetypes,
    components: &Components,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        for entity in all_entities.iter() {
            println!("Entity: {:?}", entity);
            if let Some(entity_location) = entities.get(entity) {
                if let Some(archetype) = archetypes.get(entity_location.archetype_id) {
                    for component in archetype.components() {
                        if let Some(info) = components.get_info(component) {
                            println!("\tComponent: {}", info.name());
                        }
                    }
                }
            }
        }
    }
}

fn main_menu(egui_context: ResMut<EguiContext>, mut app_state: ResMut<State<AppState>>) {
    egui::Window::new("Main Menu").show(egui_context.ctx(), |ui| {
        if ui.button("Start").clicked() {
            app_state.set(AppState::Lobby).unwrap();
        }
        if ui.button("Hello").clicked() {
            println!("Hello");
        }
        if ui.button("Quit").clicked() {
            std::process::exit(0);
        }
    });
}

fn lobby_menu(egui_context: ResMut<EguiContext>, mut app_state: ResMut<State<AppState>>) {
    egui::Window::new("Lobby Menu").show(egui_context.ctx(), |ui| {
        if ui.button("Battle").clicked() {
            app_state.set(AppState::Battle).unwrap();
        }
        if ui.button("Deck").clicked() {
            println!("Deck");
        }
        if ui.button("Back to Main Menu").clicked() {
            app_state.set(AppState::MainMenu).unwrap();
        }
    });
}

fn back_to_lobby(mut app_state: ResMut<State<AppState>>) {
    println!("Back to lobby");
    app_state.set(AppState::Lobby).unwrap();
}
