use bevy::{math::clamp, prelude::*};

use crate::{
    core::{GameState, GAME_STAGE_NAME},
    projectiles::{spawn_projectile, ProjectileAssets},
};

#[derive(Clone)]
pub struct Weapon {
    pub fire_rate: f32,
    pub enabled: bool,
    pub is_enemy: bool,
    pub cooldown: f32,
    pub damage: i32,
    pub offsets: Vec<Vec2>,
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.on_state_update(GAME_STAGE_NAME, GameState::Game, weapon_shooting.system());
        app.on_state_exit(GAME_STAGE_NAME, GameState::Game, cleanup_weapons.system());
    }
}

fn cleanup_weapons(commands: &mut Commands, weapons: Query<(Entity, &Weapon)>) {
    weapons.iter().for_each(|(ent, _)| {
        commands.despawn_recursive(ent);
    });
}

fn weapon_shooting(
    commands: &mut Commands,
    mut weapons: Query<(&mut Weapon, &Transform)>,
    passets: Res<ProjectileAssets>,
    time: Res<Time>,
) {
    for (mut weapon, trans) in weapons.iter_mut() {
        if weapon.cooldown > 0.0 {
            weapon.cooldown = clamp(weapon.cooldown - time.delta_seconds(), 0.0, std::f32::MAX);
        } else if weapon.enabled {
            for offset in &weapon.offsets {
                spawn_projectile(
                    commands,
                    &passets,
                    weapon.is_enemy,
                    trans.translation
                        + Vec3 {
                            x: offset.x,
                            y: offset.y,
                            z: 0.0,
                        },
                    weapon.damage,
                );
            }
            weapon.cooldown = 1.0 / weapon.fire_rate;
        }
    }
}
