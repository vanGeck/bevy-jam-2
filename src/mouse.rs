use bevy::prelude::*;
use iyes_loopless::prelude::ConditionSet;

use crate::game::camera::GameCamera;
use crate::Update;

#[derive(Default)]
pub struct Mouse {
    /// Position in world coordinates.
    pub position: Vec2,
    /// Position in logical pixels in the window.
    pub screen_position: Vec2,
    /// Position in logical pixels in the window inverted (needed for UI)
    pub screen_pos_inverted: Vec2,
    /// Whether or not the mouse is currently in a dragging operation.
    ///
    /// It is handy to store this separately, rather than relying on whether or not the LMB is
    /// held down, because this way we could add an accessibility mode that
    /// allows click-to-start-dragging, click-to-stop-dragging.
    pub is_dragging: bool,
    pub disabled: bool,
    pub out_of_bounds: bool,
}

/// Any entity that that be interacted with by the mouse.
#[derive(Component)]
pub struct MouseInteractive {
    /// The size of the interactive area.
    pub size: Vec2,
    /// Whether this component is clickable.
    /// If true, this will cause the mouse cursor to change when it hovers over this entity.
    pub clickable: bool,
    /// Whether the mouse is currently hovering over this entity.
    pub hovered: bool,
    /// Whether the mouse just clicked this entity.
    /// (Set to true upon signal going up, aka the LMB going down.)
    pub clicked: bool,
    pub ctrl_clicked: bool,
}

impl MouseInteractive {
    pub fn new(size: Vec2, clickable: bool) -> Self {
        Self {
            size,
            clickable,
            hovered: false,
            clicked: false,
            ctrl_clicked: false,
        }
    }
}

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Mouse>()
            .add_stage_before(Update, "mouse_2", SystemStage::parallel())
            .add_stage_before("mouse_2", "mouse_1", SystemStage::parallel())
            .add_system_set_to_stage(
                "mouse_1",
                ConditionSet::new().with_system(calc_mouse_pos).into(),
            )
            .add_system_set_to_stage(
                "mouse_2",
                ConditionSet::new().with_system(track_mouse_hover).into(),
            )
            .add_system_set_to_stage(
                Update,
                ConditionSet::new()
                    .with_system(set_cursor_appearance)
                    .into(),
            );
    }
}

/// References
/// 1. calc_mouse_pos
/// https://bevy-cheatbook.github.io/cookbook/cursor2world.html
///
/// Runs on a separate stage before everything else.
pub fn calc_mouse_pos(
    windows: Res<Windows>,
    mut mouse: ResMut<Mouse>,
    query_cam: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    if let Ok((camera, camera_transform)) = query_cam.get_single() {
        // Bevy will not return anything here if the mouse is out of screen bounds...
        // ... unless a mouse button is pressed, for whatever reason.
        // That's why there's a double check for mouse being out of bounds.
        let window = windows.get_primary().unwrap();
        if let Some(screen_pos) = window.cursor_position() {
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE; // What the heck does ndc stand for?
            let ndc_to_world =
                camera_transform.compute_matrix() * camera.projection_matrix().inverse();
            let world_position = ndc_to_world.project_point3(ndc.extend(-1.0));
            let world_position: Vec2 = world_position.truncate();

            mouse.position = world_position;
            mouse.screen_position = screen_pos;
            mouse.screen_pos_inverted = Vec2::new(screen_pos.x, window.height() - screen_pos.y);
            mouse.out_of_bounds = screen_pos.x < 0.
                || screen_pos.x > window.width()
                || screen_pos.y < 0.
                || screen_pos.y > window.height();
        } else {
            mouse.out_of_bounds = true;
        }
    }
}

/// Runs on a separate stage after cal_mouse_pos but before everything else.
pub fn track_mouse_hover(
    input: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    mouse: Res<Mouse>,
    mut query: Query<(&mut MouseInteractive, &GlobalTransform, &Visibility)>,
) {
    let (highest_z, nr_items) = query.iter_mut().fold(
        (-1000. as f32, 0),
        |(highest_z, nr_items), (mut interactive, transform, visibility)| {
            interactive.hovered = !mouse.disabled
                && visibility.is_visible
                && mouse.position.x > transform.translation().x - interactive.size.x * 0.5
                && mouse.position.x < transform.translation().x + interactive.size.x * 0.5
                && mouse.position.y > transform.translation().y - interactive.size.y * 0.5
                && mouse.position.y < transform.translation().y + interactive.size.y * 0.5;
            interactive.clicked = interactive.hovered && input.just_pressed(MouseButton::Left);
            interactive.ctrl_clicked =
                interactive.hovered && input.just_pressed(MouseButton::Left) && keys.pressed(KeyCode::LControl);
            if interactive.hovered {
                (highest_z.max(transform.translation().z), nr_items + 1)
            } else {
                (highest_z, nr_items)
            }
        },
    );
    if nr_items > 1 {
        trace!(
            "{:?} items hovered, culling all but highest at {:?}",
            nr_items,
            highest_z
        );
        query
            .iter_mut()
            .filter(|(interactive, transform, _)| {
                interactive.hovered && (transform.translation().z - highest_z).abs() > f32::EPSILON
            })
            .for_each(|(mut interactive, transform, _)| {
                trace!("Culling {:?}", transform.translation().z);
                interactive.hovered = false;
                interactive.clicked = false;
            });
    }
}

pub fn set_cursor_appearance(
    mut windows: ResMut<Windows>,
    mouse: Res<Mouse>,
    query: Query<&MouseInteractive>,
) {
    if mouse.out_of_bounds {
        return;
    }
    let window = windows.get_primary_mut().unwrap();
    if mouse.disabled {
        window.set_cursor_visibility(false);
    } else {
        window.set_cursor_visibility(true);
        let mouse_can_interact = query
            .iter()
            .any(|interactive| interactive.clickable && interactive.hovered);
        window.set_cursor_icon(if mouse.is_dragging {
            CursorIcon::Grabbing
        } else if mouse_can_interact {
            CursorIcon::Hand
        } else {
            CursorIcon::Default
        });
    }
}
