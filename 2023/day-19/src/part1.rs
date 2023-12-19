use hashbrown::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let (workflows, parts) = parse_input(input);

    parts
        .iter()
        .fold(0, |acc, part| {
            let mut workflow_id = "in";
            while workflow_id != "R" && workflow_id != "A" {
                let workflow = workflows.get(&workflow_id).expect("It's there");
                let mut condition_matched = false;
                for condition in &workflow.conditions {
                    if let Some(send_to) = condition.apply(part) {
                        workflow_id = send_to;
                        condition_matched = true;
                        break;
                    }
                }
                if !condition_matched {
                    workflow_id = workflow.send_to;
                }
            }

            if workflow_id == "A" {
                acc + part.x + part.m + part.a + part.s
            } else {
                acc
            }
        })
}

fn parse_input(input: &str) -> (HashMap<&str, Workflow>, Vec<Part>) {
    let halves = input.split_once("\n\n").expect("Unix endings");

    let workflows = parse_workflows(halves.0);
    let parts = parse_parts(halves.1);

    (workflows, parts)
}

fn parse_workflows(input: &str) -> HashMap<&str, Workflow> {
    let mut workflow_map = HashMap::new();

    input.lines().for_each(|line| {
        let mut id = "";
        let mut conditions = Vec::new();
        let mut send_to = "";
        line.split(['{', '}', ','])
            .enumerate()
            .for_each(|(i, chunk)| {
                if chunk == "" {
                    return;
                }
                if i == 0 {
                    id = chunk;
                }
                if chunk.contains(':') {
                    conditions.push(parse_condition(chunk));
                } else {
                    send_to = chunk;
                }
            });
        workflow_map.insert(
            id,
            Workflow {
                conditions,
                send_to,
            },
        );
    });

    workflow_map
}

fn parse_condition(chunk: &str) -> Condition {
    let parts = chunk.split_once(':').expect("has a colon");
    let field = match &parts.0[0..1] {
        "x" => Field::X,
        "m" => Field::M,
        "a" => Field::A,
        "s" => Field::S,
        _ => panic!("Invalid field char: {}", &parts.0[0..1]),
    };
    let comparison = match &parts.0[1..2] {
        ">" => Comparison::GT,
        "<" => Comparison::LT,
        _ => panic!("Unknown comparison: {}", &parts.0[1..2]),
    };

    let value = usize::from_str_radix(&parts.0[2..], 10).expect("It's a number");

    let send_to = parts.1;

    Condition {
        field,
        comparison,
        value,
        send_to,
    }
}

fn parse_parts(input: &str) -> Vec<Part> {
    input
        .lines()
        .map(|line| {
            let mut sections = line[1..line.len() - 1].split(",");
            let x =
                usize::from_str_radix(&sections.next().unwrap()[2..], 10).expect("It's a number");
            let m =
                usize::from_str_radix(&sections.next().unwrap()[2..], 10).expect("It's a number");
            let a =
                usize::from_str_radix(&sections.next().unwrap()[2..], 10).expect("It's a number");
            let s =
                usize::from_str_radix(&sections.next().unwrap()[2..], 10).expect("It's a number");

            Part {
                x,
                m,
                a,
                s,
            }
        })
        .collect()
}

#[derive(Clone, Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(Clone, Debug)]
struct Workflow<'a> {
    conditions: Vec<Condition<'a>>,
    send_to: &'a str,
}

#[derive(Clone, Debug)]
enum Field {
    X,
    M,
    A,
    S,
}

#[derive(Clone, Debug)]
enum Comparison {
    GT,
    LT,
}

#[derive(Clone, Debug)]
struct Condition<'a> {
    field: Field,
    comparison: Comparison,
    value: usize,
    send_to: &'a str,
}

impl Condition<'_> {
    fn apply(&self, part: &Part) -> Option<&str> {
        match (&self.field, &self.comparison) {
            (Field::X, Comparison::GT) => {
                if part.x > self.value {
                    Some(self.send_to)
                } else {
                    None
                }
            }
            (Field::X, Comparison::LT) => {
                if part.x < self.value {
                    Some(self.send_to)
                } else {
                    None
                }
            }
            (Field::M, Comparison::GT) => {
                if part.m > self.value {
                    Some(self.send_to)
                } else {
                    None
                }
            }
            (Field::M, Comparison::LT) => {
                if part.m < self.value {
                    Some(self.send_to)
                } else {
                    None
                }
            }
            (Field::A, Comparison::GT) => {
                if part.a > self.value {
                    Some(self.send_to)
                } else {
                    None
                }
            }
            (Field::A, Comparison::LT) => {
                if part.a < self.value {
                    Some(self.send_to)
                } else {
                    None
                }
            }
            (Field::S, Comparison::GT) => {
                if part.s > self.value {
                    Some(self.send_to)
                } else {
                    None
                }
            }
            (Field::S, Comparison::LT) => {
                if part.s < self.value {
                    Some(self.send_to)
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(19114, process(input));
    }
}
