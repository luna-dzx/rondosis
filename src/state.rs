use bevy_ecs::prelude::*;
use notan::draw::*;
use notan::prelude::*;
use nalgebra::Vector2;

use crate::systems::*;

#[derive(Resource)]
pub struct RenderResources{
    pub textures: Vec<Texture>,
    pub cam_pos: Vector2<f32>,
    pub zoom: f32
}

#[derive(Resource,Default)]
pub struct SimResources{
    pub time: f32,
    pub delta: f32
}

#[derive(Resource)]
pub struct NotanDraw(pub Draw);

#[derive(Component)]
pub struct Position(pub Vector2<f32>);

#[derive(Component)]
pub struct Size(pub Vector2<f32>);

#[derive(Component)]
pub struct Velocity(pub Vector2<f32>);

#[derive(Component)]
pub struct TextureId(pub usize);

#[derive(Component)]
pub struct Gestation{
    pub period:f32,
    pub progress: f32
}

#[derive(AppState)]
pub struct SimState {
    pub world: World,
    pub update_schedule: Schedule,
    pub draw_schedule: Schedule,

    pub last_mouse: Vector2<f32>
}
impl SimState {
    pub fn new(gfx: &mut Graphics) -> Self {
        let texture = gfx
            .create_texture()
            .from_image(include_bytes!("assets/mycelium_froge.png"))
            .build()
            .unwrap();

        let textures = vec![texture];

        //define the world
        let mut world = World::new();

        //define each schedule
        //add systems to be ran on update
        let mut update_schedule = Schedule::new();
        update_schedule.add_system(wander);
        update_schedule.add_system(grow);
        //add systems to be ran on draw
        let mut draw_schedule = Schedule::new();
        draw_schedule.add_system(rendering::render);

        //add resources
        let render_resources = RenderResources{
            textures,
            cam_pos: Vector2::zeros(),
            zoom: 1.0,
        };
        world.insert_resource(render_resources);
        world.insert_resource(NotanDraw(gfx.create_draw()));
        world.insert_resource(SimResources::default());

        //spawn entities
        world.spawn((
            Position(Vector2::from_vec(vec![500.0,250.0])),
            TextureId(0),
            Size(Vector2::from_vec(vec![100.0,100.0])),
            Gestation{
                period:1000.0,
                progress:0.0
            }
        ));

        Self {
            world,
            update_schedule,
            draw_schedule,
            last_mouse: Vector2::zeros()
        }
    }
}