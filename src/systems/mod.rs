use bevy_ecs::prelude::*;
use nalgebra::Vector2;
use notan::draw::*;
use notan::prelude::*;
use rand::prelude::*;

use crate::state::*;

pub mod rendering;

pub fn wander(sim_resources: ResMut<SimResources>,mut query: Query<(&mut Position)>){
    for mut pos in &mut query{
        let new = Vector2::from_fn(|_,_| random::<f32>()-0.5)
            .normalize();
        pos.0 += new;
    }
}
