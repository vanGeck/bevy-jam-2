use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Default, Copy, Clone)]
pub struct Combatant {
    pub health: i32,
    pub proficiency: i32,
    pub damage_res: i32,
    pub damage_bonus: i32,
}

#[derive(Default, PartialEq, Eq, Copy, Clone)]
pub enum CombatState {
    #[default]
    Init,
    InProgress,
    EnemyDead,
    HeroDead,
    Ended,
}

pub struct Hero {
    pub combat_stats: Combatant,
}

pub struct Enemy {
    pub combat_stats: Combatant,
}

pub fn process_combat(
    mut monster: &mut Combatant,
    mut hero: &mut Combatant,
    cmbt_state: &mut CombatState,
) {
    const DICE: i32 = 8;
    let mut rng = rand::thread_rng();
    let monster_roll = rng.gen_range(0..DICE) + monster.proficiency;
    let hero_roll = rng.gen_range(0..DICE) + hero.proficiency;

    if monster_roll > hero_roll {
        let diff = monster_roll - hero_roll;
        let damage = (monster.damage_bonus - hero.damage_res + diff).clamp(0, 500);
        hero.health = hero.health - damage;
        info!("Monster hits the hero for {} damage.", damage);
    } else if hero_roll > monster_roll {
        let diff = hero_roll - monster_roll;
        let damage = (hero.damage_bonus + diff - monster.damage_res).clamp(0, 500);
        monster.health = monster.health - damage;
        info!("Hero hits the monster for {} damage.", damage);
    } else {
        info!("Blows are exchanged, but no blood is drawn.")
    }

    if hero.health < 1 {
        *cmbt_state = CombatState::HeroDead;
    } else if monster.health < 1 {
        *cmbt_state = CombatState::EnemyDead;
    }
}
