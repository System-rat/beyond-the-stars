use bevy::prelude::*;

use crate::{core::{GameState, GAME_STAGE_NAME}, enemy::{Enemy, EnemyAssets}, player::Player};

pub struct Level {
    pub warmup_timer: Timer,
    pub wave_timer: Timer,
    pub current_wave: Option<Wave>,
    pub waves: Vec<Wave>,
    pub next_level_fn: Option<fn(&mut Commands)>
}

pub struct Wave {
    pub current_sub_wave: i32,
    pub enemy_spawns: Vec<EnemySpawn>,
}

impl Wave {
    pub fn is_done(&self) -> bool {
        self.enemy_spawns.len() == 0
    }
}

pub struct EnemySpawn {
    pub sub_wave_position: i32,
    pub spawn_function: fn(&mut Commands, &EnemyAssets, f32),
    pub spawn_position: f32,
}

pub struct LevelPlugin;

pub struct LevelWonEvent(pub i32, pub Option<fn(&mut Commands)>);

pub struct NextWaveEvent;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_update(GAME_STAGE_NAME, GameState::Game, level_spawner.system());
        app.on_state_update(GAME_STAGE_NAME, GameState::Game, level_timers.system());
        app.on_state_update(GAME_STAGE_NAME, GameState::Game, level_win.system());
        app.on_state_exit(GAME_STAGE_NAME, GameState::Game, cleanup_levels.system());
        app.add_event::<LevelWonEvent>();
        app.add_event::<NextWaveEvent>();
    }
}

fn cleanup_levels(commands: &mut Commands, levels: Query<(Entity, &Level)>) {
    levels.iter().for_each(|(ent, _)| {
        commands.despawn_recursive(ent);
    });
}

fn level_timers(mut levels: Query<&mut Level>, time: Res<Time>) {
    for mut level in levels.iter_mut() {
        level.warmup_timer.tick(time.delta_seconds());
        if level.warmup_timer.finished() {
            level.wave_timer.tick(time.delta_seconds());
        }
    }
}

fn level_win(
    commands: &mut Commands,
    levels: Query<(&Level, Entity)>,
    mut level_won_events: ResMut<Events<LevelWonEvent>>,
    players: Query<&Player>
) {
    for (level, ent) in levels.iter() {
        if level.current_wave.is_none() && level.waves.len() == 0 {
            info!("LEVEL WON!");
            commands.despawn_recursive(ent);
            for player in players.iter() {
                level_won_events.send(LevelWonEvent(player.score, level.next_level_fn));
            }
        }
    }
}

fn level_spawner(
    commands: &mut Commands,
    enemy_assets: Res<EnemyAssets>,
    mut levels: Query<&mut Level>,
    enemies: Query<&Enemy>,
    mut next_wave_event: ResMut<Events<NextWaveEvent>>
) {
    for mut level in levels.iter_mut() {
        if !level.warmup_timer.finished() {
            continue;
        }

        if let Some(ref mut wave) = level.current_wave {
            if wave.is_done() && enemies.iter().len() == 0 {
                level.current_wave = None;
                level.wave_timer.reset();
            }
        } else {
            level.current_wave = level.waves.pop();
            if level.current_wave.is_some() {
                next_wave_event.send(NextWaveEvent);
            }
        }

        if !level.wave_timer.finished() {
            continue;
        }

        if let Some(ref mut wave) = level.current_wave {
            wave.enemy_spawns
                .iter()
                .filter(|enemy_spawn| enemy_spawn.sub_wave_position == wave.current_sub_wave)
                .for_each(|enemy_spawn| {
                    (enemy_spawn.spawn_function)(
                        commands,
                        &enemy_assets,
                        enemy_spawn.spawn_position,
                    );
                });

            wave.current_sub_wave += 1;
            let sw = wave.current_sub_wave;

            wave.enemy_spawns
                .retain(|enemy_spawn| enemy_spawn.sub_wave_position >= sw);
        }
    }
}
