#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let race = parse_races(input);

    run_race(race)
}

fn run_race(race: (usize, usize)) -> usize {
    (0..race.0).fold(0, |total, current_time| {
        let speed = current_time;
        let remaining_time = race.0 - current_time;
        return if speed * remaining_time > race.1 { total + 1 } else {total }
    })
}

fn parse_races(input: &str) -> (usize, usize) {
    let lines = input.split_once("\n").expect("Unix endings");

    let time: usize = lines
        .0
        .split_once(": ")
        .expect("Starts with a colon")
        .1
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .expect("It's a number");

    let distance: usize = lines
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
        assert_eq!(71503, process(input));
    }
}
