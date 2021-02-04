use bevy::{math::clamp, prelude::*};

use crate::{
    core::{Damagable, GameOverEvent, GameState, GAME_STAGE_NAME},
    enemy::{Enemy, EnemyKilledEvent},
    weapon::Weapon,
};

pub struct Player {
    move_speed: f32,
    pub lives: i32,
    remainging_invincibility: f32,
    pub score: i32,
}

pub struct PlayerAssets {
    pub player_material: Handle<ColorMaterial>,
    pub invincible_material: Handle<ColorMaterial>,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_update(GAME_STAGE_NAME, GameState::Game, player_movement.system());
        app.on_state_update(GAME_STAGE_NAME, GameState::Game, player_shooting.system());
        app.on_state_update(GAME_STAGE_NAME, GameState::Game, player_damage.system());
        app.on_state_update(GAME_STAGE_NAME, GameState::Game, player_score.system());
        app.on_state_update(GAME_STAGE_NAME, GameState::Game, exit_to_main_menu.system());
        app.on_state_exit(GAME_STAGE_NAME, GameState::Game, cleanup_players.system());
        app.add_event::<GameOverEvent>();
    }
}

fn cleanup_players(commands: &mut Commands, players: Query<(Entity, &Player)>) {
    players.iter().for_each(|(ent, _)| {
        commands.despawn_recursive(ent);
    });
}

fn exit_to_main_menu(mut state: ResMut<State<GameState>>, key: Res<Input<KeyCode>>) {
    if key.pressed(KeyCode::Escape) {
        state.set_next(GameState::MainMenu).unwrap();
    }
}

fn player_movement(
    mut players: Query<(&Player, &mut Transform)>,
    key: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (player, mut trans) in players.iter_mut() {
        if key.pressed(KeyCode::Left) {
            trans.translation.x -= player.move_speed * time.delta_seconds();
        } else if key.pressed(KeyCode::Right) {
            trans.translation.x += player.move_speed * time.delta_seconds();
        }

        trans.translation.x = clamp(trans.translation.x, -350.0 + 15.0, 350.0 - 15.0);
    }
}

fn player_score(
    enemy_death_events: Res<Events<EnemyKilledEvent>>,
    mut event_reader: Local<EventReader<EnemyKilledEvent>>,
    mut players: Query<&mut Player>,
) {
    for enemy_event in event_reader.iter(&enemy_death_events) {
        for mut player in players.iter_mut() {
            player.score += enemy_event.0;
        }
    }
}

fn player_damage(
    commands: &mut Commands,
    mut players: Query<(
        &mut Player,
        &mut Transform,
        &mut Damagable,
        Entity,
        &mut Handle<ColorMaterial>,
    )>,
    enemies: Query<(&Enemy, &Transform, &Damagable, Entity)>,
    mut events: ResMut<Events<GameOverEvent>>,
    time: Res<Time>,
    mut killed_event: ResMut<Events<EnemyKilledEvent>>,
    pmats: Res<PlayerAssets>,
) {
    for (mut player, mut trans, mut health, ent, mut sprite) in players.iter_mut() {
        if health.health <= 0 {
            if player.lives > 0 {
                player.lives -= 1;
                trans.translation = Vec3::new(0.0, -300.0, 1.0);
                health.health = 100;
                player.remainging_invincibility = 3.0;
                info!("PLAYER KILLED!");
            } else {
                commands.despawn_recursive(ent);
                info!("PLAYER KILLED!");
                events.send(GameOverEvent(player.score));
            }
        }

        if player.remainging_invincibility <= 0.0 {
            *sprite = pmats.player_material.clone();
            health.damagable = true;
            for (enemy, etrans, ehealth, eent) in enemies.iter() {
                if (etrans.translation.x - trans.translation.x).abs()
                    <= (health.hitbox_size.x / 2.0 + ehealth.hitbox_size.x / 2.0)
                    && (etrans.translation.y - trans.translation.y).abs()
                        <= (health.hitbox_size.y / 2.0 + ehealth.hitbox_size.y / 2.0)
                {
                    player.lives -= 1;
                    trans.translation = Vec3::new(0.0, -300.0, 1.0);
                    health.health = 100;
                    player.remainging_invincibility = 3.0;
                    commands.despawn_recursive(eent);
                    killed_event.send(EnemyKilledEvent(enemy.worth));
                    info!("PLAYER KILLED BY ENEMY!");
                    break;
                }
            }
        } else {
            *sprite = pmats.invincible_material.clone();
            health.damagable = false;
            player.remainging_invincibility = clamp(
                player.remainging_invincibility - time.delta_seconds(),
                0.0,
                core::f32::MAX,
            );
        }
    }
}

fn player_shooting(mut weapons: Query<&mut Weapon, With<Player>>, key: Res<Input<KeyCode>>) {
    for mut weapon in weapons.iter_mut() {
        if key.pressed(KeyCode::Space) {
            weapon.enabled = true;
        } else {
            weapon.enabled = false;
        }
    }
}

pub fn spawn_player(commands: &mut Commands, passets: &PlayerAssets) {
    commands
        .spawn(SpriteBundle {
            material: passets.player_material.clone(),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .with(Damagable {
            health: 100,
            hitbox_size: Vec2::new(30.0, 30.0),
            is_enemy: false,
            damagable: true,
        })
        .with(Transform {
            translation: Vec3::new(0.0, -290.0, 2.0),
            ..Default::default()
        })
        .with(Player {
            move_speed: 400.0,
            lives: 3,
            remainging_invincibility: 3.0,
            score: 0,
        })
        .with(Weapon {
            enabled: false,
            cooldown: 0.0,
            fire_rate: 6.0,
            is_enemy: false,
            damage: 50,
            offsets: vec![Vec2::new(10.0, 20.0), Vec2::new(-10.0, 20.0)],
        });
}
