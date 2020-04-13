use legion::prelude::*;
use wasm_bindgen::prelude::*;

// Define our entity data types
#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Velocity {
    dx: f32,
    dy: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Model(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
struct Static;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Event {
    pub x: f32,
    pub y: f32,
}

pub struct Simulation {
    universe: Universe,
    world: World,
}

impl Simulation {
    pub fn new() -> Self {
        // Create a world to store our entities
        let universe = Universe::new();
        let world = universe.create_world();

        Simulation { universe, world }
    }

    pub fn setup(&mut self) {
        // Create entities with `Position` and `Velocity` data
        self.world.insert(
            (),
            (0..3).enumerate().map(|(_, i)| {
                (
                    Position { x: 0.0, y: 0.0 },
                    Velocity {
                        dx: i as f32,
                        dy: i as f32,
                    },
                )
            }),
        );

        // Create entities with `Position` data and a shared `Model` data, tagged as `Static`
        // Shared data values are shared across many entities,
        // and enable further batch processing and filtering use cases
        self.world.insert(
            (Model(5), Static),
            (0..999).map(|_| (Position { x: 0.0, y: 0.0 },)),
        );
    }

    pub fn tick(&mut self) -> Vec<Event> {
        // Create a query which finds all `Position` and `Velocity` components
        let query = <(Write<Position>, Read<Velocity>)>::query();

        let mut events = Vec::new();

        // Iterate through all entities that match the query in the world
        for (mut pos, vel) in query.iter(&mut self.world) {
            pos.x += vel.dx;
            pos.y += vel.dy;

            if vel.dx > 0. || vel.dy > 0. {
                events.push(Event { x: pos.x, y: pos.y })
            }
        }

        events
    }
}
