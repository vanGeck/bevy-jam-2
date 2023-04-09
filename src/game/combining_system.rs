use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;

use crate::audio::sound_event::SoundEvent;
use crate::config::data_items::ItemsData;
use crate::config::data_recipes::RecipesData;
use crate::game::items::Item;
use crate::game::recipes::Recipe;
use crate::game::{find_free_space, ItemId, SoundId, SpawnItemEvent};
use crate::mouse::MouseInteractive;
use crate::positioning::{Coords, Dimens, GridData};
use crate::states::AppState;

use super::backpack::Backpack;
use super::dungeon_sim::{ContinuePrompt, JumpTimepointEvent};
use super::items::CraftItem;

#[derive(Component)]
pub struct CombineButton {
    pub coords: Coords,
}

pub struct EvolutionPlugin;

impl Plugin for EvolutionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EvolutionEvent>().add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::InGame)
                .with_system(evolution_after_jumped_timepoint)
                .with_system(evolution)
                .into(),
        );
    }
}

pub struct EvolutionEvent {
    pub from: usize,
    pub to: usize,
}

fn evolution_after_jumped_timepoint(
    mut jump: EventReader<JumpTimepointEvent>,
    mut evolution: EventWriter<EvolutionEvent>,
) {
    for &JumpTimepointEvent { from, to } in jump.iter() {
        if from > to {
            continue;
        }
        evolution.send(EvolutionEvent { from, to });
    }
}

fn evolution(
    mut evolution: EventReader<EvolutionEvent>,
    mut commands: Commands,
    items: Query<(Entity, &Item, &Backpack, &Coords)>,
    items_data: Res<ItemsData>,
    grid: Res<GridData>,
    mut spawn_event_writer: EventWriter<SpawnItemEvent>,
) {
    for &EvolutionEvent { from, to } in evolution.iter() {
        debug!("evolution from {}, to {}", from, to);

        for (ent, _, _, _) in items.iter().filter(|(_, _, backpack, _)| backpack.0 == to) {
            commands.entity(ent).despawn();
        }

        let from_items = items
            .iter()
            .filter(|(_, _, backpack, _)| backpack.0 == from)
            .map(|(_, item, _, _)| item)
            .collect::<Vec<_>>();
        let new_items = calculate_items_after_evolution(&from_items, &items_data);
        let mut same_tick_items = vec![];
        let items_coords = vec![];
        for (item, cnt) in new_items.into_iter() {
            for _ in 0..cnt {
                if let Some(free_coords) =
                    find_free_space(&grid, Dimens::unit(), &items_coords, &same_tick_items)
                {
                    spawn_event_writer.send(SpawnItemEvent::with_backpack(
                        item.clone(),
                        free_coords,
                        grid.center_crafting(),
                        to,
                    ));
                    same_tick_items.push(free_coords);
                } else {
                    error!("Tried to find free space but failed.");
                }
            }
        }
    }
}

fn count_by_id<'a>(items: impl IntoIterator<Item = &'a &'a Item>, id: ItemId) -> usize {
    items.into_iter().filter(|it| it.id == id).count()
}

fn contains<'a>(items: impl IntoIterator<Item = &'a &'a Item>, id: ItemId) -> bool {
    items.into_iter().find(|it| it.id == id).is_some()
}

fn increase_or_unlock(original: usize, add: usize, unlock: bool) -> usize {
    if original > 0 {
        original + add
    } else {
        if unlock {
            1
        } else {
            0
        }
    }
}

