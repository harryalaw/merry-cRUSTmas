use hashbrown::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let first = parts.next()?.parse::<u32>().ok();
            let second = parts.next()?.parse::<u32>().ok();
            if let (Some(f), Some(s)) = (first, second) {
                Some((f, s))
            } else {
                None
            }
        })
        .fold(HashMap::new(), |mut acc, (num1, num2)| {
            let _ = *acc
                .entry(num1)
                .and_modify(|e: &mut (u32, u32)| e.0 += 1)
                .or_insert((1, 0));
            let _ = *acc
                .entry(num2)
                .and_modify(|e: &mut (u32, u32)| e.1 += 1)
                .or_insert((0, 1));
            acc
        })
        .into_iter()
        .fold(0, |acc, (key, value)| acc + key * (value.0 * value.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(31, process(input));
    }
}
