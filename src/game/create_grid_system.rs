use std::time::Duration;

use bevy::prelude::*;
use bevy::text::Text2dBounds;

use crate::animation::AnimationTimer;
use crate::audio::sound_event::AudioTextDisplay;
use crate::config::data_layout::LayoutData;
use crate::game::{AssetStorage, CleanupOnGameplayEnd, FontId, TextureId};
use crate::main_menu::MenuBackpack;
use crate::positioning::Depth;
use crate::positioning::Dimens;
use crate::positioning::{Coords, GridData, Pos};

pub fn create_layout_background(
    mut commands: Commands,
    layout: Res<LayoutData>,
    assets: Res<AssetStorage>,
) {
    let size = layout.factor * 1.2 * layout.screen_dimens.x.max(layout.screen_dimens.y);
    let pos_x = layout.factor * 0.5 * layout.screen_dimens.x;
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::splat(size)),
                index: 2,
                ..default()
            },
            texture_atlas: assets.atlas(&TextureId::Backpack),
            transform: Transform::from_xyz(pos_x, -5. + size * 0.5, Depth::Background.z()),
            ..default()
        })
        .insert(CleanupOnGameplayEnd);
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
        .insert(CleanupOnGameplayEnd);
}

pub fn create_layout_music(
    mut commands: Commands,
    layout: Res<LayoutData>,
    assets: Res<AssetStorage>,
) {
    let x = layout.factor * layout.left_x();
    let width = layout.factor * layout.left_width();
    let y = layout.factor * (layout.c_left.music.margin_bottom.unwrap_or(0.));
    let height = layout.factor * (layout.c_left.music.height.unwrap());

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 0.8),
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(x + width * 0.5, y + height * 0.5, Depth::Grid.z()),
            ..default()
        })
        .insert(Name::new("MusicArea"))
        .insert(CleanupOnGameplayEnd)
        .with_children(|parent| {
            let pos_box = Vec2::splat(1. * layout.factor);
            let dimens_box = Vec2::splat(2. * layout.factor);
            parent
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        custom_size: Some(dimens_box),
                        index: 0,
                        ..default()
                    },
                    texture_atlas: assets.atlas(&TextureId::RecordPlayer),
                    transform: Transform::from_xyz(
                        pos_box.x + dimens_box.x * 0.5 - width * 0.5,
                        pos_box.y + dimens_box.y * 0.5 - height * 0.5,
                        1., // Relative to parent grid.
                    ),
                    ..default()
                })
                .insert(AnimationTimer {
                    timer: Timer::new(Duration::from_millis(200), true),
                    index: 0,
                    nr_frames: 2,
                    ping_pong: false,
                })
                .insert(Name::new("RecordPlayer"));
            let text_style = TextStyle {
                font: assets.font(&FontId::Square),
                font_size: 60.0,
                color: Color::WHITE,
            };
            let text_alignment = TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Left,
            };

            let pos_text = Vec2::new(4. * layout.factor, 2. * layout.factor);
            let dimens_text = Vec2::new(width - 5. * layout.factor, 2. * layout.factor);
            parent
                .spawn()
                .insert(AudioTextDisplay)
                .insert_bundle(Text2dBundle {
                    // Default text, will probably never be seen:
                    text: Text::from_section(
                        "Click the record player to start the music.",
                        text_style,
                    )
                    .with_alignment(text_alignment),
                    // The max size that it should fit in:
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(
                            dimens_text.x * layout.text_factor,
                            dimens_text.y * layout.text_factor,
                        ),
                    },
                    transform: Transform::from_translation(Vec3::new(
                        pos_text.x - width * 0.5,
                        pos_text.y - height * 0.5,
                        1.0,
                    ))
                    .with_scale(Vec3::new(
                        1. / layout.text_factor,
                        1. / layout.text_factor,
                        1.,
                    )),
                    ..default()
                });
        });
}

pub fn create_layout_feed(mut commands: Commands, layout: Res<LayoutData>) {
    let x = layout.factor * layout.left_x();
    let width = layout.factor * layout.left_width();
    let music_y = layout.c_left.music.margin_bottom.unwrap_or(0.);
    let music_height = layout.c_left.music.height.unwrap();
    let y =
        layout.factor * (music_y + music_height + layout.c_left.feed.margin_bottom.unwrap_or(0.));
    let height =
        layout.factor * (layout.screen_dimens.y - layout.c_left.feed.margin_top.unwrap_or(0.)) - y;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 0.8),
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(x + width * 0.5, y + height * 0.5, Depth::Grid.z()),
            ..default()
        })
        .insert(Name::new("EventFeed"))
        .insert(CleanupOnGameplayEnd);
}

