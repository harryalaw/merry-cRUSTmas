use std::str::FromStr;

fn main() {
    do_part1();
    println!();
    do_part2();
}

fn do_part1() {
    let test_input = std::fs::read_to_string("./test.txt").unwrap();
    println!("Test 1: {}", part1(test_input));
    let real_input = std::fs::read_to_string("./input.txt").unwrap();
    println!("Part 1: {}", part1(real_input));
}

fn do_part2() {
    let test_input = std::fs::read_to_string("./test.txt").unwrap();
    println!("Test 2: {}", part2(test_input));
    let real_input = std::fs::read_to_string("./input.txt").unwrap();
    println!("Part 2: {}", part2(real_input));
}

enum Outcome {
    WIN,
    LOSE,
    DRAW,
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Outcome, Self::Err> {
        match s {
            "X" => Ok(Outcome::LOSE),
            "Y" => Ok(Outcome::DRAW),
            "Z" => Ok(Outcome::WIN),
            _ => Err(()),
        }
    }
}

enum Hand {
    ROCK,
    PAPER,
    SCISSORS,
}

impl Outcome {
    fn score(&self) -> usize {
        match self {
            Outcome::WIN => 6,
            Outcome::DRAW => 3,
            Outcome::LOSE => 0,
        }
    }
}

impl Hand {
    fn score(&self) -> usize {
        match self {
            Hand::ROCK => 1,
            Hand::PAPER => 2,
            Hand::SCISSORS => 3,
        }
    }

    fn compare(&self, other: Hand) -> Outcome {
        match (self, other) {
            (Hand::ROCK, Hand::ROCK) => Outcome::DRAW,
            (Hand::ROCK, Hand::PAPER) => Outcome::LOSE,
            (Hand::ROCK, Hand::SCISSORS) => Outcome::WIN,

            (Hand::PAPER, Hand::ROCK) => Outcome::WIN,
            (Hand::PAPER, Hand::PAPER) => Outcome::DRAW,
            (Hand::PAPER, Hand::SCISSORS) => Outcome::LOSE,

            (Hand::SCISSORS, Hand::ROCK) => Outcome::LOSE,
            (Hand::SCISSORS, Hand::PAPER) => Outcome::WIN,
            (Hand::SCISSORS, Hand::SCISSORS) => Outcome::DRAW,
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Hand, Self::Err> {
        match s {
            "A" => Ok(Hand::ROCK),
            "B" => Ok(Hand::PAPER),
            "C" => Ok(Hand::SCISSORS),
            "X" => Ok(Hand::ROCK),
            "Y" => Ok(Hand::PAPER),
            "Z" => Ok(Hand::SCISSORS),
            _ => Err(()),
        }
    }
}

struct GameResult {
    outcome: Outcome,
    hand: Hand,
}

impl GameResult {
    fn score(&self) -> usize {
        return self.outcome.score() + self.hand.score();
    }
}

impl FromStr for GameResult {
    type Err = ();

    fn from_str(s: &str) -> Result<GameResult, Self::Err> {
        let (them, us) = s.split_once(' ').unwrap();
        let their_hand = them.parse::<Hand>().unwrap();
        let our_hand = us.parse::<Hand>().unwrap();

        let outcome = our_hand.compare(their_hand);

        return Ok(GameResult {
            outcome,
            hand: our_hand,
        });
    }
}

fn part1(input: String) -> usize {
    return input
        .lines()
        .flat_map(|line| line.parse::<GameResult>())
        .map(|game| game.score())
        .sum();
}

fn convert_to_part1(s: &str) -> &str {
    match s {
        "A X" => "A Z",
        "A Y" => "A X",
        "A Z" => "A Y",
        "B X" => "B X",
        "B Y" => "B Y",
        "B Z" => "B Z",
        "C X" => "C Y",
        "C Y" => "C Z",
        "C Z" => "C X",
        &_ => "",
    }
}

fn part2(input: String) -> usize {
    return input
        .lines()
        .map(convert_to_part1)
        .flat_map(|line| line.parse::<GameResult>())
        .map(|game| game.score())
        .sum();
}
