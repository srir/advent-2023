use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

lazy_static! {
    static ref RE_PART: Regex = Regex::new(r"x=(\d+),m=(\d+),a=(\d+),s=(\d+)").unwrap();
    static ref RE_RULE: Regex = Regex::new(r"([A-Za-z])([<>=])(\d+):(\w+)").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn rating_number(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = RE_PART.captures(s).ok_or(())?;
        let x = caps.get(1).unwrap().as_str().parse().map_err(|_| ())?;
        let m = caps.get(2).unwrap().as_str().parse().map_err(|_| ())?;
        let a = caps.get(3).unwrap().as_str().parse().map_err(|_| ())?;
        let s = caps.get(4).unwrap().as_str().parse().map_err(|_| ())?;

        Ok(Self { x, m, a, s })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Var {
    X,
    M,
    A,
    S,
}

impl FromStr for Var {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::X),
            "m" => Ok(Self::M),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Dest {
    Reject,
    Accept,
    Workflow(String),
}

impl FromStr for Dest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Reject),
            "A" => Ok(Self::Accept),
            s => Ok(Self::Workflow(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule {
    var: Var,
    op: Ordering,
    val: usize,
    dest: Dest,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = RE_RULE.captures(s).ok_or(())?;
        let var = caps.get(1).unwrap().as_str().parse()?;
        let op = match caps.get(2).unwrap().as_str() {
            "<" => Ordering::Less,
            "=" => Ordering::Equal,
            ">" => Ordering::Greater,
            _ => return Err(()),
        };
        let val = caps.get(3).unwrap().as_str().parse().map_err(|_| ())?;
        let dest = caps.get(4).unwrap().as_str().parse()?;
        Ok(Self { var, op, val, dest })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    fallback: Dest,
}

impl Workflow {
    fn run(&self, part: &Part) -> &Dest {
        for rule in &self.rules {
            let val = match rule.var {
                Var::X => part.x,
                Var::M => part.m,
                Var::A => part.a,
                Var::S => part.s,
            };
            if val.cmp(&rule.val) == rule.op {
                return &rule.dest;
            }
        }
        &self.fallback
    }
}

impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rest) = s.split_once("{").ok_or(())?;
        let rules_s = rest.strip_suffix("}").ok_or(())?;
        let rules_strings = rules_s.split(",").collect::<Vec<_>>();
        let (rules_strings, fallback) = (
            &rules_strings[..rules_strings.len() - 1],
            rules_strings[rules_strings.len() - 1],
        );

        let rules = rules_strings
            .iter()
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            name: name.to_string(),
            rules,
            fallback: fallback.parse()?,
        })
    }
}

struct System {
    parts: Vec<Part>,
    workflows: HashMap<String, Workflow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RunResult {
    accepted: Vec<Part>,
    rejected: Vec<Part>,
}

impl RunResult {
    fn sum_rating_numbers(&self) -> usize {
        self.accepted.iter().map(|p| p.rating_number()).sum()
    }
}

impl System {
    fn run_workflows(&self, part: &Part) -> &Dest {
        let mut dest = None;
        let mut workflow = self.workflows.get("in");

        while let Some(w) = workflow {
            dest = Some(w.run(part));
            workflow = match dest {
                Some(Dest::Workflow(ref name)) => self.workflows.get(name),
                _ => None,
            };
        }

        dest.unwrap()
    }

    fn run_all_workflows(&self) -> RunResult {
        let mut accepted = vec![];
        let mut rejected = vec![];

        for part in &self.parts {
            let dest = self.run_workflows(part);
            match dest {
                Dest::Accept => accepted.push(*part),
                Dest::Reject => rejected.push(*part),
                _ => (),
            }
        }

        RunResult { accepted, rejected }
    }
}

impl FromStr for System {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (workflows_s, parts_s) = s.split_once("\n\n").ok_or(())?;

        let parts = parts_s
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<_>, _>>()?;

        let workflows = workflows_s
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<Workflow>, _>>()?;

        let workflows = workflows
            .into_iter()
            .map(|w| (w.name.clone(), w))
            .collect::<HashMap<String, Workflow>>();

        Ok(Self { parts, workflows })
    }
}

#[aoc_generator(day19)]
fn parse_input(input: &str) -> System {
    input.parse().unwrap()
}

#[aoc(day19, part1)]
fn part1(system: &System) -> usize {
    system.run_all_workflows().sum_rating_numbers()
}
