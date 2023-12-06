#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let races = parse_races(input);
    
    races.iter()
        .map(|race| run_race(*race))
        .fold(1, |acc, curr| acc * curr)
}

fn run_race(race: (usize, usize)) -> usize {
    (0..race.0).fold(0, |total, current_time| {
        let speed = current_time;
        let remaining_time = race.0 - current_time;
        return if speed * remaining_time > race.1 { total + 1 } else {total }
    })
}

fn parse_races(input: &str) -> Vec<(usize, usize)> {
    let lines = input.split_once("\n").expect("Unix endings");

    let times: Vec<usize> = lines
        .0
        .split_once(": ")
        .expect("Starts with a colon")
        .1
        .split_ascii_whitespace()
        .flat_map(|x| x.trim().parse::<usize>())
        .collect();
    let distances: Vec<usize> = lines
        .1
        .split_once(": ")
        .expect("Starts with a colon")
        .1
        .split_ascii_whitespace()
        .flat_map(|x| x.trim().parse::<usize>())
        .collect();

    times
        .iter()
        .zip(distances.iter())
        .map(|(x, y)| (*x, *y))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        assert_eq!(288, process(input));
    }
}
