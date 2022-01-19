mod components;
mod map;
mod player;
mod rect;
mod visibility_system;

pub use components::*;
pub use map::*;
pub use player::*;
pub use rect::Rect;
use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
use visibility_system::VisibilitySystem;

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();

    gs.ecs.register::<Player>();

    gs.ecs.register::<Renderable>();

    gs.ecs.register::<ViewShed>();

    let map = new_map_rooms_and_corridors();
    let (p_x, p_y) = map.rooms[0].center();

    gs.ecs.insert(map);

    gs.ecs
        .create_entity()
        .with(Position { x: p_x, y: p_y })
        .with(Player {})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(ViewShed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    rltk::main_loop(context, gs)
}

pub struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        player_input(self, ctx);
        self.run_systems();
        ctx.cls();

        let positions = self.ecs.read_storage::<Position>();

        let renderables = self.ecs.read_storage::<Renderable>();
        draw_map(&self.ecs, ctx);

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
