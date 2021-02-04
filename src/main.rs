use std::time::Duration;

use crate::core::{GameState, GAME_STAGE_NAME};

use bevy::{app::AppExit, prelude::*, window::WindowMode};
use enemy::{spawn_enemy, EnemyAssets, EnemyPlugin};
use game_ui::GameUiPlugin;
use level::{EnemySpawn, Level, LevelPlugin, Wave};
use main_menu::MainMenuPlugin;
use player::{spawn_player, PlayerAssets, PlayerPlugin};
use projectiles::{ProjectileAssets, ProjectilePlugin};
use weapon::WeaponPlugin;

mod core;
mod enemy;
mod game_ui;
mod level;
mod main_menu;
mod player;
mod projectiles;
mod weapon;

fn main() {
    App::build()
        .add_stage_after(
            stage::UPDATE,
            GAME_STAGE_NAME,
            StateStage::<GameState>::default(),
        )
        .add_resource(State::new(GameState::MainMenu))
        .add_resource(WindowDescriptor {
            height: 700.0,
            width: 700.0,
            resizable: false,
            title: "Beyond the Stars".to_owned(),
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugin(MainMenuPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugin(ProjectilePlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(WeaponPlugin)
        .add_plugin(LevelPlugin)
        .add_plugin(GameUiPlugin)
        .add_startup_system(startup.system())
        .add_system(escape_system.system())
        .on_state_enter(GAME_STAGE_NAME, GameState::Game, game_startup.system())
        .run();
}

fn startup(commands: &mut Commands, mut material: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(CameraUiBundle::default());
    commands.insert_resource(ProjectileAssets {
        player_projectile: material.add(asset_server.load("player_shot.png").into()),
        enemy_projectile: material.add(asset_server.load("enemy_shot.png").into())
    });
    commands.insert_resource(EnemyAssets {
        enemy_material: material.add(asset_server.load("enemy.png").into())
    });
    commands.insert_resource(PlayerAssets {
        player_material: material.add(asset_server.load("player.png").into()),
        invincible_material: material.add(asset_server.load("player_i.png").into())
    });
}

fn game_startup(commands: &mut Commands, player_assets: Res<PlayerAssets>) {
    spawn_player(commands, &player_assets);
    commands.spawn((Level {
        current_wave: None,
        warmup_timer: Timer::new(Duration::from_secs(2), false),
        wave_timer: Timer::new(Duration::from_secs(3), true),
        next_level_fn: Some(level2),
        waves: vec![
            Wave {
                current_sub_wave: 0,
                enemy_spawns: vec![
                    EnemySpawn {
                        spawn_function: spawn_enemy,
                        spawn_position: -300.0,
                        sub_wave_position: 0,
                    },
                    EnemySpawn {
                        spawn_function: spawn_enemy,
                        spawn_position: 300.0,
                        sub_wave_position: 0,
                    },
                    EnemySpawn {
                        spawn_function: spawn_enemy,
                        spawn_position: -300.0,
                        sub_wave_position: 2,
                    },
                    EnemySpawn {
                        spawn_function: spawn_enemy,
                        spawn_position: 300.0,
                        sub_wave_position: 2,
                    },
                ],
            },
            Wave {
                current_sub_wave: 0,
                enemy_spawns: vec![
                    EnemySpawn {
                        spawn_function: spawn_enemy,
                        spawn_position: -200.0,
                        sub_wave_position: 0,
                    },
                    EnemySpawn {
                        spawn_function: spawn_enemy,
                        spawn_position: 200.0,
                        sub_wave_position: 0,
                    },
                    EnemySpawn {
                        spawn_function: spawn_enemy,
                        spawn_position: -200.0,
                        sub_wave_position: 2,
                    },
                    EnemySpawn {
                        spawn_function: spawn_enemy,
                        spawn_position: 200.0,
                        sub_wave_position: 2,
                    },
                ],
            },
        ],
    },));
}

fn level2(commands: &mut Commands) {
    commands.spawn((Level {
        current_wave: None,
        warmup_timer: Timer::new(Duration::from_secs(2), false),
        wave_timer: Timer::new(Duration::from_secs(3), true),
        next_level_fn: None,
        waves: vec![
            Wave {
                current_sub_wave: 0,
                enemy_spawns: vec![
                    EnemySpawn {
                        spawn_function: spawn_enemy,
                        spawn_position: -300.0,
                        sub_wave_position: 0,
                    },
                    EnemySpawn {
                        spawn_function: spawn_enemy,
                        spawn_position: 300.0,
                        sub_wave_position: 0,
                    },
                    EnemySpawn {
                        spawn_function: spawn_enemy,
                        spawn_position: -300.0,
                        sub_wave_position: 2,
                    },
                    EnemySpawn {
                        spawn_function: spawn_enemy,
                        spawn_position: 300.0,
                        sub_wave_position: 2,
                    },
                ],
            },
            Wave {
                current_sub_wave: 0,
                enemy_spawns: vec![
                    EnemySpawn {
                        spawn_function: spawn_enemy,
                        spawn_position: -200.0,
                        sub_wave_position: 0,
                    },
                    EnemySpawn {
                        spawn_function: spawn_enemy,
                        spawn_position: 200.0,
                        sub_wave_position: 0,
                    },
                    EnemySpawn {
                        spawn_function: spawn_enemy,
                        spawn_position: -200.0,
                        sub_wave_position: 2,
                    },
                    EnemySpawn {
                        spawn_function: spawn_enemy,
                        spawn_position: 200.0,
                        sub_wave_position: 2,
                    },
                ],
            },
        ],
    },));
}

fn escape_system(keyboard: Res<Input<KeyCode>>, mut events: ResMut<Events<AppExit>>) {
    if keyboard.pressed(KeyCode::LAlt) && keyboard.pressed(KeyCode::Q) {
        info!("EXITING");
        events.send(AppExit);
    }
}
