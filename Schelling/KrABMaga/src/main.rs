use std::env;
use std::time::Instant;

use krabmaga::simulate;

use crate::model::world::World;

mod model;

fn parse_args() -> (i32, i32, u32, u32, i32, u64, usize) {
    let args: Vec<String> = env::args().collect();
    if args.len() != 8 {
        eprintln!(
            "Usage: {} <width> <height> <num_agents> <min_to_be_happy> <radius> <steps> <seed>",
            args[0]
        );
        std::process::exit(1);
    }

    let width = args[1].parse::<i32>().expect("invalid width");
    let height = args[2].parse::<i32>().expect("invalid height");
    let num_agents = args[3].parse::<u32>().expect("invalid num_agents");
    let min_to_be_happy = args[4].parse::<u32>().expect("invalid min_to_be_happy");
    let radius = args[5].parse::<i32>().expect("invalid radius");
    let steps = args[6].parse::<u64>().expect("invalid steps");
    let seed = args[7].parse::<usize>().expect("invalid seed");

    (
        width,
        height,
        num_agents,
        min_to_be_happy,
        radius,
        steps,
        seed,
    )
}

fn main() {
    let (width, height, num_agents, min_to_be_happy, radius, steps, _seed) = parse_args();
    let world = World::new((width, height), num_agents, min_to_be_happy, radius);

    let start = Instant::now();
    simulate!(world, steps, 1, false);
    let elapsed_ms = start.elapsed().as_secs_f64() * 1e3;
    println!("{elapsed_ms:.6}");
}
