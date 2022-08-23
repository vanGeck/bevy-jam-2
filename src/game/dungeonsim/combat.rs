use bevy::prelude::{Component};

pub struct Combatant {
    pub health: i32,
    pub proficiency: i32,
    pub damage_res: i32,
    pub damage_bonus: i32
}

#[derive(Component)]
pub struct Hero;

#[derive(Component)]
pub struct Enemy;