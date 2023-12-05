use day_05::part2_dumb::process;

#[tracing::instrument]
fn main() -> Result<(), ()> {
    use std::time::Instant;

    let file = include_str!("../../input.txt");

    let now = Instant::now();
    let result = process(file);
    println!("{}", result);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Ok(())
}

