#[tracing::instrument]
pub fn process(input: &str) -> f32 {
    let race = parse_races(input);

    run_race(race)
}

fn run_race(race: (f32, f32)) -> f32 {
    let total_time = race.0;
    let distance = race.1;

    let discriminant_squared = total_time * total_time - 4_f32 * distance;
    let discriminant = discriminant_squared.sqrt();

    let speed_min = (total_time - discriminant) / 2_f32;
    let speed_max = (total_time + discriminant) / 2_f32;

    let speed_min_int = speed_min.ceil();
    let speed_max_int = bad_floor(speed_max);

    speed_max_int - speed_min_int 
}

// handle edge case where it all works out
// so that the top value is 1 more than the largest value so that
// high - low = number of values that work
fn bad_floor(num: f32) -> f32 {
    if num.floor() == num {
        return num - 1.0;
    }
    num.floor() + 1.0
}

fn parse_races(input: &str) -> (f32, f32) {
    let lines = input.split_once("\n").expect("Unix endings");

    let time: f32 = lines
        .0
        .split_once(": ")
        .expect("Starts with a colon")
        .1
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .expect("It's a number");

    let distance: f32 = lines
        .1
        .split_once(": ")
        .expect("Starts with a colon")
        .1
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .expect("It's a number");

    (time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        assert_eq!(71503.0, process(input));
    }
}
