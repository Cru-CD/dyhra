use macroquad::prelude::*;

mod world;
mod map;
mod entity;
mod fsm;
use world::*;

#[macroquad::main("BasicShapes")]

async fn main() {
    let mut world = World::new().await;

    loop {
        world.update();
        world.draw();

        next_frame().await;
    }
}
