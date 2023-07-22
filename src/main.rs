use notan::app::Event;
use notan::draw::*;
use notan::prelude::*;

use crate::{
    rendering::*,
    calculations::*
};

#[cfg(test)]
mod tests;

mod rendering;
mod calculations;

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

pub fn draw(app: &mut App, gfx: &mut Graphics, state: &mut SimState) {
    state.draw_schedule.run(&mut state.world);

    let notan_draw = state.world.get_resource::<NotanDraw>().unwrap();

    gfx.render(&notan_draw.0);
}