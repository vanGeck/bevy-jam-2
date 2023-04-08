use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;

use crate::states::AppState;

use super::{dungeon_sim::DungeonState, Item};

pub struct BackpackPlugin;

impl Plugin for BackpackPlugin {
    fn build(&self, app: &mut App) {
        // manually manage events
        // see more: https://bevy-cheatbook.github.io/patterns/manual-event-clear.html#manual-event-clearing
        app.init_resource::<Events<SwitchBackpackEvent>>()
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::InGame)
                    .with_system(switch_backpack)
                    .with_system(show_in_backpack_items)
                    .with_system(to_debug_backpack_switching)
                    .into(),
            );
    }
}
#[derive(Component)]
pub struct Backpack(pub usize);

#[derive(Component)]
pub struct BackpackInUse(pub usize);

pub struct SwitchBackpackEvent(usize);

pub fn to_debug_backpack_switching(
    state: Res<DungeonState>,
    input: Res<Input<KeyCode>>,
    backpack_in_use: Query<&BackpackInUse>,
    mut ew_switch_backpack: EventWriter<SwitchBackpackEvent>,
) {
    if !state.running && input.just_pressed(KeyCode::B) {
        let backpack_id = match backpack_in_use.get_single() {
            Ok(BackpackInUse(backpack_id)) => *backpack_id,
            Err(e) => {
                error!(
                    "There should be only one BadInUse component in game.\n{}",
                    e
                );
                return;
            }
        };
        ew_switch_backpack.send(SwitchBackpackEvent(1 - backpack_id));
    }
}

pub fn switch_backpack(
    mut current_backpack: Query<&mut BackpackInUse>,
    mut switch_events: ResMut<Events<SwitchBackpackEvent>>,
) {
    match current_backpack.get_single_mut() {
        Ok(mut backpack_in_use) => {
            for SwitchBackpackEvent(to_switch) in switch_events.drain() {
                backpack_in_use.0 = to_switch;
                info!("backpack switched: {}", backpack_in_use.0);
            }
        }
        Err(e) => {
            error!(
                "There should be only one BadInUse component in game.\n{}",
                e
            );
            return;
        }
    }
}

pub fn show_in_backpack_items(
    mut items: Query<(&Backpack, &mut Visibility), With<Item>>,
    backpack_in_use: Query<&BackpackInUse>,
) {
    let backpack_id = match backpack_in_use.get_single() {
        Ok(BackpackInUse(backpack_id)) => *backpack_id,
        Err(e) => {
            error!(
                "There should be only one BadInUse component in game.\n{}",
                e
            );
            return;
        }
    };

    for (backpack, mut vis) in items.iter_mut() {
        vis.is_visible = backpack.0 == backpack_id;
    }
}
