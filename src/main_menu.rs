use bevy::{app::AppExit, prelude::*};

use crate::core::{GameState, GAME_STAGE_NAME};

pub struct MainMenuPlugin;

struct PlayButton;

struct ExitButton;

struct MainMenu;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_enter(
            GAME_STAGE_NAME,
            GameState::MainMenu,
            setup_main_menu.system(),
        );
        app.on_state_exit(
            GAME_STAGE_NAME,
            GameState::MainMenu,
            cleanup_main_menu.system(),
        );
        app.on_state_update(GAME_STAGE_NAME, GameState::MainMenu, play_button.system());
        app.on_state_update(GAME_STAGE_NAME, GameState::MainMenu, exit_button.system());
    }
}

fn setup_main_menu(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|root| {
            root.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Stretch,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                material: materials.add(Color::NONE.into()),
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: Rect::all(Val::Px(5.0)),
                            flex_grow: 1.0,
                            size: Size::new(Val::Percent(100.0), Val::Undefined),
                            ..Default::default()
                        },
                        material: materials.add(Color::GRAY.into()),
                        ..Default::default()
                    })
                    .with(ExitButton)
                    .with_children(|button| {
                        button.spawn(TextBundle {
                            text: Text {
                                font: asset_server.load("FiraMono-Medium.ttf"),
                                value: "Exit".to_owned(),
                                style: TextStyle {
                                    alignment: TextAlignment {
                                        horizontal: HorizontalAlign::Center,
                                        vertical: VerticalAlign::Center,
                                    },
                                    color: Color::BLACK,
                                    font_size: 30.0,
                                },
                            },
                            ..Default::default()
                        });
                    });

                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: Rect::all(Val::Px(5.0)),
                            flex_grow: 1.0,
                            size: Size::new(Val::Percent(100.0), Val::Undefined),
                            ..Default::default()
                        },
                        material: materials.add(Color::GRAY.into()),
                        ..Default::default()
                    })
                    .with(PlayButton)
                    .with_children(|button| {
                        button.spawn(TextBundle {
                            text: Text {
                                font: asset_server.load("FiraMono-Medium.ttf"),
                                value: "Play".to_owned(),
                                style: TextStyle {
                                    alignment: TextAlignment {
                                        horizontal: HorizontalAlign::Center,
                                        vertical: VerticalAlign::Center,
                                    },
                                    color: Color::BLACK,
                                    font_size: 30.0,
                                },
                            },
                            ..Default::default()
                        });
                    });

                parent.spawn(TextBundle {
                    style: Style {
                        flex_grow: 1.0,
                        size: Size::new(Val::Percent(100.0), Val::Undefined),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        align_content: AlignContent::Center,
                        ..Default::default()
                    },
                    text: Text {
                        font: asset_server.load("FiraMono-Medium.ttf"),
                        value: "Beyond the Stars".to_owned(),
                        style: TextStyle {
                            color: Color::WHITE,
                            font_size: 45.0,
                            ..Default::default()
                        },
                    },
                    ..Default::default()
                });
            });
        })
        .with(MainMenu);
    info!("MENU SETUP COMPLETE");
}

fn play_button(
    mut interaction_query: Query<
        &Interaction,
        (Mutated<Interaction>, With<Button>, With<PlayButton>),
    >,
    mut state: ResMut<State<GameState>>,
    key: Res<Input<KeyCode>>
) {
    if key.pressed(KeyCode::Space) {
        state.set_next(GameState::Game).unwrap();
    }

    for int in interaction_query.iter_mut() {
        if *int == Interaction::Clicked {
            info!("PLAY CLICKED");
            state.set_next(GameState::Game).unwrap();
        }
    }
}

fn exit_button(
    mut interaction_query: Query<
        &Interaction,
        (Mutated<Interaction>, With<Button>, With<ExitButton>),
    >,
    mut exit_events: ResMut<Events<AppExit>>,
) {
    for int in interaction_query.iter_mut() {
        if *int == Interaction::Clicked {
            exit_events.send(AppExit);
        }
    }
}

fn cleanup_main_menu(commands: &mut Commands, menu: Query<(Entity, &MainMenu)>) {
    for (ent, _) in menu.iter() {
        commands.despawn_recursive(ent);
    }
}
