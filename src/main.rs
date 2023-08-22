use std::process::exit;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        exit(1);
    }

    let stopwatch = std::time::Instant::now();
    let result = kaykyratsuba::karatsuba(&args[1], &args[2]);
    let elapsed = stopwatch.elapsed();

    println!("{}\n{:?}", result, elapsed);
}
