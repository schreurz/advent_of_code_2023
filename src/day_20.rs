#[derive(Debug, PartialEq, Eq, Clone)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, PartialEq, Eq)]
enum Module {
    FlipFlop(bool),
    Conjunction(Vec<(usize, Pulse)>),
}

impl Module {
    fn receive_pulse(&mut self, pulse: Pulse, sender: usize) -> Option<Pulse> {
        match (self, pulse) {
            (Module::FlipFlop(_), Pulse::High) => None,
            (Module::FlipFlop(on), Pulse::Low) => {
                if *on {
                    *on = false;
                    Some(Pulse::Low)
                } else {
                    *on = true;
                    Some(Pulse::High)
                }
            }
            (Module::Conjunction(inputs), pulse) => {
                inputs
                    .iter_mut()
                    .find(|(i, _)| *i == sender)
                    .map(|(_, p)| *p = pulse);
                if inputs.iter().all(|(_, p)| *p == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Puzzle {
    modules: Vec<(Module, Vec<usize>)>,
    starting_modules: Vec<usize>,
}

#[derive(Debug, PartialEq, Eq)]
enum ParsePuzzleLineResult<'a> {
    Module(char, &'a str, Vec<&'a str>),
    Broadcaster(Vec<&'a str>),
}

fn parse_line(line: &str) -> ParsePuzzleLineResult {
    let mut split = line.split(" -> ");
    let descriptor = split.next().expect("No module descriptor");
    let destinations = split
        .next()
        .expect("No destination modules")
        .split(", ")
        .collect::<Vec<_>>();

    match descriptor.split_at(1) {
        ("b", _) => ParsePuzzleLineResult::Broadcaster(destinations),
        (c, id) => ParsePuzzleLineResult::Module(
            c.chars().next().expect("Unable to get module type"),
            id,
            destinations,
        ),
    }
}

impl Puzzle {
    fn new(input: &str) -> Self {
        let modules = input
            .lines()
            .map(|line| parse_line(&line))
            .collect::<Vec<ParsePuzzleLineResult>>();

        let id_to_index_map = modules
            .iter()
            .filter_map(|module| match module {
                ParsePuzzleLineResult::Broadcaster(_) => None,
                ParsePuzzleLineResult::Module(_, id, _) => Some(*id),
            })
            .enumerate()
            .collect::<Vec<_>>();

        let broadcaster = modules
            .iter()
            .find_map(|m| match m {
                ParsePuzzleLineResult::Broadcaster(dests) => {
                    let destinations = dests
                        .iter()
                        .filter_map(
                            |d_id| match id_to_index_map.iter().find(|(_, d)| d == d_id) {
                                Some((index, _)) => Some(*index),
                                None => None,
                            },
                        )
                        .collect::<Vec<_>>();

                    Some(destinations)
                }
                _ => None,
            })
            .expect("No broadcaster module");

        let mut modules = modules
            .iter()
            .filter_map(|m| match m {
                ParsePuzzleLineResult::Module(c, _, destinations) => Some((c, destinations)),
                _ => None,
            })
            .map(|(t, destinations)| {
                let destinations = destinations
                    .iter()
                    .filter_map(
                        |d_id| match id_to_index_map.iter().find(|(_, d)| d == d_id) {
                            Some((index, _)) => Some(*index),
                            None => None,
                        },
                    )
                    .collect::<Vec<_>>();

                match t {
                    &'%' => (Module::FlipFlop(false), destinations),
                    &'&' => (Module::Conjunction(vec![]), destinations),
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<_>>();

        modules
            .iter()
            .enumerate()
            .map(|(i, (_, dests))| (i, dests.clone()))
            .collect::<Vec<_>>()
            .iter()
            .for_each(|(i, dests)| {
                dests.iter().for_each(|&d| match &mut modules[d] {
                    (Module::Conjunction(inputs), _) => inputs.push((*i, Pulse::Low)),
                    _ => (),
                });
            });

        Self {
            modules,
            starting_modules: broadcaster,
        }
    }

    pub fn create(input: String) -> Box<dyn super::Puzzle> {
        Box::new(Self::new(&input))
    }
}

impl crate::Puzzle for Puzzle {
    fn run_part_one(&self) -> Result<crate::AOCResult, Box<dyn std::error::Error>> {
        unimplemented!();
    }

    fn run_part_two(&self) -> Result<crate::AOCResult, Box<dyn std::error::Error>> {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_puzzle() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

        assert_eq!(
            Puzzle::new(&input),
            Puzzle {
                starting_modules: vec![0, 1, 2],
                modules: vec![
                    (Module::FlipFlop(false), vec![1]),
                    (Module::FlipFlop(false), vec![2]),
                    (Module::FlipFlop(false), vec![3]),
                    (Module::Conjunction(vec![(2, Pulse::Low)]), vec![0]),
                ]
            }
        );

        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

        assert_eq!(
            Puzzle::new(&input),
            Puzzle {
                starting_modules: vec![0],
                modules: vec![
                    (Module::FlipFlop(false), vec![1, 3]),
                    (Module::Conjunction(vec![(0, Pulse::Low)]), vec![2]),
                    (Module::FlipFlop(false), vec![3]),
                    (
                        Module::Conjunction(vec![(0, Pulse::Low), (2, Pulse::Low)]),
                        vec![]
                    ),
                ]
            }
        );
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("%a -> b, c, d"),
            ParsePuzzleLineResult::Module('%', "a", vec!["b", "c", "d"])
        );
        assert_eq!(
            parse_line("broadcaster -> b, c, d"),
            ParsePuzzleLineResult::Broadcaster(vec!["b", "c", "d"])
        );
        assert_eq!(
            parse_line("&ab -> bc, de, fg"),
            ParsePuzzleLineResult::Module('&', "ab", vec!["bc", "de", "fg"])
        );
    }

    #[test]
    fn test_receive_pulse() {
        let mut module = Module::FlipFlop(false);
        module.receive_pulse(Pulse::High, 0);
    }
}
