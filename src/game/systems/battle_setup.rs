use std::f32::consts::PI;

use bevy::prelude::*;

use crate::game::components::{
    Ability, AbilityInstance, Board, Card, CardInstance, CardInstanceType, CardType, Cell, Player,
    PlayerController, PlayerInstance, PlayerInstanceRef, Position, PositionType, Unit,
    UnitInstance, UnitInstanceRef,
};

use super::{Battle, InputState};

pub fn start_battle(
    mut commands: Commands,
    q: Query<Entity, With<Player>>,
    player_query: Query<&Player>,
    card_query: Query<&Card>,
    unit_query: Query<&Unit>,
    ability_query: Query<&Ability>,
) {
    let ids: Vec<Entity> = q.iter().collect();
    let player_id = ids[0];
    let enemy_id = ids[1];

    let (player, player_board) = load_player_to_battle(
        player_id,
        &mut commands,
        &player_query,
        &card_query,
        &unit_query,
        &ability_query,
    );
    commands.entity(player_board).insert(Transform {
        translation: Vec3::new(0.0, -150.0, 0.0),
        ..Default::default()
    });
    commands.entity(player).insert(PlayerController::Player);
    let (enemy, enemy_board) = load_player_to_battle(
        enemy_id,
        &mut commands,
        &player_query,
        &card_query,
        &unit_query,
        &ability_query,
    );
    commands.entity(enemy_board).insert(Transform {
        translation: Vec3::new(0.0, 150.0, 0.0),
        rotation: Quat::from_rotation_z(PI),
        ..Default::default()
    });
    commands.entity(enemy).insert(PlayerController::Ai);

    commands.insert_resource(Some(Battle {
        players: vec![player, enemy],
        boards: vec![player_board, enemy_board],
        ..Default::default()
    }));
}

pub fn load_player_to_battle(
    player_id: Entity,
    commands: &mut Commands,
    player_query: &Query<&Player>,
    card_query: &Query<&Card>,
    unit_query: &Query<&Unit>,
    ability_query: &Query<&Ability>,
) -> (Entity, Entity) {
    let player_instance_id = commands.spawn().id();
    let board = commands
        .spawn()
        .insert(Board)
        .insert(PlayerInstanceRef(player_instance_id))
        .insert_bundle((Transform::default(), GlobalTransform::default()))
        .insert(Name::new("Board"))
        .id();

    let player = player_query.get(player_id).unwrap();
    let decks: Vec<Entity> = player
        .decks
        .iter()
        .enumerate()
        .map(|(i, &card_id)| {
            spwan_card_instance(
                card_query,
                card_id,
                unit_query,
                commands,
                player_instance_id,
                Position {
                    x: 0,
                    y: 0,
                    z: i as i32,
                    face_up: false,
                    player_id: player_instance_id,
                    position_type: PositionType::Deck,
                },
                board,
                ability_query,
            )
        })
        .collect();
    commands.entity(board).push_children(&decks[..]);

    let player_card_instance = spwan_card_instance(
        card_query,
        player.character_card,
        unit_query,
        commands,
        player_instance_id,
        Position {
            x: 1,
            y: 1,
            z: 0,
            face_up: true,
            player_id: player_instance_id,
            position_type: PositionType::Board,
        },
        board,
        ability_query,
    );
    commands
        .entity(board)
        .push_children(&[player_card_instance]);

    commands
        .entity(player_instance_id)
        .insert(PlayerInstance {
            name: player.name.clone(),
            mp: 5,
            player_card_instance_id: player_card_instance,
        })
        .insert(Name::new(format!(
            "PlayerInstance Id: {:?}",
            player_instance_id
        )))
        .id();
    commands.entity(board).push_children(&[player_instance_id]);

    {
        commands.entity(board).with_children(|builder| {
            for y in 0..3 {
                for x in 0..3 {
                    builder
                        .spawn()
                        .insert(Cell)
                        .insert(Position {
                            x,
                            y,
                            z: 0,
                            face_up: true,
                            player_id: player_instance_id,
                            position_type: PositionType::Board,
                        })
                        .insert(Name::new("Cell"));
                }
            }
        });
    }
    (player_instance_id, board)
}

fn spwan_card_instance(
    card_query: &Query<&Card>,
    card_id: Entity,
    unit_query: &Query<&Unit>,
    commands: &mut Commands,
    player_instance_id: Entity,
    position: Position,
    board: Entity,
    ability_query: &Query<&Ability>,
) -> Entity {
    let card_instance_id = commands.spawn().id();

    let card = card_query.get(card_id).unwrap().clone();
    let card_type = match card.card_type {
        CardType::Unit(unit_id) => {
            let unit_instance_id = card_instance_id;
            let unit = unit_query.get(unit_id).unwrap().clone();
            let abilities: Vec<_> = unit
                .abilities
                .iter()
                .map(|&ability_id| {
                    spwan_ability_instance(
                        commands,
                        ability_query,
                        ability_id,
                        unit_id,
                        unit_instance_id,
                    )
                })
                .collect();
            commands
                .entity(unit_instance_id)
                .push_children(&abilities[..]);
            let unit_instance = UnitInstance::new(unit_id, unit, player_instance_id, abilities);
            commands
                .entity(unit_instance_id)
                .insert(unit_instance)
                // .insert(position)
                // .insert(Name::new(format!(
                //     "UnitInstance Id: {:?}",
                //     unit_instance_id
                // )))
                .id();
            // commands
            //     .entity(card_instance_id)
            //     .push_children(&[unit_instance_id]);
            CardInstanceType::UnitInstance(unit_instance_id)
        }
    };
    let card_instance = CardInstance::new(card_id, card, card_type);
    commands
        .entity(card_instance_id)
        .insert(card_instance)
        .insert(position)
        .insert(Name::new(format!(
            "CardInstance Id: {:?}",
            card_instance_id
        )))
        .id();
    card_instance_id
}

fn spwan_ability_instance(
    commands: &mut Commands,
    ability_query: &Query<&Ability>,
    ability_id: Entity,
    unit_id: Entity,
    unit_instance_id: Entity,
) -> Entity {
    let ability_instance_id = commands.spawn().id();
    let ability = ability_query.get(ability_id).unwrap().clone();

    let ability_instance = AbilityInstance::new(ability_id, ability);
    commands
        .entity(ability_instance_id)
        .insert(ability_instance)
        .insert(UnitInstanceRef(unit_instance_id))
        .insert(Name::new(format!(
            "AbilityInstance Id: {:?}",
            ability_instance_id
        )))
        .id();
    ability_instance_id
}

pub fn cleanup_battle(mut commands: Commands, query: Query<Entity, With<Board>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    commands.insert_resource(InputState::InTurn);
    commands.insert_resource::<Option<Battle>>(None);
}
