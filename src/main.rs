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
    notan::init_with(State::new)
        .add_config(DrawConfig)
        .draw(draw)
        .build()
}