use bevy_ecs::prelude::*;
use notan::draw::*;
use notan::prelude::*;

use crate::calculations::*;

//renders all entities with associated Position, Texture, and Size
pub fn rendering(mut notan_draw:ResMut<NotanDraw>,render_resources: Res<RenderResources>,query: Query<(&Position,&Size, &TextureId)>){
    notan_draw.0.clear(Color::BLACK);

    for (pos,size, texture_id) in &query{
        let tex = &render_resources.textures[texture_id.0];

        notan_draw.0.image(tex)
            .size(size.0.x, size.0.y)
            .position(pos.0.x, pos.0.y);
    }
}