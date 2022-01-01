mod systems;

pub mod components;
pub mod events;

pub use bevy::ecs::entity::Entity;
use bevy::prelude::*;

pub use systems::cleanup_battle;
pub use systems::load_game;
pub use systems::start_battle;

pub use systems::handle_selection_change;
pub use systems::InputState;
pub use systems::SelectedEntity;

pub use systems::HoveringEntity;

pub use self::systems::Battle;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_camera.system())
            // battle
            .init_resource::<Option<Battle>>()
            .add_event::<events::BattleEvent>()
            .insert_resource(systems::EventState::Init)
            .insert_resource(systems::BattleFlow::PreTurnStart)
            .add_system(systems::flow_systme.system())
            .add_system(systems::update.system())
            .add_event::<events::PreTurnStart>()
            .add_system(systems::pre_turn_start_system.system())
            .add_event::<events::UnitHurt>()
            .add_system(systems::unit_hurt_system.system())
            .add_event::<events::PlayerDraw>()
            .add_system(systems::player_draw_system.system())
            .add_event::<events::SummonUnit>()
            .add_system(systems::summon_unit_system.system())
            .add_event::<events::UnitDie>()
            .add_system(systems::unit_die_system.system())
            // graphics
            .add_system(systems::attach_graphic_to_board.system())
            .add_system(systems::attach_graphic_to_card.system())
            .add_system(systems::attach_graphic_to_cell.system())
            .add_system(systems::attach_graphic_to_unit.system())
            .add_system(systems::update_ap_text.system())
            .add_system(systems::update_hp_text.system())
            .add_system(systems::update_atk_text.system())
            .add_system(systems::update_def_text.system())
            .add_system(systems::update_transform_by_position.system())
            // battle ui
            .add_startup_system(systems::setup_ui.system())
            .add_system(systems::update_ui_system.system())
            .init_resource::<systems::ButtonMaterials>()
            .add_system(systems::button_system.system())
            // cursor
            .init_resource::<systems::HoveringEntity>()
            .add_event::<events::HoverChangedEvent>()
            .add_system(systems::cursor_system.system())
            // input
            .add_event::<events::InputMappingEvent>()
            .add_system(systems::mouse_click_system.system())
            .add_system(systems::keyboard_system.system())
            .add_system(systems::input_event_system.system())
            // player action
            .add_event::<events::PlayerActionEvent>()
            .add_system(systems::player_event_system.system())
            // input state
            .add_system(systems::handle_selection_change.system())
            .add_event::<events::SelectEntityEvent>()
            .add_event::<events::SelectionChangedEvent>()
            .init_resource::<SelectedEntity>()
            .insert_resource::<InputState>(InputState::InTurn)
            // selection
            .add_system(systems::selection_change.system())
            .add_system(systems::hovering_system.system())
            // highlight
            .init_resource::<systems::SelectionBoxColorMaterials>()
            .add_system(systems::update_highlight_color.system());
    }
}
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

// Data:
// CardData, UnitData, AbilityData

// Lobby/Normal:
// Player => name, money, decks, cards, exp, ability, ...
// Card => use_count, win/lose, etc...
// Unit => dead_count, level, exp, etc...
// Item => level, upgrade, etc...
// Ability => level, exp, use_count, ... TODO!

// In Battle:
// Player Instance => name, mp,  ...
// Card Instance (Unit Instance) => Copy Data(Unit, Item, Magic)
// Board(Transform) => display
// Ability Instance => UnitInstanceRef, ability data...

// Graphic
// Show player hands, decks, graves, boards, ...

// TODO: fix z in transform, +z, 0 is bottom

// Player Instance Entity: PlayerInstance, PlayerController

// Card Instance Entity: CardInstance, Position, CardName, CardSprite, HighlightSprite, HighlightColor, Selectable, Transform, GlobalTransform
// Children:
// CardName Entity: Text2dBundle
// CardSprite Entity: SpriteBundle
// HighlightSprite Entity: SpriteBundle

// Unit Instance Entity(Card Instance Entity): UnitInstance, HpText, ApText, AtkText, DefText
// Children:
// HpText Entity: Text2dBundle
// ApText Entity: Text2dBundle
// HighlightSprite Entity: SpriteBundle

// Ability Instance Entity: AbilityInstance, UnitInstanceRef(/Item/????)

// Cell Entity: Cell, Position, SpriteBundle, HighlightSprite, HighlightColor, Selectable
// HighlightSprite Entity: SpriteBundle

// Board Entity: Board, PlayerInstanceRef, Transform, GlobalTransform
// Children: Player Instance Entity, Card Instance Entity(Unit Instance), Cell Entity
// Children: MpText
