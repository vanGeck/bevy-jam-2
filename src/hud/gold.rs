use std::time::Duration;

use bevy::prelude::*;
use bevy::ui::Style;

use crate::game::{CleanupOnGameplayEnd, Player};

#[derive(Component, Default)]
pub struct Gold {
    amount: i32,
    timer: Timer,
}

impl Gold {
    pub fn add(&mut self, amount: i32) {
        self.amount += amount;
    }

    pub fn remove(&mut self, amount: i32) {
        self.amount -= amount;
    }
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
pub struct GoldText;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
pub struct ColorText;

pub fn setup_gold(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Text with multiple sections
    commands
        .spawn_bundle(
            // Create a TextBundle that has a Text with a list of sections.
            TextBundle::from_sections([
                TextSection::new(
                    "Gold: ",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 24.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 24.0,
                        color: Color::GOLD,
                    },
                ),
            ])
            .with_text_alignment(TextAlignment::TOP_LEFT)
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    right: Val::Px(15.),
                    top: Val::Px(15.),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(GoldText)
        .insert(CleanupOnGameplayEnd);
}

pub fn gold_update_system(
    time: Res<Time>,
    mut text_query: Query<&mut Text, With<GoldText>>,
    mut player: ResMut<Player>,
) {
    player.gold.timer.tick(time.delta());
    let mut text = text_query.single_mut();

    if player.gold.timer.finished() {
        player.gold.timer = Timer::new(Duration::from_secs(1), false);
        player.gold.add(10);
        text.sections[1].value = (player.gold.amount).to_string();
    }
}