fn calculate_items_after_evolution<'a, T>(
    // this should be items put inside 改變物品格s
    items: &'a T,
    items_data: &ItemsData,
) -> impl IntoIterator<Item = (Item, usize)>
where
    &'a T: IntoIterator<Item = &'a &'a Item>,
{
    let mut v = vec![];
    let get_item = |id: ItemId| {
        let mut item = items_data.try_get_item(id.clone()).unwrap_or_default().1;
        // HACK: for testing
        item.name = id.to_string();
        item
    };
    let tool_points = [
        (ItemId::ElectronicTechnology, 6),
        (ItemId::SteamPower, 5),
        (ItemId::SteelTool, 4),
        (ItemId::IronTool, 3),
        (ItemId::BronzeTool, 2),
        (ItemId::StoneTool, 1),
    ]
    .into_iter()
    .filter(|(id, _)| contains(items, id.clone()))
    .next()
    .map(|(_, point)| point)
    .unwrap_or(0);
    let population = count_by_id(items, ItemId::Wheat) * 100
        + count_by_id(items, ItemId::Meat) * 120
        + count_by_id(items, ItemId::Fish) * 150;

    // update wheat
    v.push((
        get_item(ItemId::Wheat),
        increase_or_unlock(count_by_id(items, ItemId::Wheat), tool_points, true),
    ));
    // update alcohol
    v.push((
        get_item(ItemId::Alcohol),
        increase_or_unlock(
            count_by_id(items, ItemId::Alcohol),
            count_by_id(items, ItemId::Wheat) / 3,
            count_by_id(items, ItemId::Wheat) > 2,
        ),
    ));
    // update meat
    v.push((
        get_item(ItemId::Meat),
        increase_or_unlock(
            count_by_id(items, ItemId::Meat),
            tool_points,
            count_by_id(items, ItemId::GatheringAndHunting) > 0,
        ),
    ));
    // update fish
    v.push((
        get_item(ItemId::Fish),
        increase_or_unlock(
            count_by_id(items, ItemId::Fish),
            tool_points,
            count_by_id(items, ItemId::Fishery) > 0,
        ),
    ));

    // update stone tool
    v.push((
        get_item(ItemId::StoneTool),
        increase_or_unlock(
            count_by_id(items, ItemId::StoneTool),
            count_by_id(items, ItemId::Chiefdom),
            false,
        ),
    ));
    // update bronze tool
    v.push((
        get_item(ItemId::BronzeTool),
        increase_or_unlock(
            count_by_id(items, ItemId::BronzeTool),
            count_by_id(items, ItemId::Religion),
            count_by_id(items, ItemId::StoneTool) > 1,
        ),
    ));
    // update iron tool
    v.push((
        get_item(ItemId::IronTool),
        increase_or_unlock(
            count_by_id(items, ItemId::IronTool),
            count_by_id(items, ItemId::Feudal),
            count_by_id(items, ItemId::BronzeTool) > 2,
        ),
    ));
    // update steel tool
    v.push((
        get_item(ItemId::SteelTool),
        increase_or_unlock(
            count_by_id(items, ItemId::SteelTool),
            count_by_id(items, ItemId::Democracy) | count_by_id(items, ItemId::Centralization),
            count_by_id(items, ItemId::IronTool) > 3,
        ),
    ));
    // update steam tool
    v.push((
        get_item(ItemId::SteamPower),
        increase_or_unlock(
            count_by_id(items, ItemId::SteamPower),
            count_by_id(items, ItemId::Theocracy)
                | count_by_id(items, ItemId::Empire)
                | count_by_id(items, ItemId::Totalitarian)
                | count_by_id(items, ItemId::PermanentMember),
            count_by_id(items, ItemId::SteelTool) > 5,
        ),
    ));
    // update eletronic tool
    v.push((
        get_item(ItemId::ElectronicTechnology),
        increase_or_unlock(
            count_by_id(items, ItemId::ElectronicTechnology),
            0,
            count_by_id(items, ItemId::SteamPower) > 5,
        ),
    ));

    v.push((
        get_item(ItemId::Chiefdom),
        increase_or_unlock(
            count_by_id(items, ItemId::Chiefdom),
            count_by_id(items, ItemId::Wheat) / 3,
            count_by_id(items, ItemId::Wheat) > 2,
        ),
    ));
    v.push((
        get_item(ItemId::Religion),
        increase_or_unlock(
            count_by_id(items, ItemId::Religion),
            0,
            count_by_id(items, ItemId::Alcohol) > 0
                && count_by_id(items, ItemId::Fish) > 0
                && count_by_id(items, ItemId::Meat) > 0,
        ),
    ));
    v.push((
        get_item(ItemId::Theocracy),
        increase_or_unlock(
            count_by_id(items, ItemId::Theocracy),
            0,
            count_by_id(items, ItemId::Religion) > 1
                && count_by_id(items, ItemId::Book) > 1
                && population > 2000,
        ),
    ));
    v.push((
        get_item(ItemId::Feudal),
        increase_or_unlock(
            count_by_id(items, ItemId::Feudal),
            0,
            count_by_id(items, ItemId::Chiefdom) > 0
                && count_by_id(items, ItemId::Writing) > 0
                && population > 1000,
        ),
    ));
    v.push((
        get_item(ItemId::Monarchy),
        increase_or_unlock(
            count_by_id(items, ItemId::Monarchy),
            count_by_id(items, ItemId::Chiefdom) / 5,
            count_by_id(items, ItemId::Chiefdom) > 1 && population > 2000,
        ),
    ));
    v.push((
        get_item(ItemId::Empire),
        increase_or_unlock(
            count_by_id(items, ItemId::Empire),
            0,
            count_by_id(items, ItemId::Monarchy) > 1
                && count_by_id(items, ItemId::Centralization) > 0
                && count_by_id(items, ItemId::Book) > 0
                && population > 2000,
        ),
    ));
    v.push((
        get_item(ItemId::Centralization),
        increase_or_unlock(
            count_by_id(items, ItemId::Centralization),
            0,
            count_by_id(items, ItemId::Monarchy) > 1 && population > 3000,
        ),
    ));
    v.push((
        get_item(ItemId::Totalitarian),
        increase_or_unlock(
            count_by_id(items, ItemId::Totalitarian),
            0,
            count_by_id(items, ItemId::Centralization) > 0
                && count_by_id(items, ItemId::Printing) > 0
                && count_by_id(items, ItemId::SteamPower) > 0
                && population > 2000,
        ),
    ));
    v.push((
        get_item(ItemId::Democracy),
        increase_or_unlock(
            count_by_id(items, ItemId::Democracy),
            0,
            count_by_id(items, ItemId::Trading) > 0
                && count_by_id(items, ItemId::Book) > 0
                && count_by_id(items, ItemId::Wheat) > 1,
        ),
    ));
    v.push((
        get_item(ItemId::PermanentMember),
        increase_or_unlock(
            count_by_id(items, ItemId::PermanentMember),
            0,
            count_by_id(items, ItemId::Democracy) > 0
                && count_by_id(items, ItemId::Trading) > 2
                && population > 2000,
        ),
    ));

    // update writing
    v.push((
        get_item(ItemId::Writing),
        increase_or_unlock(
            count_by_id(items, ItemId::Writing),
            count_by_id(items, ItemId::StoneTool),
            count_by_id(items, ItemId::Religion) > 0 && count_by_id(items, ItemId::StoneTool) > 0,
        ),
    ));
    // update book
    v.push((
        get_item(ItemId::Book),
        increase_or_unlock(
            count_by_id(items, ItemId::Book),
            count_by_id(items, ItemId::BronzeTool),
            count_by_id(items, ItemId::Monarchy) > 0 && count_by_id(items, ItemId::BronzeTool) > 0,
        ),
    ));
    // update printing
    v.push((
        get_item(ItemId::Printing),
        increase_or_unlock(
            count_by_id(items, ItemId::Printing),
            count_by_id(items, ItemId::IronTool),
            count_by_id(items, ItemId::Monarchy) > 0 && count_by_id(items, ItemId::IronTool) > 0,
        ),
    ));

    v.push((
        get_item(ItemId::Currency),
        increase_or_unlock(
            count_by_id(items, ItemId::Currency),
            count_by_id(items, ItemId::BronzeTool),
            count_by_id(items, ItemId::Feudal) > 0 && count_by_id(items, ItemId::BronzeTool) > 0,
        ),
    ));
    v.push((
        get_item(ItemId::GatheringAndHunting),
        increase_or_unlock(count_by_id(items, ItemId::GatheringAndHunting), 0, false),
    ));
    v.push((
        get_item(ItemId::Fishery),
        increase_or_unlock(count_by_id(items, ItemId::Fishery), 0, false),
    ));
    v.push((
        get_item(ItemId::Trading),
        increase_or_unlock(
            count_by_id(items, ItemId::Trading),
            0,
            count_by_id(items, ItemId::Monarchy) > 0 && count_by_id(items, ItemId::Currency) > 4,
        ),
    ));
    v.push((
        get_item(ItemId::Industrialization),
        increase_or_unlock(
            count_by_id(items, ItemId::Industrialization),
            0,
            count_by_id(items, ItemId::SteamPower) > 4,
        ),
    ));

    // filter items that appear zero times
    v.into_iter()
        .filter(|(_, cnt)| *cnt != 0)
        .collect::<Vec<_>>()
}

