#[tracing::instrument]
pub fn process(input: &str) -> f64 {
    let race = parse_races(input);

    run_race(race)
}

fn run_race(race: (f64, f64)) -> f64 {
    let total_time = race.0;
    let distance = race.1;

    let discriminant_squared = total_time * total_time - 4_f64 * distance;
    let discriminant = discriminant_squared.sqrt();

    let speed_min = (total_time - discriminant) / 2_f64;
    let speed_max = (total_time + discriminant) / 2_f64;

    let speed_min_int = next_int(speed_min);
    let speed_max_int = prev_int(speed_max);

    speed_max_int - speed_min_int + 1_f64
}

fn next_int(num: f64) -> f64 {
    if num.ceil() == num {
        return num + 1.0;
    }
    num.ceil()
}

fn prev_int(num: f64) -> f64 {
    if num.floor() == num {
        return num - 1.0;
    }
    num.floor()
}

fn parse_races(input: &str) -> (f64, f64) {
    let lines = input.split_once("\n").expect("Unix endings");

    let time: f64 = lines
        .0
        .split_once(": ")
        .expect("Starts with a colon")
        .1
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .expect("It's a number");

    let distance: f64 = lines
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
