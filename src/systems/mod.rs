use bevy_ecs::prelude::*;
use nalgebra::Vector2;
use notan::draw::*;
use notan::prelude::*;
use rand::prelude::*;

use crate::state::*;

pub mod rendering;

pub fn grow(sim_resources: ResMut<SimResources>,mut query: Query<(&mut Gestation,&Position,&Size)>,mut commands: bevy_ecs::prelude::Commands){
    for (mut gest,pos,size) in &mut query{
        gest.progress += 1.0;
        if gest.progress>gest.period{
            gest.progress = 0.0;
            
            commands.spawn((
                Position(pos.0.clone()),
                TextureId(0),
                Size(size.0.clone()*0.90),
                Gestation{
                    period:1000.0,
                    progress:0.0
                }
            ));
        }
    }
}

pub fn wander(sim_resources: ResMut<SimResources>,mut query: Query<(&mut Position)>){
    for mut pos in &mut query{
        let new = Vector2::from_fn(|_,_| random::<f32>()-0.5)
            .normalize();
        pos.0 += new;
    }
}
