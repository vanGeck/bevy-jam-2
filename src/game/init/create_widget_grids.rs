use bevy::prelude::*;
use bevy::text::Text2dBounds;

use crate::config::data_layout::LayoutData;
use crate::game::{
    AssetStorage, CleanupOnGameplayEnd, CombineButton, Eyes, FontId, Iris, TextureId,
};
use crate::mouse::MouseInteractive;
use crate::positioning::{Coords, Depth, Dimens, GridData, Pos};

pub fn create_layout_grids(
    mut commands: Commands,
    layout: Res<LayoutData>,
    assets: Res<AssetStorage>,
) {
    let inventory_x = layout.middle_x();
    let inventory_y = layout.c_mid.toasts.margin_bottom.unwrap_or(0.)
        + layout.c_mid.toasts.height.unwrap()
        + layout.c_mid.inventory.margin_bottom.unwrap_or(0.);
    let inventory_coords = Coords::new(Pos::new(0, 0), Dimens::new(8, 5));
    create_grid(
        &mut commands,
        &assets,
        &inventory_coords.dimens,
        Vec2::new(inventory_x, inventory_y),
    );
    let overseer_width = layout.middle_width();
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

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(overseer_width * 0.18, overseer_height * 0.18)),
                ..default()
            },
            texture: assets.texture(&TextureId::OverseerEyesWhite),
            transform: Transform::from_xyz(
                overseer_x + overseer_width * 0.5,
                overseer_y + overseer_height * 0.5 + 0.32,
                Depth::Grid.z() + 9.,
            ),
            ..default()
        })
        .insert(Name::new("Overseer Eyes White"))
        .insert(Eyes)
        .insert(CleanupOnGameplayEnd);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(overseer_width * 0.16, overseer_height * 0.16)),
                ..default()
            },
            texture: assets.texture(&TextureId::OverseerIris),
            transform: Transform::from_xyz(
                overseer_x + overseer_width * 0.5,
                overseer_y + overseer_height * 0.5 + 0.32,
                Depth::Grid.z() + 10.,
            ),
            ..default()
        })
        .insert(Name::new("Overseer Iris"))
        .insert(Iris)
        .insert(CleanupOnGameplayEnd);

    let x_crafting = layout.right_x() + 0.3333;
    let y_crafting = layout.c_right.crafting_y();
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
#[derive(Component)]
pub struct CombineButtonText;

pub fn create_layout_combine_button(
    mut commands: Commands,
    layout: Res<LayoutData>,
    assets: Res<AssetStorage>,
) {
    let x = layout.right_x();
    let width = layout.right_width();
    let y = layout.c_right.combine_button_y();
    let height = layout.c_right.combine_button_height();
    let dimens_text = Vec2::new(width - 2., 0.6667);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            texture: assets.texture(&TextureId::CombineButton),
            transform: Transform::from_xyz(x + width * 0.5, y + height * 0.5, Depth::Grid.z()),
            ..default()
        })
        .insert(Name::new("Combine Button"))
        .insert(CombineButton {
            coords: Coords {
                pos: Pos::new(18, 8),
                dimens: Dimens::new(14, 4),
            },
        })
        .insert(MouseInteractive::new(Vec2::new(width, height), true))
        .insert(CleanupOnGameplayEnd)
        .with_children(|parent| {
            parent
                .spawn()
                .insert(CombineButtonText)
                .insert_bundle(Text2dBundle {
                    text: Text::from_section(
                        "COMBINE",
                        TextStyle {
                            font: assets.font(&FontId::FiraSansMedium),
                            font_size: 80.0,
                            color: Color::ANTIQUE_WHITE,
                        },
                    )
                    .with_alignment(TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    }),
                    // The max size that it should fit in:
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(
                            dimens_text.x * layout.text_factor,
                            dimens_text.y * layout.text_factor,
                        ),
                    },
                    transform: Transform::from_xyz(
                        0.,  // Centered on parent.
                        0.1, // Slightly offset to account for asymmetric button
                        11., // Relative to parent
                    )
                    .with_scale(Vec3::new(
                        1. / layout.text_factor,
                        1. / layout.text_factor,
                        1.,
                    )),
                    ..default()
                });
        });
}

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
