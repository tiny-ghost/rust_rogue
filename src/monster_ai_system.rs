use std::fmt::format;

use super::{Map, Monster, Position, ViewShed};
use rltk::{console, field_of_view, Point};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadStorage<'a, ViewShed>,
        ReadExpect<'a, Point>,
        ReadStorage<'a, Monster>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (viewshed, player_pos, monster) = data;

        for (viewshed, _monster) in (&viewshed, &monster).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(format!("Monster shouts insult"));
            }
        }
    }
}
