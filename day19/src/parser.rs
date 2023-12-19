use crate::structs::{Condition, Label, Part, Rule, Workflow};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline, one_of},
    multi::{many1, separated_list1},
    sequence::{delimited, tuple},
    IResult,
};

fn parse_pass_rule(input: &str) -> IResult<&str, Rule> {
    let (input, destination) = alpha1(input)?;
    let rule = Rule::new(Condition::Pass, Label::new(destination.to_string()));
    Ok((input, rule))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    let (input, (xmas, gt_or_le, value, _, destination)) =
        tuple((one_of("xmas"), one_of("><"), digit1, tag(":"), alpha1))(input)?;
    let value = value.parse().unwrap();
    let condition = match gt_or_le {
        '>' => Condition::Greater(xmas, value),
        '<' => Condition::Lesser(xmas, value),
        _ => panic!("wrong gt_or_le: {:?}", gt_or_le),
    };
    let destination = Label::new(destination.to_string());
    let rule = Rule::new(condition, destination);
    Ok((input, rule))
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let parse_rules = separated_list1(tag(","), parse_rule);
    let (input, (name, _, rules, _, rule, _, _)) = tuple((
        alpha1,
        tag("{"),
        parse_rules,
        tag(","),
        parse_pass_rule,
        tag("}"),
        newline,
    ))(input)?;
    let mut rules = rules;
    rules.push(rule);
    let name = Label::new(name.to_string());
    let workflow = Workflow::new(name, rules);
    Ok((input, workflow))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
    let x = tuple((tag("x="), digit1));
    let m = tuple((tag("m="), digit1));
    let a = tuple((tag("a="), digit1));
    let s = tuple((tag("s="), digit1));
    let xmas = tuple((x, tag(","), m, tag(","), a, tag(","), s));
    let (input, ((_, x), _, (_, m), _, (_, a), _, (_, s))) =
        delimited(tag("{"), xmas, tag("}\n"))(input)?;
    let x = x.parse().unwrap();
    let m = m.parse().unwrap();
    let a = a.parse().unwrap();
    let s = s.parse().unwrap();
    let part = Part::new(x, m, a, s);
    Ok((input, part))
}

pub fn parse(input: &str) -> (Vec<Workflow>, Vec<Part>) {
    let (_, (workflows, _, parts)) =
        tuple((many1(parse_workflow), newline, many1(parse_part)))(input).unwrap();
    (workflows, parts)
}
