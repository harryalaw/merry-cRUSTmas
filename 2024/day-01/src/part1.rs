#[tracing::instrument]
pub fn process(input: &str) -> u32 {
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    for line in input.lines() {
        if let Some((num1, num2)) = parse_line(line) {
            insert_sorted(&mut list1, num1);
            insert_sorted(&mut list2, num2);
        }
    }

    list1
        .into_iter()
        .zip(list2.into_iter())
        .fold(0, |acc, (a, b)| acc + a.abs_diff(b))
}

fn parse_line(line: &str) -> Option<(u32, u32)> {
    let mut parts = line.split_whitespace();
    let first = parts.next()?.parse::<u32>().ok();
    let second = parts.next()?.parse::<u32>().ok();
    if let (Some(f), Some(s)) = (first, second) {
        Some((f, s))
    } else {
        None
    }
}

fn insert_sorted(list: &mut Vec<u32>, value: u32) {
    let pos = list.binary_search(&value).unwrap_or_else(|e| e);
    list.insert(pos, value);
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
        assert_eq!(11, process(input));
    }
}
