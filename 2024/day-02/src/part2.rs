#[tracing::instrument]
pub fn process(_input: &str) -> usize {
    _input
        .lines()
        .map(|line| {
            let values: Vec<usize> = line
                .split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok())
                .collect();

            if is_safe(&values) || is_safe_subset(&values) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn is_safe(list: &[usize]) -> bool {
    let mut ascending = true;
    let mut descending = true;

    for i in 0..list.len() - 1 {
        if list[i] < list[i + 1] {
            descending = false;
        } else if list[i] > list[i + 1] {
            ascending = false;
        }

        if list[i].abs_diff(list[i + 1]) > 3 || list[i] == list[i + 1] {
            return false;
        }
    }

    return descending ^ ascending;
}

fn is_safe_subset(values: &[usize]) -> bool {
    (0..values.len()).any(|i| {
        let mut ascending = true;
        let mut descending = true;
        let mut last_valid = None;

        for (j, &value) in values.iter().enumerate() {
            if j == i {
                continue;
            }

            if let Some(last) = last_valid {
                if value < last {
                    descending = false;
                } else if value > last {
                    ascending = false;
                }

                if value.abs_diff(last) > 3 || value == last {
                    return false;
                }
            }

            last_valid = Some(value);
        }

        descending ^ ascending
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        assert_eq!(4, process(input));
    }
}
