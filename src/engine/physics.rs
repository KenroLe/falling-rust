use crate::{WIDTH, HEIGHT};

use super::{EMPTY_POINT, Vector2, ZEROVECTOR, Point};
struct Move {
    from:Vector2,
    to:Vector2
}
pub fn tick(world: &mut Vec<Vec<Point>>){
    gravity(world);
}
/*
ticks gravity
*/
pub fn gravity(world: &mut Vec<Vec<Point>>) {
    let mut moves:Vec<Move> =  vec![];
    for x in 0..world.len() {
        for y in 0..world[x].len()
        {
            if world[x][y].occupied == true && y < HEIGHT-1 {
                // gravity :(
                if y < HEIGHT && world[x][y+1].occupied == false {
                    moves.push(
                        Move{
                            from:Vector2{x:x,y:y},
                            to:Vector2{x:x,y:y+1}
                        }
                    );
                }
                if x > 0 { // out of bounds check because vector panics otherwise
                    if y < HEIGHT && world[x-1][y+1].occupied == false {
                        moves.push(
                            Move{
                                from:Vector2{x:x,y:y},
                                to:Vector2{x:x-1,y:y+1}
                            }
                        );
                    }
                }
                if x < WIDTH-1 { // oob check
                    if y < HEIGHT && world[x+1][y+1].occupied == false {
                        moves.push(
                            Move{
                                from:Vector2{x:x,y:y},
                                to:Vector2{x:x+1,y:y+1}
                            }
                        );
                    }
                }
            }
        }
    }
    for i in 0..moves.len() {
        world[moves[i].to.x][moves[i].to.y] = world[moves[i].from.x][moves[i].from.y];
        world[moves[i].from.x][moves[i].from.y] = EMPTY_POINT;
    }
}