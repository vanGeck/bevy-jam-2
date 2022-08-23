use bevy::prelude::{Component};

pub struct Combatant {
    pub health: i32,
    pub proficiency: i32,
    pub damage_res: i32,
    pub damage_bonus: i32
}

#[derive(Default, PartialEq, Eq, Copy, Clone)]
pub enum CombatState {
    #[default]
    Init, 
    InProgress,
    EnemyDead,
    HeroDead,
    Ended
}

#[derive(Component)]
pub struct Hero;

#[derive(Component)]
pub struct Enemy;

pub fn combat_tick(){
    
}