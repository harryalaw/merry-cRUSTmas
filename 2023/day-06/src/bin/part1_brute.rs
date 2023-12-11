use day_06::part1_brute::process;

#[tracing::instrument]
fn main() -> Result<(), ()> {
    let file = include_str!("../../input.txt");
    let result = process(file);
    println!("{}", result);
    Ok(())
}
