use bevy::prelude::*;
use rand::Rng;

use crate::game::sim::dungeon_components::TextType;
use crate::game::sim::event_handling::SimMessageEvent;

use bevy_inspector_egui::Inspectable;

#[derive(Component, Default, Copy, Clone, Inspectable)]
pub struct Combatant {
    pub health: i32,
    pub max_health: i32,
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

#[derive(Default, Inspectable)]
pub struct Hero {
    pub combat_stats: Combatant,
}

#[derive(Default, Inspectable)]
pub struct Enemy {
    pub combat_stats: Combatant,
}

pub fn process_combat(
    events: &mut EventWriter<SimMessageEvent>,
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
        hero.health -= damage;
        events.send(SimMessageEvent(TextType::CombatHeroHit));
        debug!("Hero hit for {}: HP at {}.", damage, hero.health);
    } else if hero_roll > monster_roll {
        let diff = hero_roll - monster_roll;
        let damage = (hero.damage_bonus + diff - monster.damage_res).clamp(0, 500);
        monster.health -= damage;
        events.send(SimMessageEvent(TextType::CombatEnemyHit));
        debug!("Monster hit for {}: HP at {}.", damage, monster.health);
    } else {
        events.send(SimMessageEvent(TextType::CombatNoResolution));
    }

    if hero.health < 1 {
        *cmbt_state = CombatState::HeroDead;
    } else if monster.health < 1 {
        *cmbt_state = CombatState::EnemyDead;
    }
}
