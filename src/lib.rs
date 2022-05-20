pub mod board;
use board::*;
pub mod custom_commands;
use custom_commands::*;
pub mod assets;
pub mod blocks;
pub mod levels;
pub mod scoring;
pub mod ui;

use bevy::prelude::*;

pub const STARTING_GAME_STATE: GameState = GameState::Menu;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
}

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Ball;

#[derive(Component)]
pub struct PlayingAreaBorder;

#[derive(Component)]
pub struct DespawnArea;

#[derive(Component)]
pub struct ConnectToPaddle {
    pub diff: f32,
}

#[derive(Component, Deref, DerefMut)]
pub struct Damage(pub u8);

#[derive(Component, Debug)]
pub enum Powerup {
    TripleBall,
    WidePaddle,
    Gunship,
}

pub struct SpawnThreeBallsEvent;
