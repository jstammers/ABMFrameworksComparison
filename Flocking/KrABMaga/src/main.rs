use std::env;
use std::time::Instant;

use krabmaga::simulate;

use crate::model::state::Flocker;

mod model;

pub static COHESION: f32 = 0.03;
pub static AVOIDANCE: f32 = 0.015;
pub static RANDOMNESS: f32 = 0.0;
pub static CONSISTENCY: f32 = 0.05;
pub static MOMENTUM: f32 = 1.0;
pub static JUMP: f32 = 1.0;
pub static DISCRETIZATION: f32 = 5.0 / 1.5;
pub static TOROIDAL: bool = true;

fn parse_args() -> (u32, f32, f32, u64, usize) {
    let args: Vec<String> = env::args().collect();
    if args.len() != 6 {
        eprintln!(
            "Usage: {} <population> <width> <height> <steps> <seed>",
            args[0]
        );
        std::process::exit(1);
    }

    let population = args[1].parse::<u32>().expect("invalid population");
    let width = args[2].parse::<f32>().expect("invalid width");
    let height = args[3].parse::<f32>().expect("invalid height");
    let steps = args[4].parse::<u64>().expect("invalid steps");
    let seed = args[5].parse::<usize>().expect("invalid seed");

    (population, width, height, steps, seed)
}

fn main() {
    let (population, width, height, steps, _seed) = parse_args();
    let state = Flocker::new((width, height), population);

    let start = Instant::now();
    let _ = simulate!(state, steps, 1, false);
    let elapsed_ms = start.elapsed().as_secs_f64() * 1e3;
    println!("{elapsed_ms:.6}");
}
