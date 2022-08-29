use bevy::prelude::*;

use crate::config::data_layout::LayoutData;
use crate::game::CleanupOnGameplayEnd;
use crate::game::dungeon_sim::ContinuePrompt;
use crate::positioning::Depth;
use crate::game::{AssetStorage, FontId};

pub fn create_layout_toasts(
    mut commands: Commands,
    layout: Res<LayoutData>,
    assets: Res<AssetStorage>,
) {
    let x = layout.middle_x();
    let width = layout.middle_width();
    let y = layout.c_mid.toasts.margin_bottom.unwrap_or(0.);
    let height = layout.c_mid.toasts.height.unwrap();
    let text_alignment = TextAlignment {
        horizontal: HorizontalAlign::Center,
        vertical: VerticalAlign::Center,
    };
    let text_style = TextStyle {
        font: assets.font(&FontId::FiraSansBold),
        font_size: 80.0,
        color: Color::ANTIQUE_WHITE,
    };

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
        .insert(CleanupOnGameplayEnd)
        .with_children(|parent| {
            parent
                .spawn()
                .insert(ContinuePrompt)
                .insert(CleanupOnGameplayEnd)
                .insert_bundle(Text2dBundle {
                    text: Text::from_section("", text_style.clone()).with_alignment(text_alignment),
                    transform: Transform::from_xyz(
                        0., // Centered on parent.
                        0.,
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
