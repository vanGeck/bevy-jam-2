use crate::game::GameResult;
use crate::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::GameEnded).with_system(draw_game_over_screen),
        );
    }
}

fn draw_game_over_screen(
    mut egui_context: ResMut<EguiContext>,
    windows: ResMut<Windows>,
    mut state: ResMut<State<AppState>>,
    result: Res<State<GameResult>>,
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
                match *result.current() {
                    GameResult::Won => "YOU WON!",
                    GameResult::Lost => "YOU LOST!",
                },
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
                egui::Button::new("Restart game"),
            );
            if start_btn.clicked() {
                state.replace(AppState::InGame).ok();
            }
            let quit_btn = ui.put(
                Rect::from_center_size(pos2(win_wi / 2., win_ht / 2. + 132.), vec2(280., 66.)),
                egui::Button::new("Back to menu"),
            );
            if quit_btn.clicked() {
                state.replace(AppState::MainMenu).ok();
            }
        });
}
