use bevy::prelude::*;

use crate::config::data_layout::LayoutData;
use crate::game::backpack::BackpackInUse;
use crate::game::{AssetStorage, CleanupOnGameplayEnd, TextureId, MENU_ZOOM};
use crate::main_menu::MenuBackpack;
use crate::mouse::MouseInteractive;
use crate::positioning::Depth;

pub fn create_layout_background(
    mut commands: Commands,
    layout: Res<LayoutData>,
    assets: Res<AssetStorage>,
) {
    let center = layout.screen_dimens * 0.5;
    let menu_size = MENU_ZOOM * layout.screen_dimens;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(menu_size),
                ..default()
            },
            texture: assets.texture(&TextureId::MenuCaveBg),
            transform: Transform::from_xyz(center.x, center.y, Depth::Background.z()),
            ..default()
        })
        .insert(CleanupOnGameplayEnd);

    let size = 1.2 * layout.screen_dimens.x.max(layout.screen_dimens.y);
    let pos_x = 0.5 * layout.screen_dimens.x;
    // Create the background backpack that will be visible during the game.
    // During the game, not much of this image is visible. But during the transition,
    // this really sells the illusion that you're entering the backpack.
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::splat(size)),
                index: 2,
                ..default()
            },
            texture_atlas: assets.atlas(&TextureId::Backpack),
            transform: Transform::from_xyz(pos_x, -5. + size * 0.5, Depth::Background.z() + 10.),
            ..default()
        })
        .insert(CleanupOnGameplayEnd);
    // Create the clickable menu backpack.
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::splat(size)),
                index: 0,
                ..default()
            },
            texture_atlas: assets.atlas(&TextureId::Backpack),
            transform: Transform::from_xyz(pos_x, -5. + size * 0.5, Depth::Menu.z()),
            ..default()
        })
        .insert(MenuBackpack::default())
        .insert(MouseInteractive::new(Vec2::splat(size), true))
        .insert(CleanupOnGameplayEnd);
}

pub fn create_backpack_data(mut commands: Commands) {
    const INIT_BACKPACK_ID: usize = 0;
    commands.spawn().insert(BackpackInUse(INIT_BACKPACK_ID));
}
