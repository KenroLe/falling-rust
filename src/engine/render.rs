pub fn tick(world: &mut Vec<Vec<crate::engine::Point>>,render_buffer: &mut Vec<u32>) {
    for x in 0..world.len() {
        for y in 0..world[x].len() {
            render_buffer[x+y*crate::WIDTH] =  world[x][y].color;
        }
    }
}