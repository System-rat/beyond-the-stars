use bevy::prelude::*;

use crate::{
    core::{Damagable, GameState, GAME_STAGE_NAME, THRESHOLD_NY},
    weapon::Weapon,
};

#[derive(Clone)]
pub struct Enemy {
    pub worth: i32,
}

pub struct EnemyAssets {
    pub enemy_material: Handle<ColorMaterial>,
}

pub struct EnemyPlugin;

pub struct EnemyKilledEvent(pub i32);

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_update(GAME_STAGE_NAME, GameState::Game, enemy_movement.system());
        app.on_state_update(GAME_STAGE_NAME, GameState::Game, enemy_death.system());
        app.on_state_exit(GAME_STAGE_NAME, GameState::Game, cleanup_enemies.system());
        app.add_event::<EnemyKilledEvent>();
    }
}

fn cleanup_enemies(commands: &mut Commands, enemies: Query<(Entity, &Enemy)>) {
    enemies.iter().for_each(|(ent, _)| {
        commands.despawn_recursive(ent);
    });
}

pub fn spawn_enemy(commands: &mut Commands, eassets: &EnemyAssets, x_pos: f32) {
    commands
        .spawn(SpriteBundle {
            material: eassets.enemy_material.clone(),
            sprite: Sprite::new(Vec2::new(50.0, 50.0)),
            ..Default::default()
        })
        .with(Transform {
            translation: Vec3::new(x_pos, 360.0, 1.0),
            ..Default::default()
        })
        .with(Enemy { worth: 20 })
        .with(Damagable {
            health: 90,
            hitbox_size: Vec2::new(30.0, 30.0),
            is_enemy: true,
            damagable: true,
        })
        .with(Weapon {
            cooldown: 0.0,
            enabled: true,
            fire_rate: 1.0,
            is_enemy: true,
            damage: 10,
            offsets: vec![Vec2::new(-3.0, 0.0)],
        });
}

fn enemy_death(
    commands: &mut Commands,
    enemies: Query<(&Enemy, &Damagable, Entity)>,
    mut killed_event: ResMut<Events<EnemyKilledEvent>>,
) {
    for (enemy, damage, ent) in enemies.iter() {
        if damage.health == 0 {
            info!("ENEMY KILLED! Worth: {}", enemy.worth);
            commands.despawn_recursive(ent);
            killed_event.send(EnemyKilledEvent(enemy.worth));
        }
    }
}

fn enemy_movement(
    commands: &mut Commands,
    mut enemies: Query<(Entity, &Enemy, &mut Transform)>,
    time: Res<Time>,
) {
    for (ent, _, mut trans) in enemies.iter_mut() {
        trans.translation.y -= 50.0 * time.delta_seconds();

        if trans.translation.y < THRESHOLD_NY {
            commands.despawn_recursive(ent);
        }
    }
}
