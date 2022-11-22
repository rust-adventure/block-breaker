use std::time::{Duration, Instant};

use bevy::prelude::{App, Plugin, Res, ResMut, Resource};
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::GameState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Timer>()
            .init_resource::<Score>()
            .init_resource::<HighScore>()
            .add_enter_system(
                GameState::Playing,
                start_timer,
            )
            .add_exit_system(
                GameState::Playing,
                close_timer,
            );
    }
}

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Resource,
)]
pub struct Score {
    pub score: u32,
}

#[derive(
    Debug, Default, Clone, PartialEq, Eq, Resource,
)]
pub struct HighScore {
    pub score: u32,
    pub time: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq, Resource)]
pub struct Timer {
    pub start: Option<Instant>,
    pub runtime: Option<Duration>,
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            start: None,
            runtime: None,
        }
    }
}

fn start_timer(mut timer: ResMut<Timer>) {
    *timer = Timer {
        start: Some(Instant::now()),
        runtime: None,
    };
}

fn close_timer(
    mut timer: ResMut<Timer>,
    score: Res<Score>,
    mut high_score: ResMut<HighScore>,
) {
    let elapsed = timer.start.unwrap().elapsed();
    timer.runtime = Some(elapsed);
    if score.score > high_score.score
        || score.score == high_score.score
            && elapsed < high_score.time
    {
        *high_score = HighScore {
            score: score.score,
            time: elapsed,
        }
    }
}
