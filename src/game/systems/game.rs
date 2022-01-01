use bevy::prelude::*;

use crate::game::components::{
    Ability, AbilityDataId, Card, CardDataId, CardDataType, CardType, Player, Unit, UnitDataId,
};

fn spawn_ability(commands: &mut Commands, ability_data_id: AbilityDataId) -> Entity {
    let ability_id = commands
        .spawn()
        .insert(Ability::new(ability_data_id))
        .insert(Name::new("Ability"))
        .id();
    ability_id
}
fn spawn_unit(commands: &mut Commands, unit_data_id: UnitDataId) -> Entity {
    let unit_id = commands.spawn().id();
    let data = unit_data_id.unit_data();
    let abilities: Vec<_> = data
        .abilities
        .iter()
        .map(|&ability_data_id| spawn_ability(commands, ability_data_id))
        .collect();
    commands.entity(unit_id).push_children(&abilities[..]);
    commands
        .entity(unit_id)
        .insert(Unit::new(unit_data_id, abilities))
        .insert(Name::new("Unit"))
        .id();
    unit_id
}

fn spawn_card(commands: &mut Commands, card_data_id: CardDataId) -> Entity {
    let card_id = commands.spawn().id();
    let data = card_data_id.card_data();
    let card_type = match data.card_type {
        CardDataType::Unit(unit_data_id) => {
            let unit_id = spawn_unit(commands, unit_data_id);
            let card_type = CardType::Unit(unit_id);
            commands.entity(card_id).push_children(&[unit_id]);
            card_type
        }
    };
    commands
        .entity(card_id)
        .insert(Card::new(card_data_id, card_type))
        .insert(Name::new("Card"))
        .id();
    card_id
}

pub fn load_game(mut commands: Commands) {
    let game = commands.spawn().id();
    {
        // Load player
        let decks = vec![
            spawn_card(&mut commands, CardDataId::Solider),
            spawn_card(&mut commands, CardDataId::Solider),
        ];
        let cards = vec![
            spawn_card(&mut commands, CardDataId::Spare),
            spawn_card(&mut commands, CardDataId::Spare),
        ];
        let character_card = spawn_card(&mut commands, CardDataId::You);
        commands.entity(game).push_children(&decks[..]);
        commands.entity(game).push_children(&cards[..]);
        commands.entity(game).push_children(&[character_card]);
        let player = commands
            .spawn()
            .insert(Player {
                name: "You".into(),
                money: 100,
                decks,
                cards,
                character_card,
            })
            .insert(Name::new("Player"))
            .id();
        commands.entity(game).push_children(&[player]);
    }
    {
        // Load enemy
        let decks = vec![
            spawn_card(&mut commands, CardDataId::Demon),
            spawn_card(&mut commands, CardDataId::Demon),
        ];

        let character_card = spawn_card(&mut commands, CardDataId::Enemy);
        commands.entity(game).push_children(&decks[..]);
        commands.entity(game).push_children(&[character_card]);
        let player = commands
            .spawn()
            .insert(Player {
                name: "Enemy".into(),
                money: 100,
                decks,
                cards: vec![],
                character_card,
            })
            .insert(Name::new("Player"))
            .id();
        commands.entity(game).push_children(&[player]);
    }
}
