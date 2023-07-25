use nalgebra::Vector2;
use notan::app::Event;
use notan::draw::*;
use notan::prelude::*;

use crate::{
    systems::*,
    state::*
};

#[cfg(test)]
mod tests;

mod systems;
mod state;

#[notan_main]
fn main() -> Result<(), String>{
    notan::init_with(SimState::new)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

pub fn update(app: &mut App, state: &mut SimState) {
    //state.world.get_resource_mut::<RenderResources>().unwrap().zoom += (app.mouse.wheel_delta.y/1000.0);
    //get mouse pos as vector2
    let current_m = {
        let m = app.mouse.position();
        Vector2::new(m.0, m.1)
    };

    //if dragging with left click
    if app.mouse.left_is_down(){
        let mouse_delta = current_m-state.last_mouse;
        //change in-world camera pos accordingly
        state.world.get_resource_mut::<RenderResources>().unwrap().cam_pos += mouse_delta;
    }

    //record current mouse pos as last
    state.last_mouse = current_m;
    //update world
    state.update_schedule.run(&mut state.world);
}

pub fn draw(app: &mut App, gfx: &mut Graphics, state: &mut SimState){
    state.world.get_resource_mut::<NotanDraw>().unwrap().0 = gfx.create_draw();
    state.draw_schedule.run(&mut state.world);
    gfx.render(&state.world.get_resource::<NotanDraw>().unwrap().0);
}