use bevy::math::{clamp, Vec2};

pub const GAME_STAGE_NAME: &str = "Game";
pub const THRESHOLD_X: f32 = 370.0;
pub const THRESHOLD_NX: f32 = -370.0;
pub const THRESHOLD_Y: f32 = 370.0;
pub const THRESHOLD_NY: f32 = -370.0;

#[derive(Debug)]
pub struct Damagable {
    pub health: i32,
    pub is_enemy: bool,
    pub hitbox_size: Vec2,
    pub damagable: bool
}

impl Damagable {
    pub fn damage(&mut self, amount: i32) {
        self.health = clamp(self.health - amount, 0, std::i32::MAX);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameState {
    MainMenu,
    Game,
}

pub struct GameOverEvent(pub i32);
