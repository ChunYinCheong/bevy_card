use bevy::prelude::*;

use crate::game::components::HighlightSprite;

#[derive(Debug)]
pub enum HighlightColor {
    None,
    Green,
    Yellow,
    GreenYellow,
    Red,
}
impl Default for HighlightColor {
    fn default() -> Self {
        HighlightColor::None
    }
}

pub struct SelectionBoxColorMaterials {
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
            none: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0).into()),
            green: materials.add(Color::rgba(0.0, 0.9, 0.0, 0.3).into()),
            yellow: materials.add(Color::rgba(0.9, 0.9, 0.0, 0.3).into()),
            green_yellow: materials.add(Color::rgba(0.5, 0.9, 0.0, 0.3).into()),
            red: materials.add(Color::rgba(0.9, 0.0, 0.0, 0.3).into()),
        }
    }
}

pub fn update_highlight_color(
    character_color_materials: Res<SelectionBoxColorMaterials>,
    mut query: Query<(&HighlightColor, &HighlightSprite), Changed<HighlightColor>>,
    mut query_m: Query<&mut Handle<ColorMaterial>>,
) {
    for (color, entity) in query.iter_mut() {
        let mut material = query_m.get_mut(entity.0).unwrap();
        match color {
            HighlightColor::None => *material = character_color_materials.none.clone(),
            HighlightColor::Green => *material = character_color_materials.green.clone(),
            HighlightColor::Yellow => *material = character_color_materials.yellow.clone(),
            HighlightColor::GreenYellow => {
                *material = character_color_materials.green_yellow.clone()
            }
            HighlightColor::Red => *material = character_color_materials.red.clone(),
        }
    }
}
