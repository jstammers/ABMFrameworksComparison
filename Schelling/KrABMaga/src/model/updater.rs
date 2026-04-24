use crate::model::world::{Patch, World};
use core::fmt;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::ScheduleOptions;
use krabmaga::engine::state::State;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy)]
pub struct Updater {
    pub id: u32,
}

impl Agent for Updater {
    fn step(&mut self, state: &mut dyn State) {
        let real_state = state.as_any().downcast_ref::<World>().unwrap();
        let updates = RefCell::new(Vec::<(Patch, Int2D)>::new());

        real_state.field.iter_objects(|loc, value| {
            let x = loc.x;
            let y = loc.y;
            let mut similar = 0;

            for dy in -real_state.radius..=real_state.radius {
                for dx in -real_state.radius..=real_state.radius {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nloc = Int2D {
                        x: x + dx,
                        y: y + dy,
                    };
                    if nloc.x < 0
                        || nloc.y < 0
                        || nloc.x >= real_state.dim.0
                        || nloc.y >= real_state.dim.1
                    {
                        continue;
                    }

                    if let Some(neighbors) = real_state.field.get_objects(&nloc) {
                        let neighbor = neighbors[0];
                        if value.group == neighbor.group {
                            similar += 1;
                        }
                    }
                }
            }

            let mut updates = updates.borrow_mut();
            if similar < real_state.min_to_be_happy {
                match real_state.field.get_random_empty_bag() {
                    Some(rloc) => updates.push((*value, rloc)),
                    None => updates.push((*value, *loc)),
                }
            } else {
                updates.push((*value, *loc));
            }
        });

        let mut updates = updates.borrow_mut();
        for (obj, loc) in updates.iter() {
            real_state.field.set_object_location(*obj, loc);
        }
        updates.clear();
    }

    fn before_step(
        &mut self,
        _state: &mut dyn State,
    ) -> Option<Vec<(Box<dyn Agent>, ScheduleOptions)>> {
        None
    }

    fn after_step(
        &mut self,
        _state: &mut dyn State,
    ) -> Option<Vec<(Box<dyn Agent>, ScheduleOptions)>> {
        None
    }
}

impl Hash for Updater {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl fmt::Display for Updater {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Eq for Updater {}

impl PartialEq for Updater {
    fn eq(&self, other: &Updater) -> bool {
        self.id == other.id
    }
}
