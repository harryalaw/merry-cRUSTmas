use std::str::FromStr;

pub fn process(input: &str) -> usize {
    return input
        .lines()
        .flat_map(|line| line.parse::<Game>())
        .filter(|game| game.is_valid())
        .map(|game| game.id)
        .sum();
}

struct Game {
    id: usize,
    most_blue: usize,
    most_green: usize,
    most_red: usize,
}

impl Game {
    fn is_valid(&self) -> bool {
        self.most_blue <= 14 && self.most_green <= 13 && self.most_red <= 12
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once(": ").expect("It has a colon");
        let game_id = parts
            .0
            .split_once(' ')
            .expect("It has a space")
            .1
            .parse::<usize>()
            .expect("It's a number");
        let words: &str = parts.1;
        Ok(words.split("; ").flat_map(|word| word.split(", ")).fold(
            Game {
                id: game_id,
                most_blue: 0,
                most_green: 0,
                most_red: 0,
            },
            |acc, word| {
                let (number, color) = word.split_once(' ').expect("Should have a space");
                let count = number.parse::<usize>().expect("Should be a number");
                let mut game = acc;
                match color {
                    "blue" => {
                        game.most_blue = game.most_blue.max(count);
                    }
                    "green" => {
                        game.most_green = game.most_green.max(count);
                    }
                    "red" => {
                        game.most_red = game.most_red.max(count);
                    }
                    _ => todo!(),
                };
                game
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(8, process(input))
    }
}
