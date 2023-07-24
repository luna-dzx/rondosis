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
    state.update_schedule.run(&mut state.world);
}

pub fn draw(app: &mut App, gfx: &mut Graphics, state: &mut SimState){
    state.world.get_resource_mut::<NotanDraw>().unwrap().0 = gfx.create_draw();
    state.draw_schedule.run(&mut state.world);
    gfx.render(&state.world.get_resource::<NotanDraw>().unwrap().0);
}