use std::time::Duration;

use bevy::prelude::*;

use crate::{
    core::{Damagable, GameOverEvent, GameState, GAME_STAGE_NAME},
    level::{LevelWonEvent, NextWaveEvent},
    player::Player,
};

pub struct GameUiPlugin;

enum WinStatus {
    Playing,
    Won(Option<fn(&mut Commands)>),
    Lost,
}

struct GameUi {
    win_status: WinStatus,
    fade_timer: Timer,
}

struct ScoreText;

struct HealthBar;

struct LivesText;

struct FadeText;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_exit(GAME_STAGE_NAME, GameState::Game, cleanup_game_ui.system());
        app.on_state_enter(GAME_STAGE_NAME, GameState::Game, setup_game_ui.system());
        app.on_state_update(GAME_STAGE_NAME, GameState::Game, score_system.system());
        app.on_state_update(GAME_STAGE_NAME, GameState::Game, lives_system.system());
        app.on_state_update(GAME_STAGE_NAME, GameState::Game, fade_text.system());
        app.on_state_update(
            GAME_STAGE_NAME,
            GameState::Game,
            notification_system.system(),
        );
        app.on_state_update(GAME_STAGE_NAME, GameState::Game, health_system.system());
    }
}

fn setup_game_ui(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|root| {
            root.spawn(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(10.0),
                        left: Val::Px(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    font: asset_server.load("FiraMono-Medium.ttf"),
                    style: TextStyle {
                        color: Color::WHITE.into(),
                        font_size: 20.0,
                        ..Default::default()
                    },
                    value: "Score".into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(ScoreText);

            root.spawn(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(30.0),
                        left: Val::Px(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text {
                    font: asset_server.load("FiraMono-Medium.ttf"),
                    style: TextStyle {
                        color: Color::WHITE.into(),
                        font_size: 20.0,
                        ..Default::default()
                    },
                    value: "Lives".into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(LivesText);

            root.spawn(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Percent(45.0),
                        bottom: Val::Percent(45.0),
                        left: Val::Percent(20.0),
                        right: Val::Percent(20.0),
                    },
                    margin: Rect::all(Val::Auto),
                    ..Default::default()
                },
                text: Text {
                    font: asset_server.load("FiraMono-Medium.ttf"),
                    style: TextStyle {
                        color: Color::WHITE.into(),
                        font_size: 50.0,
                        ..Default::default()
                    },
                    value: "".into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(FadeText);

            root.spawn(NodeBundle {
                material: materials.add(Color::DARK_GRAY.into()),
                style: Style {
                    position_type: PositionType::Absolute,
                    position: Rect {
                        left: Val::Px(10.0),
                        right: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                        top: Val::Undefined,
                    },
                    padding: Rect::all(Val::Px(2.0)),
                    size: Size::new(Val::Auto, Val::Px(30.0)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|node| {
                node.spawn(NodeBundle {
                    style: Style {
                        position: Rect {
                            left: Val::Px(0.0),
                            top: Val::Px(0.0),
                            bottom: Val::Px(0.0),
                            right: Val::Px(0.0),
                        },
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        ..Default::default()
                    },
                    material: materials.add(Color::RED.into()),
                    ..Default::default()
                })
                .with(HealthBar);

                node.spawn(TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Percent(45.0),
                            right: Val::Percent(45.0),
                            bottom: Val::Px(5.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text {
                        font: asset_server.load("FiraMono-Medium.ttf"),
                        style: TextStyle {
                            color: Color::BLACK.into(),
                            font_size: 20.0,
                            alignment: TextAlignment {
                                horizontal: HorizontalAlign::Center,
                                vertical: VerticalAlign::Center,
                            },
                            ..Default::default()
                        },
                        value: "Health".into(),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
        })
        .with(GameUi {
            win_status: WinStatus::Playing,
            fade_timer: Timer::new(Duration::from_secs(3), false),
        });
}

fn notification_system(
    mut text: Query<(&mut Visible, &mut Text), With<FadeText>>,
    mut game_ui: Query<&mut GameUi>,
    wave_events: Res<Events<NextWaveEvent>>,
    mut wave_reader: Local<EventReader<NextWaveEvent>>,
    game_over_events: Res<Events<GameOverEvent>>,
    mut go_reader: Local<EventReader<GameOverEvent>>,
    won_events: Res<Events<LevelWonEvent>>,
    mut won_reader: Local<EventReader<LevelWonEvent>>,
) {
    for _ in wave_reader.iter(&wave_events) {
        for mut game_ui in game_ui.iter_mut() {
            if let WinStatus::Playing = game_ui.win_status {
                for (mut vis, mut text) in text.iter_mut() {
                    vis.is_visible = true;
                    text.value = "Next wave incomming!".to_owned();
                    game_ui.fade_timer.reset();
                }
            }
        }
    }

    for event in go_reader.iter(&game_over_events) {
        for mut game_ui in game_ui.iter_mut() {
            game_ui.win_status = WinStatus::Lost;
            for (mut vis, mut text) in text.iter_mut() {
                vis.is_visible = true;
                text.value = format!("GAME OVER\nFINAL SCORE {}", event.0);
                game_ui.fade_timer.reset();
            }
        }
    }

    for event in won_reader.iter(&won_events) {
        for mut game_ui in game_ui.iter_mut() {
            game_ui.win_status = WinStatus::Won(event.1);
            for (mut vis, mut text) in text.iter_mut() {
                vis.is_visible = true;
                if event.1.is_some() {
                    text.value = "LEVEL WON".to_owned();
                } else {
                    text.value = format!("YOU WON\nFINAL SCORE: {}", event.0);
                }
                game_ui.fade_timer.reset();
            }
        }
    }
}

fn fade_text(
    commands: &mut Commands,
    mut text: Query<&mut Visible, With<FadeText>>,
    mut game_ui: Query<&mut GameUi>,
    time: Res<Time>,
    mut state: ResMut<State<GameState>>,
) {
    for mut game_ui in game_ui.iter_mut() {
        if game_ui
            .fade_timer
            .tick(time.delta_seconds())
            .just_finished()
        {
            for mut text in text.iter_mut() {
                text.is_visible = false;
            }

            if let WinStatus::Won(func) = game_ui.win_status {
                if let Some(func) = func {
                    (func)(commands);
                    game_ui.win_status = WinStatus::Playing;
                } else {
                    state.set_next(GameState::MainMenu).unwrap();
                }
            } else if let WinStatus::Lost = game_ui.win_status {
                state.set_next(GameState::MainMenu).unwrap();
            }
        }
    }
}

fn score_system(mut text: Query<&mut Text, With<ScoreText>>, players: Query<&Player>) {
    for mut text in text.iter_mut() {
        for player in players.iter() {
            text.value = format!("Score {}", player.score);
        }
    }
}

fn lives_system(mut text: Query<&mut Text, With<LivesText>>, players: Query<&Player>) {
    for mut text in text.iter_mut() {
        for player in players.iter() {
            text.value = format!("Lives remaining {}", player.lives);
        }
    }
}

fn health_system(mut bar: Query<(&mut Style, &HealthBar)>, players: Query<(&Player, &Damagable)>) {
    for (_, health) in players.iter() {
        for (mut style, _) in bar.iter_mut() {
            style.size = Size::new(Val::Percent(health.health as f32), Val::Percent(100.0));
        }
    }
}

fn cleanup_game_ui(commands: &mut Commands, menu: Query<(Entity, &GameUi)>) {
    for (ent, _) in menu.iter() {
        commands.despawn_recursive(ent);
    }
}
