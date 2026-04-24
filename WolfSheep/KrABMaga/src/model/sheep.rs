use crate::model::state::{LifeState, WsgState};
use core::fmt;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
pub struct Sheep {
    pub id: u32,
    pub animal_state: LifeState,
    pub loc: Int2D,
    pub energy: f64,
    pub gain_energy: f64,
    pub prob_reproduction: f64,
}

impl Sheep {
    pub fn new(
        id: u32,
        loc: Int2D,
        energy: f64,
        gain_energy: f64,
        prob_reproduction: f64,
    ) -> Sheep {
        Sheep {
            id,
            loc,
            energy,
            gain_energy,
            prob_reproduction,
            animal_state: LifeState::Alive,
        }
    }
}

impl Agent for Sheep {
    fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any_mut().downcast_mut::<WsgState>().unwrap();
        if self.animal_state == LifeState::Dead {
            return;
        }

        let x = self.loc.x;
        let y = self.loc.y;
        let mut rng = rand::rng();

        let xmin = if x > 0 { -1 } else { 0 };
        let xmax = i32::from(x < state.dim.0 - 1);
        let ymin = if y > 0 { -1 } else { 0 };
        let ymax = i32::from(y < state.dim.1 - 1);

        let nx = rng.random_range(xmin..=xmax);
        let ny = rng.random_range(ymin..=ymax);
        self.loc = Int2D {
            x: x + nx,
            y: y + ny,
        };

        state.sheep_grid.set_object_location(*self, &self.loc);

        if let Some(grass_val) = state.grass_field.get_value(&self.loc) {
            if grass_val >= state.grass_regrowth_time {
                state.grass_field.set_value_location(0, &self.loc);
                self.energy += self.gain_energy;
            }
        }

        self.energy -= 1.0;
        if self.energy < 1.0 {
            self.animal_state = LifeState::Dead;
        } else if rng.random_bool(self.prob_reproduction) {
            self.energy /= 2.0;
            let new_sheep = Sheep::new(
                state.next_id,
                self.loc,
                self.energy,
                self.gain_energy,
                self.prob_reproduction,
            );
            state.next_id += 1;
            state.new_sheep.push(new_sheep);
        }
    }

    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        self.animal_state == LifeState::Dead
    }
}

impl Eq for Sheep {}

impl PartialEq for Sheep {
    fn eq(&self, other: &Sheep) -> bool {
        self.id == other.id
    }
}

impl Hash for Sheep {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl fmt::Display for Sheep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
