use day_08::part1::process;

#[tracing::instrument]
fn main() -> Result<(), ()> {
    let file = include_str!("../../input.txt");
    let result = process(file);
    println!("{}", result);

    let file = include_str!("../../input.txt");
    let result = day_08::part1_hash::process(file);
    println!("{}", result);
    Ok(())
}
