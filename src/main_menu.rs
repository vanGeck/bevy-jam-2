use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionSet};

use crate::states::{handle_state_transition, MenuTransition};
use crate::{AppState, DebugConfig};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system_set(
            AppState::MainMenu,
            ConditionSet::new()
                .run_in_state(AppState::MainMenu)
                .with_system(check_menu_bypass)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::MainMenu)
                .with_system(track_backpack_hover)
                .with_system(handle_state_transition)
                .into(),
        )
        .add_exit_system_set(
            AppState::MainMenu,
            ConditionSet::new().run_in_state(AppState::MainMenu).into(),
        );
    }
}

pub fn check_menu_bypass(mut query: Query<&mut MenuBackpack>, mut config: ResMut<DebugConfig>) {
    if config.skip_straight_to_game {
        config.skip_straight_to_game = false;
        query.single_mut().transition = MenuTransition::menu_to_game();
    }
}

#[derive(Component, Default)]
pub struct MenuBackpack {
    pub transition: MenuTransition,
}

//todo: hover

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
    input: Res<Input<MouseButton>>,
    mut query: Query<&mut MenuBackpack>,
    // mut commands: Commands,
    // mut audio: EventWriter<SoundEvent>,
    // input: Res<Input<MouseButton>>,
    // mouse: Res<Mouse>,
    // mut queries: ParamSet<(
    //     Query<(&Transform, &Backpack)>,
    //     Query<(&mut Transform, &BackpackFlap, &mut TextureAtlasSprite)>,
    // )>,
) {
    if input.just_pressed(MouseButton::Left) {
        query.single_mut().transition = MenuTransition::menu_to_game();
    }
    // let mouse_hovers_over_backpack =
    //     queries
    //         .p0()
    //         .get_single()
    //         .map_or(false, |(transform, backpack)| {
    //             mouse.position.x > transform.translation.x - backpack.dimens.x * 0.5
    //                 && mouse.position.x < transform.translation.x + backpack.dimens.x * 0.5
    //                 && mouse.position.y > transform.translation.y - backpack.dimens.y * 0.5
    //                 && mouse.position.y < transform.translation.y + backpack.dimens.y * 0.5
    //         });
    //
    // if mouse_hovers_over_backpack && input.just_pressed(MouseButton::Left) {
    //     audio.send(SoundEvent::Sfx(SoundId::Placeholder));
    //     commands.insert_resource(NextState(AppState::InGame))
    // }
    //
    // if let Ok((mut flap_transform, flap, mut sprite)) = queries.p1().get_single_mut() {
    //     if mouse_hovers_over_backpack {
    //         sprite.index = 1;
    //         flap_transform.translation.y = flap.height + flap.dimens.y - 1.;
    //     } else {
    //         sprite.index = 0;
    //         flap_transform.translation.y = flap.height;
    //     }
    // }
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
