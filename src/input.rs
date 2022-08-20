use bevy::prelude::*;
use bevy::sprite::Rect;
use heron::prelude::*;
use heron::rapier_plugin::{PhysicsWorld, ShapeCastCollisionType};
use crate::{AppState, PhysLayer};

pub struct InputsPlugin;

impl Plugin for InputsPlugin { // named "Inputs" to avoid confusing with bevy's InputPlugin
    fn build(&self, app: &mut App){
        // I'm executing those only during gameplay, since egui doesn't rely on them
        app.init_resource::<MousePos>();
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
pub struct MousePos {
    pub pos:Vec2
}

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct Draggable;

#[derive(Component, Default)]
pub struct DragStart(pub Vec2);

#[derive(Component)]
pub struct BeingDragged{
    pub offset:Vec2
}

pub fn world_cursor_system(
    mut crs: ResMut<MousePos>,
    wnds: Res<Windows>,
    q_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>
) {
    if let Ok((camera, camera_transform)) = q_camera.get_single() {
        let wnd = wnds.get_primary().unwrap();

        if let Some(screen_pos) = wnd.cursor_position() {
            let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
            let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
            let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
            let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
            let world_pos: Vec2 = world_pos.truncate();

            crs.pos = world_pos;
        }
    }
}

// WARNING: There's no logic separating normal clicks from drag initiation.
// Needs to be included down the road.

// Cast a ray to check for draggable items
pub fn process_drag_attempt(
    keys: Res<Input<MouseButton>>, 
    m_pos: Res<MousePos>,
    physics_world: PhysicsWorld,
    mut cmd: Commands){
    
    if !keys.just_pressed(MouseButton::Left) { return; }
    
    let hit = physics_world.ray_cast(
        m_pos.pos.extend(20.0), 
        -Vec3::Z * 25.0, 
        false);
    
    // Can't get info about other components without a query, so tag the object for one more processing step
    if let Some(collision) = hit {
        cmd.entity(collision.entity).insert(DragStart(collision.collision_point.truncate()));
    }
}

// Take over from the raycast, calculate mouse offset, tag the item for actual drag
pub fn process_fresh_drag(entity_q: Query<Entity, With<DragStart>>, drag_q: Query<(&Transform, &DragStart)>, mut cmd: Commands) {
    let mut offset = Vec2::ZERO;
    if let Ok((tform, drag)) = drag_q.get_single(){
        offset = drag.0 - tform.translation.truncate();
    }

    if let Ok(e) = entity_q.get_single() {
        cmd.entity(e).remove::<DragStart>();
        cmd.entity(e).insert(BeingDragged{
            offset
        });
    }
}

// Move the item respecting mouse offset, Z for moving items is set to 2.0 to avoid Z-fighting
pub fn move_dragged_item(crs: Res<MousePos>, mut q: Query<(&mut Transform, &BeingDragged)>) {
    if let Ok((mut tform, drag)) = q.get_single_mut(){
        tform.translation = crs.pos.extend(2.0) - drag.offset.extend(0.0);
    }
}

// Remove the tag, restore item to Z = 1.0
pub fn process_drag_end(keys: Res<Input<MouseButton>>, 
                        q: Query<Entity, With<BeingDragged>>, 
                        mut q_tform: Query<&mut Transform, With<BeingDragged>>, 
                        mut cmd: Commands) {
    if !keys.just_released(MouseButton::Left) { return; }
    
    if let Ok(mut tform) = q_tform.get_single_mut(){
        let mut pos = tform.translation;
        pos.z = 1.0;
        tform.translation = pos;
    }
    
    if let Ok(e) = q.get_single(){
        cmd.entity(e).remove::<BeingDragged>();
    }
    
}



