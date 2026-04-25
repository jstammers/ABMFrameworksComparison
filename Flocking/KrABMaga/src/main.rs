use std::env;
use std::time::Instant;

use krabmaga::simulate;

use crate::model::state::Flocker;

mod model;

pub static COHESION: f32 = 0.03;
pub static SEPARATE: f32 = 0.015;
pub static MATCH: f32 = 0.05;
pub static JUMP: f32 = 1.0;
pub static SEPARATION: f32 = 1.0;
pub static DISCRETIZATION: f32 = 4.0;
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
    let (population, width, height, steps, seed) = parse_args();
    let vision = if population <= 200 { 5.0 } else { 15.0 };
    let state = Flocker::new((width, height), population, vision, seed as u64);

    let start = Instant::now();
    let _ = simulate!(state, steps, 1, false);
    let elapsed_ms = start.elapsed().as_secs_f64() * 1e3;
    println!("{elapsed_ms:.6}");
}
