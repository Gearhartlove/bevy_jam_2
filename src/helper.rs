use bevy::prelude::*;
use bevy::render::camera::RenderTarget;

pub struct HelperPlugin;

impl Plugin for HelperPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameHelper>()
            .add_system_to_stage(CoreStage::PreUpdate, update_mouse_world_pos);
    }
}

//==================================================================================================
//                          Helper Resource
//==================================================================================================

#[derive(Default)]
pub struct GameHelper {
    mouse_world_pos : Vec2
}

impl GameHelper {
    pub fn mouse_world_pos(&self) -> Vec2 {
        self.mouse_world_pos
    }
}

fn update_mouse_world_pos(
    mut game_info : ResMut<GameHelper>,
    windows : Res<Windows>,
    cam : Query<(&Camera, &GlobalTransform)>
) {
    let (camera, camera_transform) = cam.single();

    let wnd = if let RenderTarget::Window(id) = camera.target {
        windows.get(id).unwrap()
    } else {
        windows.get_primary().unwrap()
    };

    if let Some(screen_pos) = wnd.cursor_position() {
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
        game_info.mouse_world_pos = world_pos.truncate();
    }
}