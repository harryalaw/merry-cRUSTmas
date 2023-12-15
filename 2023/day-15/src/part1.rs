#[tracing::instrument]
pub fn process(input: &str) -> usize {
    input.trim().split(',').map(hash).sum()
}

fn hash(input: &str) -> usize {
    input.bytes().fold(0, |mut total, c| {
        total += usize::from(c);
        total *= 17;
        total %= 256;
        total
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("HASH", 52)]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn test_hash(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, process(input));
    }

    #[test]
    fn test_process() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(1320, process(input));
    }
}
