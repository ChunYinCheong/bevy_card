use bevy::prelude::*;

pub struct SelectionBoxPlugin;
impl Plugin for SelectionBoxPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectionBoxColorMaterials>()
            .add_system(update_selection_box_color.system());
    }
}

#[derive(Debug, Default)]
pub struct SelectionBox;

#[derive(Debug)]
pub enum SelectionBoxColor {
    None,
    Green,
    Yellow,
    GreenYellow,
    Red,
}
impl Default for SelectionBoxColor {
    fn default() -> Self {
        SelectionBoxColor::None
    }
}

struct SelectionBoxColorMaterials {
    none: Handle<ColorMaterial>,
    green: Handle<ColorMaterial>,
    yellow: Handle<ColorMaterial>,
    green_yellow: Handle<ColorMaterial>,
    red: Handle<ColorMaterial>,
}

impl FromWorld for SelectionBoxColorMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        SelectionBoxColorMaterials {
            none: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
            green: materials.add(Color::rgb(0.0, 0.9, 0.0).into()),
            yellow: materials.add(Color::rgb(0.9, 0.9, 0.0).into()),
            green_yellow: materials.add(Color::rgb(0.5, 0.9, 0.0).into()),
            red: materials.add(Color::rgb(0.9, 0.0, 0.0).into()),
        }
    }
}

fn update_selection_box_color(
    character_color_materials: Res<SelectionBoxColorMaterials>,
    mut query: Query<(&mut Handle<ColorMaterial>, &SelectionBoxColor), Changed<SelectionBoxColor>>,
) {
    for (mut material, color) in query.iter_mut() {
        match color {
            SelectionBoxColor::None => *material = character_color_materials.none.clone(),
            SelectionBoxColor::Green => *material = character_color_materials.green.clone(),
            SelectionBoxColor::Yellow => *material = character_color_materials.yellow.clone(),
            SelectionBoxColor::GreenYellow => {
                *material = character_color_materials.green_yellow.clone()
            }
            SelectionBoxColor::Red => *material = character_color_materials.red.clone(),
        }
    }
}
