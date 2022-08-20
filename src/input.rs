use bevy::prelude::*;
use bevy::sprite::Rect;
use heron::prelude::*;
use heron::rapier_plugin::{PhysicsWorld, ShapeCastCollisionType};
use crate::{AppState, PhysLayer};

pub struct InputsPlugin;

impl Plugin for InputsPlugin {
    // named "Inputs" to avoid confusing with bevy's InputPlugin
    fn build(&self, app: &mut App) {
        // I'm executing those only during gameplay, since egui doesn't rely on them
        app.init_resource::<MousePosition>();
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(world_cursor_system)
                .with_system(process_drag_attempt)
                .with_system(process_fresh_drag)
                .with_system(move_dragged_item)
                .with_system(process_drag_end)
        );
    }
}

#[derive(Default)]
pub struct MousePosition {
    pub position: Vec2,
}

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct Draggable;

#[derive(Component, Default)]
pub struct DragStart(pub Vec2);

#[derive(Component)]
pub struct BeingDragged {
    pub offset: Vec2,
}

pub fn world_cursor_system(
    mut mouse_position: ResMut<MousePosition>,
    windows: Res<Windows>,
    query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    if let Ok((camera, camera_transform)) = query.get_single() {
        let window = windows.get_primary().unwrap();

        if let Some(screen_pos) = window.cursor_position() {
            let window_size = Vec2::new(window.width() as f32, window.height() as f32);
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE; // What the heck does ndc stand for?
            let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
            let world_position = ndc_to_world.project_point3(ndc.extend(-1.0));
            let world_position: Vec2 = world_position.truncate();

            mouse_position.position = world_position;
        }
    }
}

// WARNING: There's no logic separating normal clicks from drag initiation.
// Needs to be included down the road.

// Cast a ray to check for draggable items
pub fn process_drag_attempt(
    mouse_buttons: Res<Input<MouseButton>>,
    mouse_position: Res<MousePosition>,
    physics_world: PhysicsWorld,
    mut commands: Commands) {
    if !mouse_buttons.just_pressed(MouseButton::Left) { return; }

    let hit = physics_world.ray_cast(
        mouse_position.position.extend(20.0),
        -Vec3::Z * 25.0,
        false);

    // Can't get info about other components without a query, so tag the object for one more processing step
    if let Some(collision) = hit {
        commands.entity(collision.entity).insert(DragStart(collision.collision_point.truncate()));
    }
}

// Take over from the raycast, calculate mouse offset, tag the item for actual drag
pub fn process_fresh_drag(
    mut commands: Commands,
    entity_query: Query<Entity, With<DragStart>>,
    drag_query: Query<(&Transform, &DragStart)>,
) {
    let mut offset = Vec2::ZERO;
    if let Ok((drag_entity_transform, drag_start_component)) = drag_query.get_single() {
        offset = drag_start_component.0 - drag_entity_transform.translation.truncate();
    }

    if let Ok(e) = entity_query.get_single() {
        commands.entity(e).remove::<DragStart>();
        commands.entity(e).insert(BeingDragged {
            offset
        });
    }
}

// Move the item respecting mouse offset, Z for moving items is set to 2.0 to avoid Z-fighting
pub fn move_dragged_item(mouse_position: Res<MousePosition>, mut query: Query<(&mut Transform, &BeingDragged)>) {
    if let Ok((mut dragged_entity_transform, being_dragged_component)) = query.get_single_mut() {
        dragged_entity_transform.translation = mouse_position.position.extend(2.0) - being_dragged_component.offset.extend(0.0);
    }
}

// Remove the tag, restore item to Z = 1.0
pub fn process_drag_end(
    mut commands: Commands,
    mouse_buttons: Res<Input<MouseButton>>,
    entity_query: Query<Entity, With<BeingDragged>>,
    mut transform_query: Query<&mut Transform, With<BeingDragged>>,
) {
    if !mouse_buttons.just_released(MouseButton::Left) { return; }

    if let Ok(mut dragged_entity_transform) = transform_query.get_single_mut() {
        let mut dragged_entity_position = dragged_entity_transform.translation;
        dragged_entity_position.z = 1.0;
        dragged_entity_transform.translation = dragged_entity_position;
    }

    if let Ok(dragged_entity) = entity_query.get_single() {
        commands.entity(dragged_entity).remove::<BeingDragged>();
    }
}



