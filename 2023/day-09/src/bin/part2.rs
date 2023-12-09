use day_09::part2::process;

#[tracing::instrument]
fn main() -> Result<(), ()> {
    let file = include_str!("../../input.txt");
    let result = process(file);
    println!("{}", result);

    let result = day_09::part2_pascal::process(file);
    println!("{}", result);
    Ok(())
}
