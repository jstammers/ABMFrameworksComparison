use core::fmt;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::sparse_object_grid_2d::SparseGrid2D;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::rand::rngs::StdRng;
use krabmaga::rand::seq::SliceRandom;
use krabmaga::rand::Rng;
use krabmaga::rand::SeedableRng;
use std::any::Any;
use std::cell::RefCell;
use std::hash::{Hash, Hasher};

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
    pub happy: bool,
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
        write!(f, "{} group {} happy {}", self.id, self.group, self.happy)
    }
}

pub struct World {
    pub step: u64,
    pub field: SparseGrid2D<Patch>,
    pub dim: (i32, i32),
    pub num_agents: u32,
    pub min_to_be_happy: u32,
    pub radius: i32,
    pub rng: StdRng,
}

impl World {
    pub fn new(
        dim: (i32, i32),
        num_agents: u32,
        min_to_be_happy: u32,
        radius: i32,
        seed: u64,
    ) -> World {
        World {
            step: 0,
            field: SparseGrid2D::new(dim.0, dim.1),
            dim,
            num_agents,
            min_to_be_happy,
            radius,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    fn random_empty_cell(&mut self) -> Option<Int2D> {
        let mut empties = Vec::new();
        for y in 0..self.dim.1 {
            for x in 0..self.dim.0 {
                let loc = Int2D { x, y };
                if self.field.get_objects(&loc).is_none() {
                    empties.push(loc);
                }
            }
        }
        if empties.is_empty() {
            None
        } else {
            let idx = self.rng.random_range(0..empties.len());
            Some(empties[idx])
        }
    }

    fn random_ordered_ids(&mut self) -> Vec<u32> {
        let ids: RefCell<Vec<u32>> = RefCell::new(Vec::new());
        self.field.iter_objects_unbuffered(|_, value| {
            ids.borrow_mut().push(value.id);
        });
        let mut ids = ids.into_inner();
        ids.shuffle(&mut self.rng);
        ids
    }

    fn locate_by_id(&self, id: u32) -> Option<(Patch, Int2D)> {
        let found: RefCell<Option<(Patch, Int2D)>> = RefCell::new(None);
        self.field.iter_objects_unbuffered(|loc, value| {
            if value.id == id {
                *found.borrow_mut() = Some((*value, *loc));
            }
        });
        found.into_inner()
    }

    fn count_similar_neighbors(&self, patch: Patch, loc: Int2D) -> u32 {
        let mut similar = 0;
        for dy in -self.radius..=self.radius {
            for dx in -self.radius..=self.radius {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nloc = Int2D {
                    x: loc.x + dx,
                    y: loc.y + dy,
                };
                if nloc.x < 0 || nloc.y < 0 || nloc.x >= self.dim.0 || nloc.y >= self.dim.1 {
                    continue;
                }
                if let Some(neighbors) = self.field.get_objects(&nloc) {
                    let neighbor = neighbors[0];
                    if neighbor.group == patch.group {
                        similar += 1;
                    }
                }
            }
        }
        similar
    }
}

impl State for World {
    fn update(&mut self, _step: u64) {
        let ids = self.random_ordered_ids();

        for id in ids {
            let Some((mut patch, loc)) = self.locate_by_id(id) else {
                continue;
            };

            let similar = self.count_similar_neighbors(patch, loc);
            if similar >= self.min_to_be_happy {
                patch.happy = true;
                self.field.set_object_location(patch, &loc);
            } else {
                patch.happy = false;
                if let Some(new_loc) = self.random_empty_cell() {
                    self.field.remove_object_location(patch, &loc);
                    self.field.set_object_location(patch, &new_loc);
                } else {
                    self.field.set_object_location(patch, &loc);
                }
            }
        }

        self.field.lazy_update();
        self.step += 1;
    }

    fn reset(&mut self) {
        self.step = 0;
        self.field = SparseGrid2D::new(self.dim.0, self.dim.1);
    }

    fn init(&mut self, _schedule: &mut Schedule) {
        self.step = 0;
        for i in 0..self.num_agents {
            let group = if i < self.num_agents / 2 {
                Group::One
            } else {
                Group::Two
            };

            if let Some(loc) = self.random_empty_cell() {
                self.field.set_object_location(
                    Patch {
                        id: i,
                        group,
                        happy: false,
                    },
                    &loc,
                );
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
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
}
