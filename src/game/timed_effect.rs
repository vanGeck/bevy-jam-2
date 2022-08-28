use crate::game::combat::{Enemy, Hero};
use crate::{default, Entity, KeyCode, Query, Res};
use bevy::input::Input;
use bevy::prelude::{Commands, Component, ResMut};
use bevy::time::{Time, Timer};
use serde::{Serialize, Deserialize};

pub struct TimedEffectTicker {
    pub timer: Timer,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct TemporaryModifier {
    pub time: f32,
    pub max_health_mod: i32,
    pub combat_prof_mod: i32,
    pub damage_mod: i32,
    pub damage_res_mod: i32,
    pub on_hero: bool,
    pub applied: bool,
    pub expired: bool,
}

impl Default for TemporaryModifier {
    fn default() -> Self {
        TemporaryModifier {
            time: 1.0,
            max_health_mod: 0,
            combat_prof_mod: 0,
            damage_mod: 0,
            damage_res_mod: 0,
            on_hero: false,
            applied: false,
            expired: false,
        }
    }
}

#[derive(Component)]
pub struct DamageOverTime {
    pub ticks: f32,
    pub damage: f32,
    pub on_hero: bool,
}

#[derive(Component)]
pub struct ModifierExpired;

pub fn tick_temporary_modifiers(
    mut q: Query<(Entity, &mut TemporaryModifier)>,
    mut hero: ResMut<Hero>,
    mut enemy: ResMut<Enemy>,
    mut ticker: ResMut<TimedEffectTicker>,
    time: Res<Time>,
    mut cmd: Commands,
) {
    if ticker.timer.tick(time.delta()).just_finished() {
        for (e, mut modifier) in q.iter_mut() {
            // actually apply modifier to stats if not yet applied
            if !modifier.applied {
                modifier.applied = true;
                if modifier.on_hero {
                    hero.combat_stats.damage_bonus += modifier.damage_mod;
                    hero.combat_stats.max_health += modifier.max_health_mod;
                    hero.combat_stats.damage_res += modifier.damage_res_mod;
                    hero.combat_stats.proficiency += modifier.combat_prof_mod;
                    if hero.combat_stats.health > hero.combat_stats.max_health {
                        hero.combat_stats.health = hero.combat_stats.max_health;
                    }
                } else {
                    enemy.combat_stats.damage_bonus += modifier.damage_mod;
                    enemy.combat_stats.max_health += modifier.max_health_mod;
                    enemy.combat_stats.damage_res += modifier.damage_res_mod;
                    enemy.combat_stats.proficiency += modifier.combat_prof_mod;
                    if enemy.combat_stats.health > enemy.combat_stats.max_health {
                        enemy.combat_stats.health = enemy.combat_stats.max_health;
                    }
                }
            }

            // despawn modifier and return early if combatant is dead
            if hero.combat_stats.health < 1 && modifier.on_hero {
                hero.combat_stats.damage_bonus -= modifier.damage_mod;
                hero.combat_stats.max_health -= modifier.max_health_mod;
                hero.combat_stats.damage_res -= modifier.damage_res_mod;
                hero.combat_stats.proficiency -= modifier.combat_prof_mod;
                cmd.entity(e).despawn();
                continue;
            }

            if enemy.combat_stats.health < 1 && !modifier.on_hero {
                enemy.combat_stats.damage_bonus -= modifier.damage_mod;
                enemy.combat_stats.max_health -= modifier.max_health_mod;
                enemy.combat_stats.damage_res -= modifier.damage_res_mod;
                enemy.combat_stats.proficiency -= modifier.combat_prof_mod;
                cmd.entity(e).despawn();
                continue;
            }

            // tick timer
            modifier.time -= (ticker.timer.duration().as_millis() / 1000.0 as u128) as f32;
            // if expired, remove effect and despawn entity
            if modifier.time <= 0.0 {
                if modifier.on_hero {
                    hero.combat_stats.damage_bonus -= modifier.damage_mod;
                    hero.combat_stats.max_health -= modifier.max_health_mod;
                    hero.combat_stats.damage_res -= modifier.damage_res_mod;
                    hero.combat_stats.proficiency -= modifier.combat_prof_mod;
                } else {
                    enemy.combat_stats.damage_bonus -= modifier.damage_mod;
                    enemy.combat_stats.max_health -= modifier.max_health_mod;
                    enemy.combat_stats.damage_res -= modifier.damage_res_mod;
                    enemy.combat_stats.proficiency -= modifier.combat_prof_mod;
                }
                cmd.entity(e).despawn();
            }
        }
    }
}

pub fn apply_timed_modifier(modifier: TemporaryModifier, cmd: &mut Commands) {
    cmd.spawn().insert(modifier);
}

pub fn test_apply_modifier(input: Res<Input<KeyCode>>, mut cmd: Commands) {
    if input.just_pressed(KeyCode::M) {
        apply_timed_modifier(
            TemporaryModifier {
                time: 10.0,
                damage_mod: 4,
                on_hero: true,
                ..default()
            },
            &mut cmd,
        );
    }

    if input.just_pressed(KeyCode::N) {
        apply_timed_modifier(
            TemporaryModifier {
                time: 10.0,
                combat_prof_mod: -3,
                on_hero: false,
                ..default()
            },
            &mut cmd,
        );
    }
}

pub fn tick_damage_over_time() {}
