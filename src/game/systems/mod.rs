mod battle;
mod battle_action;
mod battle_setup;
mod battle_ui;
mod cursor;
mod game;
mod graphics;
mod highlight;
mod input;
mod input_state;
mod selection;

pub use graphics::attach_graphic_to_board;
pub use graphics::attach_graphic_to_card;
pub use graphics::attach_graphic_to_cell;
pub use graphics::attach_graphic_to_unit;
pub use graphics::update_ap_text;
pub use graphics::update_atk_text;
pub use graphics::update_def_text;
pub use graphics::update_hp_text;
pub use graphics::update_transform_by_position;

pub use battle_setup::cleanup_battle;
pub use battle_setup::start_battle;

pub use game::load_game;

pub use cursor::cursor_system;
pub use cursor::HoveringEntity;

pub use input::keyboard_system;
pub use input::mouse_click_system;

pub use input_state::handle_selection_change;
pub use input_state::input_event_system;
pub use input_state::InputState;
pub use input_state::SelectedEntity;

pub use battle_action::player_event_system;

pub use battle_ui::button_system;
pub use battle_ui::setup_ui;
pub use battle_ui::update_ui_system;
pub use battle_ui::ButtonMaterials;
pub use battle_ui::DetailUi;

pub use selection::hovering_system;
pub use selection::selection_change;

pub use highlight::update_highlight_color;
pub use highlight::HighlightColor;
pub use highlight::SelectionBoxColorMaterials;

pub use battle::flow_systme;
pub use battle::player_draw_system;
pub use battle::pre_turn_start_system;
pub use battle::summon_unit_system;
pub use battle::unit_die_system;
pub use battle::unit_hurt_system;
pub use battle::update;
pub use battle::Battle;
pub use battle::BattleFlow;
pub use battle::EventState;