pub fn create_layout_toasts(mut commands: Commands, layout: Res<LayoutData>) {
    let x = layout.factor * layout.middle_x();
    let width = layout.factor * layout.middle_width();
    let y = layout.factor * (layout.c_mid.toasts.margin_bottom.unwrap_or(0.));
    let height = layout.factor * (layout.c_mid.toasts.height.unwrap());
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 0.8),
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(x + width * 0.5, y + height * 0.5, Depth::Grid.z()),
            ..default()
        })
        .insert(Name::new("Toasts"))
        .insert(CleanupOnGameplayEnd);
}

pub fn create_layout_grids(
    mut commands: Commands,
    layout: Res<LayoutData>,
    assets: Res<AssetStorage>,
) {
    let inventory_x = layout.factor * layout.middle_x();
    let inventory_y = layout.factor
        * (layout.c_mid.toasts.margin_bottom.unwrap_or(0.)
            + layout.c_mid.toasts.height.unwrap()
            + layout.c_mid.inventory.margin_bottom.unwrap_or(0.));
    let inventory_coords = Coords::new(Pos::new(0, 0), Dimens::new(8, 5));
    create_grid(
        &mut commands,
        &assets,
        &inventory_coords.dimens,
        Vec2::new(inventory_x, inventory_y),
    );
    let overseer_width = layout.factor * layout.middle_width();
    let overseer_height = overseer_width * 0.3; // Image is 1000x300.
    let inventory_height = 5.;
    let overseer_x = inventory_x;
    let overseer_y = inventory_y + inventory_height - overseer_height * layout.overseer_baseline;
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(overseer_width, overseer_height)),
                ..default()
            },
            texture: assets.texture(&TextureId::Overseer),
            transform: Transform::from_xyz(
                overseer_x + overseer_width * 0.5,
                overseer_y + overseer_height * 0.5,
                Depth::Grid.z() + 10.,
            ),
            ..default()
        })
        .insert(Name::new("Overseer"))
        .insert(CleanupOnGameplayEnd);

    let x_crafting = layout.factor * (layout.right_x() + 1.);
    let foo_y = layout.c_left.music.margin_bottom.unwrap_or(0.);
    let foo_height = layout.c_left.music.height.unwrap();
    let y_crafting =
        layout.factor * (foo_y + foo_height + layout.c_right.crafting.margin_bottom.unwrap_or(0.));
    let crafting_coords = Coords::new(Pos::new(9, 1), Dimens::new(4, 3));
    create_grid(
        &mut commands,
        &assets,
        &crafting_coords.dimens,
        Vec2::new(x_crafting, y_crafting),
    );

    commands.insert_resource(GridData {
        offset: Vec2::new(inventory_x, inventory_y),
        inventory: inventory_coords,
        crafting: crafting_coords,
    });
}

/// Sets up the lower-right container.
/// It's called foo for now because I don't think there's a plan for it.
pub fn create_layout_foo(mut commands: Commands, layout: Res<LayoutData>) {
    let x = layout.factor * layout.right_x();
    let width = layout.factor * layout.right_width();
    let y = layout.factor * (layout.c_left.music.margin_bottom.unwrap_or(0.));
    let height = layout.factor * (layout.c_left.music.height.unwrap());

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 0.8),
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(x + width * 0.5, y + height * 0.5, Depth::Grid.z()),
            ..default()
        })
        .insert(Name::new("MusicArea"))
        .insert(CleanupOnGameplayEnd);
}

pub fn create_layout_hero() {}

fn create_grid(commands: &mut Commands, assets: &AssetStorage, dimens: &Dimens, offset: Vec2) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 0.8),
                custom_size: Some(dimens.as_vec2()),
                ..default()
            },
            transform: Transform::from_xyz(
                offset.x + dimens.x as f32 * 0.5,
                offset.y + dimens.y as f32 * 0.5,
                Depth::Grid.z(),
            ),
            ..default()
        })
        .insert(Name::new("Grid"))
        .insert(CleanupOnGameplayEnd)
        .with_children(|grid| {
            for y in 0..dimens.y {
                for x in 0..dimens.x {
                    grid.spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            // color: Color::rgba(0.8, 0.8, 0.8, 0.5),
                            custom_size: Some(Dimens::unit().as_vec2()),
                            ..default()
                        },
                        texture: assets.texture(&TextureId::TileThirtyTwo),
                        transform: Transform::from_xyz(
                            (x as f32 + 1. * 0.5) - (dimens.x as f32 * 0.5),
                            (y as f32 + 1. * 0.5) - (dimens.y as f32 * 0.5),
                            1., // Relative to parent grid.
                        )
                        .with_scale(Vec3::new(0.9, 0.9, 1.)),
                        ..default()
                    });
                }
            }
        });
}

