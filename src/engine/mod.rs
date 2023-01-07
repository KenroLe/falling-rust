pub mod physics;
pub mod game;
#[derive(Clone,Copy)]
pub struct Point {
    pub color:u32,
    pub physics_properties:PhysicsProperties
}
#[derive(Clone,Copy)]
pub struct PhysicsProperties {
    collider:bool,
    has_gravity:bool
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
pub const KINEMATIC_PHYSICS:PhysicsProperties = PhysicsProperties {collider:true, has_gravity:false};
pub const DEFAULT_PHYSICS:PhysicsProperties = PhysicsProperties {collider:true, has_gravity:true};
pub const EMPTY_PHYSICS: PhysicsProperties = PhysicsProperties {collider:false, has_gravity:false};
pub const EMPTY_POINT :Point = Point{color:0x000000, physics_properties:EMPTY_PHYSICS};
pub const RUST_POINT :Point = Point{color:0xe36414, physics_properties:DEFAULT_PHYSICS};
pub const STATIC_POINT :Point = Point{color:0xe37f9f, physics_properties:KINEMATIC_PHYSICS};