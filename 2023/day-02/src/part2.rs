use std::str::FromStr;

pub fn process(input: &str) -> usize {
    return input
        .lines()
        .flat_map(|line| line.parse::<Game>())
        .map(|game| game.power())
        .sum();
}

struct Game {
    most_blue: usize,
    most_green: usize,
    most_red: usize,
}

impl Game {
    fn power(&self) -> usize {
        self.most_blue * self.most_green * self.most_red
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once(": ").expect("It has a colon");
        let words: &str = parts.1;
        Ok(words.split("; ").flat_map(|word| word.split(", ")).fold(
            Game {
                most_blue: 0,
                most_green: 0,
                most_red: 0,
            },
            |acc, word| {
                let (number, color) = word.split_once(" ").expect("Should have a space");
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
                return game;
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(2286, process(input))
    }
}

