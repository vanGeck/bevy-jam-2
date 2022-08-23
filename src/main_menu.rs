use crate::audio::sound_event::SoundEvent;
use crate::game::SoundId;
use crate::*;
use iyes_loopless::prelude::ConditionSet;
use iyes_loopless::state::NextState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::MainMenu)
                .with_system(draw_main_menu)
                .into(),
        );
    }
}

fn draw_main_menu(
    mut commands: Commands,
    mut audio: EventWriter<SoundEvent>,
    mut egui_context: ResMut<EguiContext>,
    windows: ResMut<Windows>,
) {
    let win_fill = egui_context.ctx_mut().style().visuals.window_fill();
    let text_col = egui_context.ctx_mut().style().visuals.text_color();
    let window = windows.get_primary().unwrap();
    let win_ht = window.height();
    let win_wi = window.width();
    let height = 220.0;

    CentralPanel::default()
        .frame(Frame::none())
        .show(egui_context.ctx_mut(), |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();

            painter.rect(
                // window border
                rect.shrink(2.0),
                5.0,
                win_fill,
                Stroke::new(1.0, text_col),
            );
            painter.text(
                // title text
                rect.center_top() + vec2(0.0, height / 2.0),
                Align2::CENTER_CENTER,
                crate::GAME_NAME,
                FontId::proportional(46.0),
                text_col,
            );
            painter.line_segment(
                // divider
                [
                    rect.left_top() + vec2(2.0, height),
                    rect.right_top() + vec2(-2.0, height),
                ],
                Stroke::new(1.0, text_col),
            );

            let start_btn = ui.put(
                Rect::from_center_size(pos2(win_wi / 2., win_ht / 2.), vec2(280., 66.)),
                egui::Button::new("Start game"),
            );
            if start_btn.clicked() {
                audio.send(SoundEvent::Sfx(SoundId::Placeholder));
                commands.insert_resource(NextState(AppState::InGame))
            }
            let _quit_btn = ui.put(
                Rect::from_center_size(pos2(win_wi / 2., win_ht / 2. + 132.), vec2(280., 66.)),
                egui::Button::new("Quit"),
            );
        });
}
