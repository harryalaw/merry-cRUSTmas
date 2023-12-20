use day_20::part1::process;

#[tracing::instrument]
fn main() -> Result<(), ()> {
    let file = include_str!("../../input.txt");
    let result = process(file);
    println!("{}", result);

    let result = day_20::part1_no_map::process(file);
    println!("{}", result);

    Ok(())
}
