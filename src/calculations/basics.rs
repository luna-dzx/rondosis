use bevy_ecs::prelude::*;
use nalgebra::{
    Vector2,
};

#[derive(Component)]
pub struct Position(Vector2<f32>);

#[derive(Component)]
pub struct Velocity(Vector2<f32>);

pub fn movement(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        position.0.x += velocity.0.x;
        position.0.y += velocity.0.y;
    }
}
