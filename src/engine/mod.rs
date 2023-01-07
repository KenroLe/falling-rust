pub mod physics;
pub mod game;
#[derive(Clone,Copy)]
pub struct Point {
    pub color:u32,
    pub kinematic:bool,
    pub occupied:bool,
    pub entity:Entity
}
#[derive(Clone,Copy)]
pub struct Entity {

}

#[derive(Clone,Copy)]
pub struct Vector2 {
    x:usize,
    y:usize,
}

pub const ZEROVECTOR:Vector2= Vector2{
    x:0,
    y:0
};
pub const EMPTY_ENTITY :Entity = Entity{};
pub const EMPTY_POINT :Point = Point{color:0x000000,kinematic:false,occupied:false, entity:EMPTY_ENTITY};
pub const RUST_POINT :Point = Point{color:0xe36414,kinematic:false,occupied:true, entity:EMPTY_ENTITY};