//
// /// Deprecated. No longer called.
// pub fn create_grids2(mut commands: Commands, config: Res<GridConfig>) {
//     create_grid(&mut commands, &config.inventory);
//     create_grid(&mut commands, &config.crafting);
//
//     commands
//         .spawn_bundle(SpriteBundle {
//             sprite: Sprite {
//                 color: Color::rgba(0.2, 0.2, 0.2, 0.8),
//                 custom_size: Some(config.lower_bar.dimens.as_vec2()),
//                 ..default()
//             },
//             transform: Transform::from_xyz(
//                 config.lower_bar.pos.x as f32 + config.lower_bar.dimens.x as f32 * 0.5,
//                 config.lower_bar.pos.y as f32 + config.lower_bar.dimens.y as f32 * 0.5,
//                 Depth::Grid.z(),
//             ),
//             ..default()
//         })
//         .insert(Name::new("LowerBar"))
//         .insert(CleanupOnGameplayEnd);
//
//     commands
//         .spawn_bundle(SpriteBundle {
//             sprite: Sprite {
//                 color: Color::rgba(0.2, 0.2, 0.2, 0.8),
//                 custom_size: Some(config.combine.dimens.as_vec2()),
//                 ..default()
//             },
//
//             transform: Transform::from_xyz(
//                 config.combine.pos.x as f32 + config.combine.dimens.x as f32 * 0.5,
//                 config.combine.pos.y as f32 + config.combine.dimens.y as f32 * 0.5,
//                 Depth::Grid.z(),
//             ),
//             ..default()
//         })
//         .insert(Name::new("Combine Trigger"))
//         .insert(CombineButton {
//             coords: config.combine,
//         })
//         .insert(CleanupOnGameplayEnd);
//
//     commands
//         .spawn_bundle(SpriteBundle {
//             visibility: Visibility { is_visible: false },
//             sprite: Sprite {
//                 color: Color::rgba(0.2, 0.2, 0.2, 0.8),
//                 custom_size: Some(config.drop_in.dimens.as_vec2()),
//                 ..default()
//             },
//             transform: Transform::from_xyz(
//                 config.drop_in.pos.x as f32 + config.drop_in.dimens.x as f32 * 0.5,
//                 config.drop_in.pos.y as f32 + config.drop_in.dimens.y as f32 * 0.5,
//                 Depth::Grid.z(),
//             ),
//             ..default()
//         })
//         .insert(Name::new("DropIn"))
//         .insert(CleanupOnGameplayEnd);
//
//     let parent_coords = config.equipped.coords;
//     commands
//         .spawn_bundle(SpriteBundle {
//             sprite: Sprite {
//                 color: Color::rgba(0.2, 0.2, 0.2, 0.8),
//                 custom_size: Some(parent_coords.dimens.as_vec2()),
//                 ..default()
//             },
//             transform: Transform::from_xyz(
//                 parent_coords.pos.x as f32 + parent_coords.dimens.x as f32 * 0.5,
//                 parent_coords.pos.y as f32 + parent_coords.dimens.y as f32 * 0.5,
//                 Depth::Grid.z(),
//             ),
//             ..default()
//         })
//         .insert(Name::new("InventoryGrid"))
//         .insert(CleanupOnGameplayEnd)
//         .with_children(|parent| {
//             config.equipped.slots.iter().for_each(|(slot, coords)| {
//                 parent
//                     .spawn_bundle(SpriteBundle {
//                         sprite: Sprite {
//                             color: Color::rgba(0.8, 0.8, 0.8, 0.5),
//                             custom_size: Some(coords.dimens.as_vec2()),
//                             ..default()
//                         },
//                         transform: Transform::from_xyz(
//                             (coords.pos.x as f32 + coords.dimens.x as f32 * 0.5)
//                                 - (parent_coords.dimens.x as f32 * 0.5),
//                             (coords.pos.y as f32 + coords.dimens.y as f32 * 0.5)
//                                 - (parent_coords.dimens.y as f32 * 0.5),
//                             1., // Relative to parent grid.
//                         ),
//                         ..default()
//                     })
//                     .insert(Name::new(format!("{:?}", slot)));
//             });
//         });
// }
