use crate::model::updater::Updater;
use core::fmt;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::any::Any;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Group {
    One,
    Two,
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Group::One => write!(f, "One"),
            Group::Two => write!(f, "Two"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Patch {
    pub id: u32,
    pub group: Group,
}

impl Hash for Patch {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl Eq for Patch {}

impl PartialEq for Patch {
    fn eq(&self, other: &Patch) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for Patch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} group {}", self.id, self.group)
    }
}

pub struct World {
    pub step: u64,
    pub field: SparseGrid2D<Patch>,
    pub dim: (i32, i32),
    pub num_agents: u32,
    pub min_to_be_happy: u32,
    pub radius: i32,
}

impl World {
    pub fn new(dim: (i32, i32), num_agents: u32, min_to_be_happy: u32, radius: i32) -> World {
        World {
            step: 0,
            field: SparseGrid2D::new(dim.0, dim.1),
            dim,
            num_agents,
            min_to_be_happy,
            radius,
        }
    }

    pub fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }
}

impl State for World {
    fn update(&mut self, _step: u64) {
        self.field.lazy_update();
    }

    fn reset(&mut self) {
        self.step = 0;
        self.field = SparseGrid2D::new(self.dim.0, self.dim.1);
    }

    fn init(&mut self, schedule: &mut Schedule) {
        self.step = 0;
        let mut rng = rand::rng();

        for i in 0..self.num_agents {
            let xx: i32 = rng.random_range(0..self.dim.0);
            let yy: i32 = rng.random_range(0..self.dim.1);

            if i < self.num_agents / 2 {
                self.field.set_object_location(
                    Patch {
                        id: i,
                        group: Group::One,
                    },
                    &Int2D { x: xx, y: yy },
                );
            } else {
                self.field.set_object_location(
                    Patch {
                        id: i,
                        group: Group::Two,
                    },
                    &Int2D { x: xx, y: yy },
                );
            }
        }

        let agent = Updater { id: 0 };
        schedule.schedule_repeating(Box::new(agent), 0.0, 0);
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }

    fn before_step(&mut self, _schedule: &mut Schedule) {}

    fn after_step(&mut self, _schedule: &mut Schedule) {
        self.step += 1;
    }

    fn end_condition(&mut self, _schedule: &mut Schedule) -> bool {
        false
    }
}
