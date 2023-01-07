use crate::{WIDTH, HEIGHT};

use super::{EMPTY_POINT, Vector2, ZEROVECTOR};

pub fn tick(world: &mut Vec<Vec<crate::engine::Point>>){
    let tmp = world.clone();
    let mut moves:Vec<(Vector2,Vector2)> =  vec![(ZEROVECTOR,ZEROVECTOR);0];
    for x in 0..tmp.len() {
        for y in 0..tmp[x].len()
        {
            if world[x][y].occupied == true && y < HEIGHT-1 {
                // gravity :(
                if y < HEIGHT && world[x][y+1].occupied == false {
                    moves.push(
                        (
                            Vector2{x:x,y:y},
                            Vector2{x:x,y:y+1}
                        )
                    );
                }
                if x > 0 { // out of bounds check because vector panics otherwise
                    if y < HEIGHT && world[x-1][y+1].occupied == false {
                        moves.push(
                            (
                                Vector2{x:x,y:y},
                                Vector2{x:x-1,y:y+1}
                            )
                        );
                    }
                }
                if x < WIDTH-1 { // oob check
                    if y < HEIGHT && world[x+1][y+1].occupied == false {
                        moves.push(
                            (
                                Vector2{x:x,y:y},
                                Vector2{x:x+1,y:y+1}
                            )
                        );
                    }
                }
            }
        }
    }
    for i in 0..moves.len() {
        world[moves[i].1.x][moves[i].1.y] = world[moves[i].0.x][moves[i].0.y];
        world[moves[i].0.x][moves[i].0.y] = EMPTY_POINT;
    }
}