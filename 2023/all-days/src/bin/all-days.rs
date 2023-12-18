use std::time::Duration;

#[tracing::instrument]
fn main() -> Result<(), ()> {
    use std::time::Instant;

    let mut total_time_in_nanos = 0;

    // day 1
    let file = include_str!("../../../day-01/input.txt");
    let now = Instant::now();
    let _result = day_01::part1::process(file);
    let duration = now.elapsed();
    println!("Day 01 Part1: {duration:?}");
    total_time_in_nanos += duration.as_nanos();

    let now = Instant::now();
    let _result = day_01::part2::process(file);
    let duration = now.elapsed();
    println!("Day 01 Part2: {duration:?}");
    total_time_in_nanos += duration.as_nanos();
    println!();

    // day 2
    let file = include_str!("../../../day-02/input.txt");
    let now = Instant::now();
    let _result = day_02::part1::process(file);
    let duration = now.elapsed();
    println!("Day 02 Part1: {duration:?}");
    total_time_in_nanos += duration.as_nanos();

    let now = Instant::now();
    let _result = day_02::part2::process(file);
    let duration = now.elapsed();
    println!("Day 02 Part2: {duration:?}");
    total_time_in_nanos += duration.as_nanos();
    println!();

    // day 3
    let file = include_str!("../../../day-03/input.txt");
    let now = Instant::now();
    let _result = day_03::part1::process(file);
    let duration = now.elapsed();
    println!("Day 03 Part1: {duration:?}");
    total_time_in_nanos += duration.as_nanos();

    let now = Instant::now();
    let _result = day_03::part2::process(file);
    let duration = now.elapsed();
    println!("Day 03 Part2: {duration:?}");
    total_time_in_nanos += duration.as_nanos();
    println!();

    // day 4
    let file = include_str!("../../../day-04/input.txt");
    let now = Instant::now();
    let _result = day_04::part1::process(file);
    let duration = now.elapsed();
    println!("Day 04 Part1: {duration:?}");
    total_time_in_nanos += duration.as_nanos();

    let now = Instant::now();
    let _result = day_04::part2::process(file);
    let duration = now.elapsed();
    println!("Day 04 Part2: {duration:?}");
    total_time_in_nanos += duration.as_nanos();
    println!();

    // day 5
    let file = include_str!("../../../day-05/input.txt");
    let now = Instant::now();
    let _result = day_05::part1::process(file);
    let duration = now.elapsed();
    println!("Day 05 Part1: {duration:?}");
    total_time_in_nanos += duration.as_nanos();

    let now = Instant::now();
    let _result = day_05::part2::process(file);
    let duration = now.elapsed();
    println!("Day 05 Part2: {duration:?}");
    total_time_in_nanos += duration.as_nanos();
    println!();

    // day 6
    let file = include_str!("../../../day-06/input.txt");
    let now = Instant::now();
    let _result = day_06::part1::process(file);
    let duration = now.elapsed();
    println!("Day 06 Part1: {duration:?}");
    total_time_in_nanos += duration.as_nanos();

    let now = Instant::now();
    let _result = day_06::part2::process(file);
    let duration = now.elapsed();
    println!("Day 06 Part2: {duration:?}");
    total_time_in_nanos += duration.as_nanos();
    println!();

    // day 7
    let file = include_str!("../../../day-07/input.txt");
    let now = Instant::now();
    let _result = day_07::part1::process(file);
    let duration = now.elapsed();
    println!("Day 07 Part1: {duration:?}");
    total_time_in_nanos += duration.as_nanos();

    let now = Instant::now();
    let _result = day_07::part2::process(file);
    let duration = now.elapsed();
    println!("Day 07 Part2: {duration:?}");
    total_time_in_nanos += duration.as_nanos();
    println!();

    // day 8
    let file = include_str!("../../../day-08/input.txt");
    let now = Instant::now();
    let _result = day_08::part1_hash::process(file);
    let duration = now.elapsed();
    println!("Day 08 Part1: {duration:?}");
    total_time_in_nanos += duration.as_nanos();

    let now = Instant::now();
    let _result = day_08::part2_hash::process(file);
    let duration = now.elapsed();
    println!("Day 08 Part2: {duration:?}");
    total_time_in_nanos += duration.as_nanos();
    println!();

    // day 9
    let file = include_str!("../../../day-09/input.txt");
    let now = Instant::now();
    let _result = day_09::part1_pascal::process(file);
    let duration = now.elapsed();
    println!("Day 09 Part1: {duration:?}");
    total_time_in_nanos += duration.as_nanos();

    let now = Instant::now();
    let _result = day_09::part2_pascal::process(file);
    let duration = now.elapsed();
    println!("Day 09 Part2: {duration:?}");
    total_time_in_nanos += duration.as_nanos();
    println!();

    // day 10
    let file = include_str!("../../../day-10/input.txt");
    let now = Instant::now();
    let _result = day_10::part1_no_map::process(file);
    let duration = now.elapsed();
    println!("Day 10 Part1: {duration:?}");
    total_time_in_nanos += duration.as_nanos();

    let now = Instant::now();
    let _result = day_10::part2_no_map::process(file);
    let duration = now.elapsed();
    println!("Day 10 Part2: {duration:?}");
    total_time_in_nanos += duration.as_nanos();
    println!();

    // day 11
    let file = include_str!("../../../day-11/input.txt");
    let now = Instant::now();
    let _result = day_11::part1::process(file);
    let duration = now.elapsed();
    println!("Day 11 Part1: {duration:?}");
    total_time_in_nanos += duration.as_nanos();

    let now = Instant::now();
    let _result = day_11::part2::process(file);
    let duration = now.elapsed();
    println!("Day 11 Part2: {duration:?}");
    total_time_in_nanos += duration.as_nanos();
    println!();

    // day 12
    let file = include_str!("../../../day-12/input.txt");
    let now = Instant::now();
    let _result = day_12::part1::process(file);
    let duration = now.elapsed();
    println!("Day 12 Part1: {duration:?}");
    total_time_in_nanos += duration.as_nanos();

    let now = Instant::now();
    let _result = day_12::part2::process(file);
    let duration = now.elapsed();
    println!("Day 12 Part2: {duration:?}");
    total_time_in_nanos += duration.as_nanos();
    println!();

    // day 13
    let file = include_str!("../../../day-13/input.txt");
    let now = Instant::now();
    let _result = day_13::part1::process(file);
    let duration = now.elapsed();
    println!("Day 13 Part1: {duration:?}");
    total_time_in_nanos += duration.as_nanos();

    let now = Instant::now();
    let _result = day_13::part2::process(file);
    let duration = now.elapsed();
    println!("Day 13 Part2: {duration:?}");
    total_time_in_nanos += duration.as_nanos();
    println!();

    // day 14
    let file = include_str!("../../../day-14/input.txt");
    let now = Instant::now();
    let _result = day_14::part1::process(file);
    let duration = now.elapsed();
    println!("Day 14 Part1: {duration:?}");
    total_time_in_nanos += duration.as_nanos();

    let now = Instant::now();
    let _result = day_14::part2_mut::process(file);
    let duration = now.elapsed();
    println!("Day 14 Part2: {duration:?}");
    total_time_in_nanos += duration.as_nanos();
    println!();

    // day 15
    let file = include_str!("../../../day-15/input.txt");
    let now = Instant::now();
    let _result = day_15::part1::process(file);
    let duration = now.elapsed();
    println!("Day 15 Part1: {duration:?}");
    total_time_in_nanos += duration.as_nanos();

    let now = Instant::now();
    let _result = day_15::part2::process(file);
    let duration = now.elapsed();
    println!("Day 15 Part2: {duration:?}");
    total_time_in_nanos += duration.as_nanos();
    println!();

    // day 16
    let file = include_str!("../../../day-16/input.txt");
    let now = Instant::now();
    let _result = day_16::part1::process(file);
    let duration = now.elapsed();
    println!("Day 16 Part1: {duration:?}");
    total_time_in_nanos += duration.as_nanos();

    let now = Instant::now();
    let _result = day_16::part2::process(file);
    let duration = now.elapsed();
    println!("Day 16 Part2: {duration:?}");
    total_time_in_nanos += duration.as_nanos();
    println!();

    // day 17
    let file = include_str!("../../../day-17/input.txt");
    let now = Instant::now();
    let _result = day_17::part1::process(file);
    let duration = now.elapsed();
    println!("Day 17 Part1: {duration:?}");
    total_time_in_nanos += duration.as_nanos();

    let now = Instant::now();
    let _result = day_17::part2::process(file);
    let duration = now.elapsed();
    println!("Day 17 Part2: {duration:?}");
    total_time_in_nanos += duration.as_nanos();
    println!();

    // day 18
    let file = include_str!("../../../day-18/input.txt");
    let now = Instant::now();
    let _result = day_18::part1::process(file);
    let duration = now.elapsed();
    println!("Day 18 Part1: {duration:?}");
    total_time_in_nanos += duration.as_nanos();

    let now = Instant::now();
    let _result = day_18::part2::process(file);
    let duration = now.elapsed();
    println!("Day 18 Part2: {duration:?}");
    total_time_in_nanos += duration.as_nanos();
    println!();

    let total_duration = Duration::from_nanos(total_time_in_nanos as u64);

    println!("Total time: {total_duration:?}");
    Ok(())
}
