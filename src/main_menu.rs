use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet, NextState};

use crate::audio::sound_event::SoundEvent;
use crate::game::camera::create_camera;
use crate::game::{
    despawn_gameplay_entities, AssetStorage, CleanupOnGameplayEnd, SoundId, TextureId,
};
use crate::mouse::Mouse;
use crate::positioning::Depth;
use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::MainMenu,
            ConditionSet::new()
                .run_in_state(AppState::MainMenu)
                .with_system(create_camera)
                .with_system(create_menu)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::MainMenu)
                .with_system(track_backpack_hover)
                .into(),
        )
        .add_exit_system_set(
            AppState::MainMenu,
            ConditionSet::new()
                .run_in_state(AppState::MainMenu)
                .with_system(despawn_gameplay_entities)
                .into(),
        );
    }
}

fn create_menu(mut commands: Commands, assets: Res<AssetStorage>) {
    let dimens = Vec2::new(18., 24.);
    let pos = Vec2::new(32., 18.);
    let flap_pos = Vec2::new(3.5, 8.85);
    let flap_dimens = Vec2::splat(dimens.x * 0.63);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(dimens),
                ..default()
            },
            texture: assets.texture(&TextureId::Backpack),
            transform: Transform::from_xyz(pos.x, pos.y, Depth::Background.z()),
            ..Default::default()
        })
        .insert(Backpack { dimens })
        .insert(Name::new("Backpack"))
        .insert(CleanupOnGameplayEnd);

    let flap_height = pos.y - dimens.y * 0.5 + flap_pos.y + flap_dimens.y * 0.5;
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(flap_dimens),
                index: 0,
                ..default()
            },
            texture_atlas: assets.atlas(&TextureId::BackpackFlap),
            transform: Transform::from_xyz(
                pos.x - dimens.x * 0.5 + flap_pos.x + flap_dimens.x * 0.5,
                flap_height,
                Depth::Background.z() + 1.,
            ),
            ..Default::default()
        })
        .insert(BackpackFlap {
            dimens: flap_dimens,
            height: flap_height,
        })
        .insert(Name::new("BackpackFlap"))
        .insert(CleanupOnGameplayEnd);
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
    mut audio: EventWriter<SoundEvent>,
    input: Res<Input<MouseButton>>,
    query_mouse: Query<&Mouse>,
    mut queries: ParamSet<(
        Query<(&Transform, &Backpack)>,
        Query<(&mut Transform, &BackpackFlap, &mut TextureAtlasSprite)>,
    )>,
) {
    let mouse = query_mouse.single();
    let mouse_hovers_over_backpack =
        queries
            .p0()
            .get_single()
            .map_or(false, |(transform, backpack)| {
                mouse.position.x > transform.translation.x - backpack.dimens.x * 0.5
                    && mouse.position.x < transform.translation.x + backpack.dimens.x * 0.5
                    && mouse.position.y > transform.translation.y - backpack.dimens.y * 0.5
                    && mouse.position.y < transform.translation.y + backpack.dimens.y * 0.5
            });

    if mouse_hovers_over_backpack && input.just_pressed(MouseButton::Left) {
        audio.send(SoundEvent::Sfx(SoundId::Placeholder));
        commands.insert_resource(NextState(AppState::InGame))
    }

    if let Ok((mut flap_transform, flap, mut sprite)) = queries.p1().get_single_mut() {
        if mouse_hovers_over_backpack {
            sprite.index = 1;
            flap_transform.translation.y = flap.height + flap.dimens.y - 1.;
        } else {
            sprite.index = 0;
            flap_transform.translation.y = flap.height;
        }
    }
}

// fn draw_main_menu(
//     mut commands: Commands,
//     mut audio: EventWriter<SoundEvent>,
//     mut egui_context: ResMut<EguiContext>,
//     windows: ResMut<Windows>,
// ) {
//     let win_fill = egui_context.ctx_mut().style().visuals.window_fill();
//     let text_col = egui_context.ctx_mut().style().visuals.text_color();
//     let window = windows.get_primary().unwrap();
//     let win_ht = window.height();
//     let win_wi = window.width();
//     let height = 220.0;
//
//     CentralPanel::default()
//         .frame(Frame::none())
//         .show(egui_context.ctx_mut(), |ui| {
//             let rect = ui.max_rect();
//             let painter = ui.painter();
//
//             painter.rect(
//                 // window border
//                 rect.shrink(2.0),
//                 5.0,
//                 win_fill,
//                 Stroke::new(1.0, text_col),
//             );
//             painter.text(
//                 // title text
//                 rect.center_top() + vec2(0.0, height / 2.0),
//                 Align2::CENTER_CENTER,
//                 crate::GAME_NAME,
//                 FontId::proportional(46.0),
//                 text_col,
//             );
//             painter.line_segment(
//                 // divider
//                 [
//                     rect.left_top() + vec2(2.0, height),
//                     rect.right_top() + vec2(-2.0, height),
//                 ],
//                 Stroke::new(1.0, text_col),
//             );
//
//             let start_btn = ui.put(
//                 Rect::from_center_size(pos2(win_wi / 2., win_ht / 2.), vec2(280., 66.)),
//                 egui::Button::new("Start game"),
//             );
//             if start_btn.clicked() {
//                 audio.send(SoundEvent::Sfx(SoundId::Placeholder));
//                 commands.insert_resource(NextState(AppState::InGame))
//             }
//             let _quit_btn = ui.put(
//                 Rect::from_center_size(pos2(win_wi / 2., win_ht / 2. + 132.), vec2(280., 66.)),
//                 egui::Button::new("Quit"),
//             );
//         });
// }
