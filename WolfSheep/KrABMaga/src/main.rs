use std::env;
use std::time::Instant;

use krabmaga::simulate;

use crate::model::state::WsgState;

mod model;

fn parse_args() -> (i32, i32, u32, u32, f64, f64, u16, u64, usize) {
    let args: Vec<String> = env::args().collect();
    if args.len() != 10 {
        eprintln!(
            "Usage: {} <width> <height> <n_sheep> <n_wolves> <sheep_reproduce> <wolf_reproduce> <regrowth_time> <steps> <seed>",
            args[0]
        );
        std::process::exit(1);
    }

    let width = args[1].parse::<i32>().expect("invalid width");
    let height = args[2].parse::<i32>().expect("invalid height");
    let n_sheep = args[3].parse::<u32>().expect("invalid n_sheep");
    let n_wolves = args[4].parse::<u32>().expect("invalid n_wolves");
    let sheep_reproduce = args[5].parse::<f64>().expect("invalid sheep_reproduce");
    let wolf_reproduce = args[6].parse::<f64>().expect("invalid wolf_reproduce");
    let regrowth_time = args[7].parse::<u16>().expect("invalid regrowth_time");
    let steps = args[8].parse::<u64>().expect("invalid steps");
    let _seed = args[9].parse::<usize>().expect("invalid seed");

    (
        width,
        height,
        n_sheep,
        n_wolves,
        sheep_reproduce,
        wolf_reproduce,
        regrowth_time,
        steps,
        _seed,
    )
}

fn main() {
    let (
        width,
        height,
        n_sheep,
        n_wolves,
        sheep_reproduce,
        wolf_reproduce,
        regrowth_time,
        steps,
        _seed,
    ) = parse_args();

    let state = WsgState::new(
        (width, height),
        (n_sheep, n_wolves),
        sheep_reproduce,
        wolf_reproduce,
        regrowth_time,
        5.0,
        13.0,
    );

    let start = Instant::now();
    let _ = simulate!(state, steps, 1, false);
    let elapsed_ms = start.elapsed().as_secs_f64() * 1e3;
    println!("{elapsed_ms:.6}");
}
