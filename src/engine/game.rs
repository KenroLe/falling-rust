use minifb::{Window, MouseButton, MouseMode};

pub fn tick(world: &mut Vec<Vec<crate::engine::Point>>,window:&Window) {
    if window.get_mouse_down(MouseButton::Left) {
        let opt = window.get_mouse_pos(MouseMode::Discard);
        match opt {
            Some(pos) => {
                let world_pos = window_to_world_pos(pos);
                world[world_pos.x][world_pos.y] = crate::engine::Point{color:0xe36414,kinematic:false,occupied:true};
            },
            None => {}
        }
    }
}
fn window_to_world_pos(window_pos:(f32,f32)) -> crate::engine::Vector2 {
    return crate::engine::Vector2 { x: window_pos.0 as usize/crate::WRATIO, y: window_pos.1 as usize/crate::HRATIO }
}