use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> usize {
    let workflows = parse_input(input);

    let mut parts = Vec::new();
    parts.push((
        "in",
        Part {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
    ));

    let mut total = 0;
    while !parts.is_empty() {
        let mut new_parts = Vec::new();
        for (workflow_id, part) in parts.iter() {
            let mut part = part.clone();
            if workflow_id == &"R" {
                continue;
            };
            if workflow_id == &"A" {
                total += (part.x.1 - part.x.0 + 1)
                    * (part.m.1 - part.m.0 + 1)
                    * (part.a.1 - part.a.0 + 1)
                    * (part.s.1 - part.s.0 + 1);
                continue;
            }
            let workflow = workflows.get(workflow_id).expect("It's there");
            for condition in &workflow.conditions {
                let (matched_condition, failed_condition) = condition.apply(&part);
                new_parts.push((condition.send_to, matched_condition));
                part = failed_condition;
            }
            new_parts.push((workflow.send_to, part.to_owned()));
        }
        parts = new_parts;
    }
    total
}

fn parse_input(input: &str) -> HashMap<&str, Workflow> {
    let halves = input.split_once("\n\n").expect("Unix endings");

    let workflows = parse_workflows(halves.0);

    workflows
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

#[derive(Clone, Debug)]
struct Part {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl Part {
    fn with(&self, field: Field, min: usize, max: usize) -> Part {
        let mut new_part = self.clone();
        match field {
            Field::X => new_part.x = (min, max),
            Field::M => new_part.m = (min, max),
            Field::A => new_part.a = (min, max),
            Field::S => new_part.s = (min, max),
        }

        new_part
    }
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
    // returns (matched_condition, failed_condition);
    fn apply(&self, part: &Part) -> (Part, Part) {
        match (&self.field, &self.comparison) {
            (Field::X, Comparison::GT) => (
                part.with(Field::X, self.value + 1, part.x.1),
                part.with(Field::X, part.x.0, self.value),
            ),
            (Field::X, Comparison::LT) => (
                part.with(Field::X, part.x.0, self.value - 1),
                part.with(Field::X, self.value, part.x.1),
            ),
            (Field::M, Comparison::GT) => (
                part.with(Field::M, self.value + 1, part.m.1),
                part.with(Field::M, part.m.0, self.value),
            ),
            (Field::M, Comparison::LT) => (
                part.with(Field::M, part.m.0, self.value - 1),
                part.with(Field::M, self.value, part.m.1),
            ),
            (Field::A, Comparison::GT) => (
                part.with(Field::A, self.value + 1, part.a.1),
                part.with(Field::A, part.a.0, self.value),
            ),
            (Field::A, Comparison::LT) => (
                part.with(Field::A, part.a.0, self.value - 1),
                part.with(Field::A, self.value, part.a.1),
            ),
            (Field::S, Comparison::GT) => (
                part.with(Field::S, self.value + 1, part.s.1),
                part.with(Field::S, part.s.0, self.value),
            ),
            (Field::S, Comparison::LT) => (
                part.with(Field::S, part.s.0, self.value - 1),
                part.with(Field::S, self.value, part.s.1),
            ),
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
        assert_eq!(167409079868000, process(input));
    }
}
