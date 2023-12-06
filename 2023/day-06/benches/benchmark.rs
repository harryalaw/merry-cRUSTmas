use day_06::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!(
        "../input.txt",
    )));
}

#[divan::bench]
fn part1_quadratic() {
    part1_quadratic::process(divan::black_box(include_str!(
        "../input.txt",
    )));
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!(
        "../input.txt",
    )));
}

#[divan::bench]
fn part2_quadratic() {
    part2_quadratic::process(divan::black_box(include_str!(
        "../input.txt",
    )));
}
