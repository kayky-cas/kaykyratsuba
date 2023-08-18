use std::process::exit;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 3 {
        exit(1);
    }

    let stopwatch = std::time::Instant::now();
    println!(
        "{}\n{:?}",
        kaykyratsuba::karatsuba(&args[1], &args[2]),
        stopwatch.elapsed()
    );
}
