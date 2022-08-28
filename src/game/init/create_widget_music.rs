use std::time::Duration;

use bevy::prelude::*;
use bevy::text::Text2dBounds;

use crate::animation::AnimationTimer;
use crate::audio::record_player::RecordPlayer;
use crate::audio::sound_event::AudioTextDisplay;
use crate::config::data_layout::LayoutData;
use crate::game::{AssetStorage, CleanupOnGameplayEnd, FontId, TextureId};
use crate::mouse::MouseInteractive;
use crate::positioning::Depth;

pub fn create_layout_music(
    mut commands: Commands,
    layout: Res<LayoutData>,
    assets: Res<AssetStorage>,
) {
    let x = layout.factor * layout.left_x();
    let width = layout.factor * layout.left_width();
    let y = layout.factor * layout.c_left.music_y();
    let height = layout.factor * layout.c_left.music_height();

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
            let pos_box = Vec2::new(0.5 * layout.factor, 0.5 * layout.factor);
            let dimens_box = Vec2::splat(4. * layout.factor);
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
                    nr_frames: 1,
                    ping_pong: false,
                })
                .insert(Name::new("RecordPlayer"))
                .insert(RecordPlayer)
                .insert(MouseInteractive::new(dimens_box, true));
            let text_style = TextStyle {
                font: assets.font(&FontId::FiraSansMedium),
                font_size: 60.0,
                color: Color::WHITE,
            };
            let text_alignment = TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Left,
            };

            let pos_text = Vec2::new(5. * layout.factor, 2. * layout.factor);
            let dimens_text = Vec2::new(width - 6. * layout.factor, 2. * layout.factor);
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
