use crate::core::{
    Damagable, GameState, GAME_STAGE_NAME, THRESHOLD_NX, THRESHOLD_NY, THRESHOLD_X, THRESHOLD_Y,
};
use bevy::prelude::*;

mod consts {}

pub struct Projectile {
    pub direction: Vec2,
    pub damage: i32,
    pub speed: f32,
    pub hitbox_size: Vec2,
    pub is_enemy: bool,
}

pub struct ProjectileAssets {
    pub player_projectile: Handle<ColorMaterial>,
    pub enemy_projectile: Handle<ColorMaterial>,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_update(GAME_STAGE_NAME, GameState::Game, projectile_system.system());
        app.on_state_exit(
            GAME_STAGE_NAME,
            GameState::Game,
            cleanup_projectiles.system(),
        );
    }
}

pub fn spawn_projectile(
    commands: &mut Commands,
    passets: &ProjectileAssets,
    is_enemy: bool,
    pos: Vec3,
    damage: i32,
) {
    commands
        .spawn(SpriteBundle {
            material: if is_enemy {
                passets.enemy_projectile.clone()
            } else {
                passets.player_projectile.clone()
            },
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .with(Projectile {
            damage,
            direction: if is_enemy {
                Vec2::new(0.0, -1.0).normalize()
            } else {
                Vec2::new(0.0, 1.0).normalize()
            },
            speed: if is_enemy { 300.0 } else { 500.0 },
            hitbox_size: Vec2::new(20.0, 20.0),
            is_enemy,
        })
        .with(Transform {
            translation: Vec3 { z: 0.0, ..pos },
            ..Default::default()
        });
}

fn cleanup_projectiles(commands: &mut Commands, projectiles: Query<(Entity, &Projectile)>) {
    projectiles.iter().for_each(|(ent, _)| {
        commands.despawn_recursive(ent);
    });
}

fn projectile_system(
    commands: &mut Commands,
    mut projectiles: Query<(Entity, &Projectile, &mut Transform)>,
    mut damagables: Query<(&mut Damagable, &Transform)>,
    time: Res<Time>,
) {
    for (ent, projectile, mut transform) in projectiles.iter_mut() {
        let projectile = Projectile {
            direction: projectile.direction.normalize(),
            ..*projectile
        };
        transform.translation.x += projectile.direction.x * projectile.speed * time.delta_seconds();
        transform.translation.y += projectile.direction.y * projectile.speed * time.delta_seconds();

        // Damage
        for (mut damagable, dtransform) in damagables.iter_mut() {
            if (dtransform.translation.x - transform.translation.x).abs()
                <= damagable.hitbox_size.x / 2.0 + projectile.hitbox_size.x / 2.0
                && (dtransform.translation.y - transform.translation.y).abs()
                    <= damagable.hitbox_size.y / 2.0 + projectile.hitbox_size.y / 2.0
                && damagable.is_enemy != projectile.is_enemy
                && damagable.damagable
            {
                damagable.damage(projectile.damage);
                info!("DAMAGED! Current health: {:?}", damagable);
                commands.despawn_recursive(ent);
            }
        }

        // Out of bounds
        if transform.translation.y < THRESHOLD_NY
            || transform.translation.y > THRESHOLD_Y
            || transform.translation.x < THRESHOLD_NX
            || transform.translation.x > THRESHOLD_X
        {
            commands.despawn_recursive(ent);
        }
    }
}