// use events here so this doesn't run once a frame?
pub fn combine_items_system(
    mut commands: Commands,
    mut spawn_event_writer: EventWriter<SpawnItemEvent>,
    mut audio: EventWriter<SoundEvent>,
    recipes_data: Res<RecipesData>,
    items_data: Res<ItemsData>,
    grid: Res<GridData>,
    combine_button_query: Query<&MouseInteractive, With<CombineButton>>,
    crafting_items_query: Query<(Entity, &Item), With<CraftItem>>,
    items_query: Query<&Coords, With<Item>>,
) {
    if let Ok(combine_button) = combine_button_query.get_single() {
        if combine_button.clicked {
            let number_of_crafting_items = crafting_items_query.iter().count();
            if number_of_crafting_items <= 1 {
                return;
            }

            let mut items = Vec::new();
            for (_, item) in crafting_items_query.iter() {
                items.push(item.clone());
            }

            let possible_recipe = try_get_recipe(&recipes_data, &items);
            trace!("found possible recipe: {:?}", possible_recipe);

            if let Some(recipe) = possible_recipe {
                // debug!("found recipe: {:?}", recipe);
                if let Some((dimens, item)) = items_data.try_get_item(recipe.result) {
                    // debug!("got random item: {:?}", item);

                    if let Some(free_coords) = find_free_space(&grid, dimens, &items_query, &[]) {
                        // ^ this is failing
                        debug!("found free space to place the item");
                        // Spawn the result of the recipe
                        spawn_event_writer.send(SpawnItemEvent::new(
                            item,
                            free_coords,
                            grid.center_crafting(),
                            true,
                        ));
                        // Delete the craft items entities
                        for (entity, _) in crafting_items_query.iter() {
                            commands.entity(entity).despawn_recursive();
                        }
                        // Alchemy sound not working or extremely low volume?
                        audio.send(SoundEvent::Sfx(SoundId::CombineAlchemy))
                    } else {
                        warn!("Tried to find free space but failed.");
                    }
                }
            } else {
                audio.send(SoundEvent::Sfx(SoundId::CombineCant))
            }
        }
    }
}

pub fn try_get_recipe(data: &RecipesData, items: &Vec<Item>) -> Option<Recipe> {
    let mut possible_recipe: Option<Recipe> = None;

    let mut flat_recipe = Vec::<ItemId>::new();
    let items_ids: Vec<ItemId> = items.into_iter().map(|f| f.id.clone()).collect();

    for recipe in &data.recipes {
        flat_recipe.clear();
        for ingr in &recipe.ingredients {
            for _ in 0..(ingr.quantity) {
                flat_recipe.push(ingr.item_id.clone());
            }
        }
        let difference: Vec<_> = items_ids
            .clone()
            .into_iter()
            .filter(|item| !flat_recipe.contains(item))
            .collect();
        if difference.len() == 0 {
            possible_recipe = Option::from(recipe.clone());
            break;
        }
    }

    possible_recipe
}
