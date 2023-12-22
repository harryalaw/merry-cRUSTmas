use day_21::part2::process;

#[tracing::instrument]
fn main() -> Result<(), ()> {
    let file = include_str!("../../input.txt");
    let result = process(file);
    println!("{}", result);

    let result = day_21::part2_geometry::process(file);
    println!("{}", result);
    Ok(())
}
