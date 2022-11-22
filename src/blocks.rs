use crate::{custom_commands::SpawnPowerup, Damage};
use bevy::prelude::*;
use rand::Rng;

use Block::*;

#[derive(Clone, Copy, Component)]
pub enum Block {
    White,
    Orange,
    LightBlue,
    Green,
    Red,
    Blue,
    Pink,
    Yellow,
    Silver,
    Gold,
}

impl Block {
    pub fn color(&self) -> Color {
        // based on #c6eefe
        match self {
            White => Color::WHITE,
            Orange => Color::hex("fef2c6").unwrap(), /* Color::ORANGE, */
            LightBlue => Color::hex("c6eefe").unwrap(), /* Color::ALICE_BLUE, */
            Green => Color::hex("c6fed6").unwrap(), /* Color::GREEN, */
            Red => Color::hex("fec6d2").unwrap(), /* Color::RED, */
            Blue => Color::hex("c6d2fe").unwrap(), /* Color::BLUE, */
            Pink => Color::hex("fec6ee").unwrap(), /* Color::PINK, */
            Yellow => Color::hex("eefec6").unwrap(), /* Color::YELLOW, */
            Silver => Color::hex("dae5ea").unwrap(), /* Color::SILVER, */
            Gold => Color::GOLD,
        }
    }
}

pub fn block_removal(
    mut commands: Commands,
    blocks: Query<(Entity, &Transform, &Damage, &Block)>,
) {
    let mut rng = rand::thread_rng();
    for (entity, position, damage, block) in blocks.iter() {
        match block {
            Silver => {
                if damage.0 >= 5 {
                    commands
                        .entity(entity)
                        .despawn_recursive();
                }
            }
            Gold => {
                // do nothing, gold can't be
                // destroyed
            }
            _ => {
                if damage.0 >= 1 {
                    commands
                        .entity(entity)
                        .despawn_recursive();
                    if rng.gen_range(0..10) == 5 {
                        commands.add(SpawnPowerup {
                            transform: position.clone(),
                        });
                    };
                }
            }
        }
    }
}
