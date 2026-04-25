use hashbrown::HashSet;
use krabmaga::engine::fields::dense_number_grid_2d::DenseNumberGrid2D;
use krabmaga::engine::fields::dense_object_grid_2d::DenseGrid2D;
use krabmaga::engine::fields::field::Field;
use krabmaga::engine::fields::grid_option::GridOption;
use krabmaga::engine::location::Int2D;
use krabmaga::engine::schedule::Schedule;
use krabmaga::engine::state::State;
use krabmaga::rand;
use krabmaga::rand::Rng;
use std::any::Any;

use super::sheep::Sheep;
use super::wolf::Wolf;

#[derive(Clone, Copy, PartialEq)]
pub enum LifeState {
    Alive,
    Dead,
}

pub struct WsgState {
    pub dim: (i32, i32),
    pub wolves_grid: DenseGrid2D<Wolf>,
    pub sheep_grid: DenseGrid2D<Sheep>,
    pub grass_field: DenseNumberGrid2D<u16>,
    pub step: u64,
    pub next_id: u32,
    pub new_sheep: Vec<Sheep>,
    pub new_wolves: Vec<Wolf>,
    pub killed_sheep: HashSet<Sheep>,
    pub initial_animals: (u32, u32),
    pub sheep_gain_from_food: f64,
    pub wolf_gain_from_food: f64,
    pub sheep_reproduce: f64,
    pub wolf_reproduce: f64,
    pub grass_regrowth_time: u16,
}

impl WsgState {
    pub fn new(
        dim: (i32, i32),
        initial_animals: (u32, u32),
        sheep_reproduce: f64,
        wolf_reproduce: f64,
        grass_regrowth_time: u16,
        sheep_gain_from_food: f64,
        wolf_gain_from_food: f64,
    ) -> WsgState {
        WsgState {
            dim,
            wolves_grid: DenseGrid2D::new(dim.0, dim.1),
            sheep_grid: DenseGrid2D::new(dim.0, dim.1),
            grass_field: DenseNumberGrid2D::new(dim.0, dim.1),
            step: 0,
            next_id: initial_animals.1 + initial_animals.0,
            new_sheep: Vec::new(),
            new_wolves: Vec::new(),
            initial_animals,
            killed_sheep: HashSet::new(),
            sheep_gain_from_food,
            wolf_gain_from_food,
            sheep_reproduce,
            wolf_reproduce,
            grass_regrowth_time,
        }
    }
}

impl State for WsgState {
    fn reset(&mut self) {
        self.step = 0;
        self.wolves_grid = DenseGrid2D::new(self.dim.0, self.dim.1);
        self.sheep_grid = DenseGrid2D::new(self.dim.0, self.dim.1);
        self.grass_field = DenseNumberGrid2D::new(self.dim.0, self.dim.1);
        self.next_id = self.initial_animals.0 + self.initial_animals.1;
        self.new_sheep = Vec::new();
        self.new_wolves = Vec::new();
    }

    fn init(&mut self, schedule: &mut Schedule) {
        self.reset();
        generate_grass(self);
        generate_wolves(self, schedule);
        generate_sheep(self, schedule);
    }

    fn update(&mut self, step: u64) {
        if step != 0 {
            self.grass_field.apply_to_all_values(
                |grass| {
                    let growth = *grass;
                    if growth < self.grass_regrowth_time {
                        growth + 1
                    } else {
                        growth
                    }
                },
                GridOption::READWRITE,
            );
        }

        self.grass_field.lazy_update();
        self.sheep_grid.lazy_update();
        self.wolves_grid.lazy_update();
        self.step = step;
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }

    fn before_step(&mut self, _schedule: &mut Schedule) {
        self.new_sheep.clear();
        self.new_wolves.clear();
    }

    fn after_step(&mut self, schedule: &mut Schedule) {
        for sheep in self.new_sheep.iter() {
            schedule.schedule_repeating(Box::new(*sheep), schedule.time + 1.0, 0);
        }

        for wolf in self.new_wolves.iter() {
            schedule.schedule_repeating(Box::new(*wolf), schedule.time + 1.0, 1);
        }

        for sheep in self.killed_sheep.iter() {
            schedule.dequeue(Box::new(*sheep), sheep.id);
        }

        self.killed_sheep.clear();
    }
}

fn generate_grass(state: &mut WsgState) {
    for x in 0..state.dim.1 {
        for y in 0..state.dim.0 {
            let mut rng = rand::rng();
            let fully_growth = rng.random_bool(0.5);
            if fully_growth {
                state
                    .grass_field
                    .set_value_location(state.grass_regrowth_time, &Int2D { x, y });
            } else {
                let grass_init_value = rng.random_range(1..state.grass_regrowth_time + 1);
                state
                    .grass_field
                    .set_value_location(grass_init_value, &Int2D { x, y });
            }
        }
    }
}

fn generate_sheep(state: &mut WsgState, schedule: &mut Schedule) {
    let mut rng = rand::rng();

    for id in 0..state.initial_animals.0 {
        let loc = Int2D {
            x: rng.random_range(0..state.dim.0),
            y: rng.random_range(0..state.dim.1),
        };
        let init_energy =
            rng.random_range(1..(2.0 * state.sheep_gain_from_food) as usize + 1) as f64;
        let sheep = Sheep::new(
            id + state.initial_animals.1,
            loc,
            init_energy,
            state.sheep_gain_from_food,
            state.sheep_reproduce,
        );
        state.sheep_grid.set_object_location(sheep, &loc);
        schedule.schedule_repeating(Box::new(sheep), 0.0, 0);
    }
}

fn generate_wolves(state: &mut WsgState, schedule: &mut Schedule) {
    let mut rng = rand::rng();
    for id in 0..state.initial_animals.1 {
        let loc = Int2D {
            x: rng.random_range(0..state.dim.0),
            y: rng.random_range(0..state.dim.1),
        };
        let init_energy =
            rng.random_range(1..(2.0 * state.wolf_gain_from_food) as usize + 1) as f64;
        let wolf = Wolf::new(
            id,
            loc,
            init_energy,
            state.wolf_gain_from_food,
            state.wolf_reproduce,
        );
        state.wolves_grid.set_object_location(wolf, &loc);
        schedule.schedule_repeating(Box::new(wolf), 0.0, 1);
    }
}
