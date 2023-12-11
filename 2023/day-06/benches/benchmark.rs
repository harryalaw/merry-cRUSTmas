use day_06::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!("../input.txt",)));
}

#[divan::bench]
fn part1_brute() {
    part1_brute::process(divan::black_box(include_str!("../input.txt",)));
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!("../input.txt",)));
}

#[divan::bench]
fn part2_brute() {
    part2_brute::process(divan::black_box(include_str!("../input.txt",)));
}
