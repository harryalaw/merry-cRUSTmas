use day_14::*;

#[tracing::instrument]
fn main() -> Result<(), ()> {
    let file = include_str!("../../input.txt");
    let result = part2::process(file);
    println!("{}", result);

    let result = part2_mut::process(file);
    println!("{}", result);

    let result = part2_hash::process(file);
    println!("{}", result);
    Ok(())
}
