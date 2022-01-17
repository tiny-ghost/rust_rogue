mod components;
mod map;
mod player;

pub use components::*;
pub use map::*;
pub use player::*;
use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        player_input(self, ctx);

        ctx.cls();

        let positions = self.ecs.read_storage::<Position>();

        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();

    gs.ecs.register::<Player>();

    gs.ecs.register::<Renderable>();

    gs.ecs.insert(new_map_test());

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Player {})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .build();

    rltk::main_loop(context, gs)
}

pub struct State {
    ecs: World,
}
