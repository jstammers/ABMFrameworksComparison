use crate::model::state::{LifeState, WsgState};
use core::fmt;
use krabmaga::engine::agent::Agent;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
pub struct Wolf {
    pub id: u32,
    pub animal_state: LifeState,
    pub loc: Int2D,
    pub energy: f64,
    pub gain_energy: f64,
    pub prob_reproduction: f64,
}

impl Wolf {
    pub fn new(id: u32, loc: Int2D, energy: f64, gain_energy: f64, prob_reproduction: f64) -> Wolf {
        Wolf {
            id,
            loc,
            energy,
            gain_energy,
            prob_reproduction,
            animal_state: LifeState::Alive,
        }
    }
}

impl Agent for Wolf {
    fn step(&mut self, state: &mut dyn State) {
        let state = state.as_any_mut().downcast_mut::<WsgState>().unwrap();
        if self.animal_state == LifeState::Dead {
            return;
        }

        let x = self.loc.x;
        let y = self.loc.y;
        let mut rng = rand::rng();

        let mut moves = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x + dx;
                let ny = y + dy;
                if nx >= 0 && nx < state.dim.0 && ny >= 0 && ny < state.dim.1 {
                    moves.push((dx, dy));
                }
            }
        }
        let (nx, ny) = moves[rng.random_range(0..moves.len())];
        self.loc = Int2D {
            x: x + nx,
            y: y + ny,
        };

        state.wolves_grid.set_object_location(*self, &self.loc);

        if let Some(sheep) = state.sheep_grid.get_objects(&self.loc) {
            for mut candidate in sheep {
                if state.killed_sheep.get(&candidate).is_none()
                    && candidate.animal_state == LifeState::Alive
                {
                    candidate.animal_state = LifeState::Dead;
                    state
                        .sheep_grid
                        .remove_object_location(candidate, &candidate.loc);
                    self.energy += self.gain_energy;
                    state.killed_sheep.insert(candidate);
                    break;
                }
            }
        }

        self.energy -= 1.0;
        if self.energy < 0.0 {
            self.animal_state = LifeState::Dead;
        } else if rng.random_bool(self.prob_reproduction) {
            self.energy /= 2.0;
            let new_wolf = Wolf::new(
                state.next_id,
                self.loc,
                self.energy,
                self.gain_energy,
                self.prob_reproduction,
            );
            state.next_id += 1;
            state.new_wolves.push(new_wolf);
        }
    }

    fn is_stopped(&mut self, _state: &mut dyn State) -> bool {
        self.animal_state == LifeState::Dead
    }
}

impl Eq for Wolf {}

impl PartialEq for Wolf {
    fn eq(&self, other: &Wolf) -> bool {
        self.id == other.id
    }
}

impl Hash for Wolf {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}

impl fmt::Display for Wolf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}
