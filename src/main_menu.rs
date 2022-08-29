use bevy::prelude::*;
use bevy::window::WindowMode;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet, IntoConditionalSystem};
use iyes_loopless::state::NextState;

use crate::{AppState, DebugConfig, GAME_NAME};
use crate::audio::sound_event::SoundEvent;
use crate::config::data_layout::LayoutData;
use crate::game::{AlbumId, AssetStorage, create_camera, FontId, MENU_ZOOM};
use crate::game::create_backpack::create_layout_background;
use crate::game::create_widget_feed::create_layout_feed;
use crate::game::create_widget_grids::{create_layout_combine_button, create_layout_grids};
use crate::game::create_widget_hero::create_layout_hero;
use crate::game::create_widget_music::create_layout_music;
use crate::game::create_widget_toasts::create_layout_toasts;
use crate::mouse::MouseInteractive;
use crate::positioning::Depth;
use crate::states::delete_all_entities;
use crate::transition_state::MenuTransition;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MainMenuMusicTimer::default())
            .add_enter_system_set(
                AppState::MainMenu,
                ConditionSet::new()
                    .run_in_state(AppState::MainMenu)
                    .with_system(delete_all_entities)
                    .with_system(create_camera)
                    .with_system(create_layout_background)
                    .with_system(create_layout_music)
                    .with_system(create_layout_feed)
                    .with_system(create_layout_grids)
                    .with_system(create_layout_toasts)
                    .with_system(create_layout_combine_button)
                    .with_system(create_layout_hero)
                    .with_system(init_menu)
                    .with_system(play_menu_music.run_if(should_play_music_right_away))
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(AppState::MainMenu)
                    .with_system(check_menu_bypass.run_if(should_check_bypass))
                    .with_system(check_fullscreen.run_if(should_check_fullscreen))
                    .with_system(track_backpack_hover)
                    .with_system(music_countdown_finished)
                    .into(),
            )
            .add_exit_system_set(
                AppState::MainMenu,
                ConditionSet::new()
                    .run_in_state(AppState::MainMenu)
                    .with_system(clean_menu_entities)
                    .into(),
            );
    }
}

pub fn should_check_bypass(config: Res<DebugConfig>) -> bool {
    config.skip_straight_to_game
}

pub fn check_menu_bypass(
    mut commands: Commands,
    mut query: Query<&mut MenuBackpack>,
    mut config: ResMut<DebugConfig>,
) {
    if config.skip_straight_to_game {
        config.skip_straight_to_game = false;
        query.single_mut().transition = MenuTransition::InactiveGame;
        commands.insert_resource(NextState(AppState::InGame));
    }
}

pub fn should_check_fullscreen(config: Res<DebugConfig>) -> bool {
    config.launch_fullscreen
}

pub fn check_fullscreen(mut windows: ResMut<Windows>, mut config: ResMut<DebugConfig>) {
    if config.launch_fullscreen {
        config.launch_fullscreen = false;
        windows.primary_mut().set_mode(if config.launch_fullscreen {
            WindowMode::BorderlessFullscreen
        } else {
            WindowMode::Windowed
        });
    }
}

#[derive(Component, Default)]
pub struct MenuBackpack {
    pub transition: MenuTransition,
}

#[derive(Component)]
pub struct Backpack {
    dimens: Vec2,
}

#[derive(Component)]
pub struct BackpackFlap {
    dimens: Vec2,
    height: f32,
}

pub fn track_backpack_hover(
    mut commands: Commands,
    mut query_backpack: Query<(
        &mut MenuBackpack,
        &mut TextureAtlasSprite,
        &MouseInteractive,
    )>,
) {
    if let Ok((mut backpack, mut sprite, interactive)) = query_backpack.get_single_mut() {
        if interactive.clicked {
            commands.insert_resource(NextState(AppState::Transition));
            backpack.transition = MenuTransition::menu_to_game();
            // TODO: Maybe sound effect?
        }
        if matches!(backpack.transition, MenuTransition::InactiveMenu) {
            sprite.index = if interactive.hovered { 1 } else { 0 };
        }
    }
}

#[derive(Component)]
pub struct MenuEntity;

pub fn init_menu(mut commands: Commands, assets: Res<AssetStorage>, layout: Res<LayoutData>) {
    let menu_screen_dimens = layout.screen_dimens * MENU_ZOOM;
    let screen_center = layout.screen_dimens * 0.5;
    let screen_anchor = screen_center - menu_screen_dimens * 0.5;
    let text_style = TextStyle {
        font: assets.font(&FontId::FiraSansBold),
        font_size: 250.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        horizontal: HorizontalAlign::Center,
        vertical: VerticalAlign::Center,
    };

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(GAME_NAME, text_style).with_alignment(text_alignment),
        transform: Transform::from_translation(Vec3::new(
            screen_anchor.x + menu_screen_dimens.x * 0.2,
            screen_anchor.y + menu_screen_dimens.y * 0.8,
            Depth::Menu.z() + 10.,
        ))
            .with_scale(Vec3::new(
                MENU_ZOOM / layout.text_factor,
                MENU_ZOOM / layout.text_factor,
                1.,
            )),
        ..default()
    });
}

pub fn clean_menu_entities(mut commands: Commands, query: Query<Entity, With<MenuEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// When running the game in the browser, the first half second is always quite stuttery.
/// This is a timer to wait half a second before starting the music.
pub struct MainMenuMusicTimer(Timer, bool);

impl Default for MainMenuMusicTimer {
    fn default() -> Self {
        MainMenuMusicTimer(Timer::from_seconds(1., false), false)
    }
}

pub fn music_countdown_finished(
    mut audio: EventWriter<SoundEvent>,
    time: Res<Time>,
    mut timer: ResMut<MainMenuMusicTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        audio.send(SoundEvent::PlayAlbum(AlbumId::Ominous));
        timer.1 = true;
    }
}

pub fn should_play_music_right_away(timer: Res<MainMenuMusicTimer>) -> bool {
    timer.1
}

pub fn play_menu_music(mut audio: EventWriter<SoundEvent>) {
    audio.send(SoundEvent::PlayAlbum(AlbumId::Ominous));
}
