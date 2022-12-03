fn main() {
    do_part1();
    println!();
    do_part2();
}

fn do_part1() {
    println!("Test 1: {}", part1(include_str!("../../test.txt")));
    println!("Part 1: {}", part1(include_str!("../../input.txt")));
}

fn do_part2() {
    println!("Test 2: {}", part2(include_str!("../../test.txt")));
    println!("Part 2: {}", part2(include_str!("../../input.txt")));
}

fn part1(input: &str) -> i32 {
    let output = input
        .split("\n\n")
        .map(|line| {
            line.lines()
                .map(|num| num.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max();

    return output.unwrap();
}

fn part2(input: &str) -> i32 {
    let mut output: Vec<i32> = input
        .split("\n\n")
        .map(|line| {
            line.lines()
                .map(|num| num.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();

    output.sort_by(|a, b| b.cmp(a));
    return output.iter().take(3).sum::<i32>();
}
