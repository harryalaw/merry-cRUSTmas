use day_20::*;

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
fn part1_no_map() {
    part1_no_map::process(divan::black_box(include_str!(
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
fn part2_no_map() {
    part2_no_map::process(divan::black_box(include_str!(
        "../input.txt",
    )));
}
