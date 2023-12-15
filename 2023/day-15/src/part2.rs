#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let instructions: Vec<&str> = input.trim().split(',').collect();
    let mut hashbox = HashBox::new(instructions);

    hashbox.execute();

    hashbox.value()
}

struct HashBox<'a> {
    instructions: Vec<&'a str>,
    boxes: Vec<Vec<Lens<'a>>>,
}

impl HashBox<'_> {
    fn new(instructions: Vec<&str>) -> HashBox {
        let mut boxes = Vec::with_capacity(256);
        for _i in 0..256 {
            boxes.push(Vec::new());
        }

        HashBox {
            instructions,
            boxes,
        }
    }

    fn execute(&mut self) {
        for instruction in self.instructions.iter() {
            match instruction.ends_with('-') {
                true => {
                    let label = &instruction[..instruction.len() - 1];
                    let box_idx = hash(label);
                    let prev_box = &mut self.boxes[box_idx];
                    let mut i = 0;
                    let mut found = false;
                    while i < prev_box.len() {
                        if prev_box[i].label == label {
                            found = true;
                            break;
                        }
                        i += 1
                    }
                    if found {
                        prev_box.remove(i);
                    }
                }
                false => {
                    let label = &instruction[..instruction.len() - 2];
                    let focal = instruction[instruction.len() - 1..]
                        .parse::<usize>()
                        .expect("It's a number");

                    let box_idx = hash(label);
                    let prev_box = &mut self.boxes[box_idx];

                    let mut i = 0;
                    let mut found = false;
                    while i < prev_box.len() {
                        if prev_box[i].label == label {
                            found = true;
                            break;
                        }
                        i += 1
                    }
                    if found {
                        prev_box[i].focal = focal;
                    } else {
                        let lens = Lens { label, focal };
                        prev_box.push(lens);
                    }
                }
            }
        }
    }

    fn value(&self) -> usize {
        let mut total = 0;
        for i in 0..256 {
            for (j, lens) in self.boxes[i].iter().enumerate() {
                total += (i + 1) * (j + 1) * lens.focal;
            }
        }
        total
    }
}

struct Lens<'a> {
    label: &'a str,
    focal: usize,
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
    #[case("rn", 0)]
    #[case("qp", 1)]
    #[case("cm", 0)]
    #[case("pc", 3)]
    #[case("ot", 3)]
    #[case("ab", 3)]
    fn test_hash(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(expected, hash(input));
    }

    #[test]
    fn test_process() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(145, process(input));
    }
}
