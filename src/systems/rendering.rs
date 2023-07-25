use bevy_ecs::prelude::*;
use notan::draw::*;
use notan::prelude::*;

use crate::state::*;

//renders all entities with associated Position, Texture, and Size
pub fn render(mut notan_draw:ResMut<NotanDraw>,render_resources: Res<RenderResources>,query: Query<(&Position,&Size, &TextureId)>){
    notan_draw.0.clear(Color::BLACK);

    for (pos,size, texture_id) in &query{
        let tex = &render_resources.textures[texture_id.0];

        notan_draw.0.image(tex)
            .size(size.0.x*render_resources.zoom, size.0.y*render_resources.zoom)
            .position(
                (pos.0.x+render_resources.cam_pos.x)*render_resources.zoom, 
                (pos.0.y+render_resources.cam_pos.y)*render_resources.zoom);
    }
}