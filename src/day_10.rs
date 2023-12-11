use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

type Map = Vec<Vec<Option<Pipe>>>;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Position {
    x: usize,
    y: usize,
}

fn get_initial_state(map: &Map, starting_position: &Position) -> Pipe {
    let north = map
        .get(starting_position.y - 1)
        .and_then(|row| row.get(starting_position.x))
        .unwrap_or(&None);
    let south = map
        .get(starting_position.y + 1)
        .and_then(|row| row.get(starting_position.x))
        .unwrap_or(&None);
    let east = map
        .get(starting_position.y)
        .and_then(|row| row.get(starting_position.x + 1))
        .unwrap_or(&None);
    let west = map
        .get(starting_position.y)
        .and_then(|row| row.get(starting_position.x - 1))
        .unwrap_or(&None);

    match (north, south, east, west) {
        (Some(_), Some(_), _, _) => Pipe::NorthSouth,
        (Some(_), _, Some(_), _) => Pipe::NorthEast,
        (Some(_), _, _, Some(_)) => Pipe::NorthWest,
        (_, Some(_), Some(_), _) => Pipe::SouthEast,
        (_, Some(_), _, Some(_)) => Pipe::SouthWest,
        (_, _, Some(_), Some(_)) => Pipe::EastWest,
        _ => panic!("Starting pipe is None"),
    }
}

struct Diagram {
    map: Map,
    starting_position: Position,
}

impl Diagram {
    fn new(mut map: Map, starting_position: Position) -> Self {
        let starting_pipe_type = get_initial_state(&map, &starting_position);
        let s = map
            .get_mut(starting_position.y)
            .and_then(|row| row.get_mut(starting_position.x))
            .unwrap();
        *s = Some(starting_pipe_type);

        Self {
            map,
            starting_position,
        }
    }
}

fn parse_diagram(input: &str) -> Diagram {
    let mut starting_position = None;
    let m = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '|' => Some(Pipe::NorthSouth),
                    '-' => Some(Pipe::EastWest),
                    'L' => Some(Pipe::NorthEast),
                    'J' => Some(Pipe::NorthWest),
                    '7' => Some(Pipe::SouthWest),
                    'F' => Some(Pipe::SouthEast),
                    'S' => {
                        starting_position = Some(Position { x, y });
                        None
                    }
                    _ => None,
                })
                .collect::<Vec<Option<Pipe>>>()
        })
        .collect::<Map>();

    Diagram::new(
        m,
        starting_position.expect("Could not find starting position"),
    )
}

struct PipeNavigator<'a> {
    diagram: &'a Diagram,
    heading: Direction,
    position: Position,
    current_pipe: Pipe,
    visited: HashSet<Position>,
}

impl<'a> PipeNavigator<'a> {
    fn new(diagram: &'a Diagram) -> Self {
        let starting_pipe = diagram
            .map
            .get(diagram.starting_position.y)
            .unwrap()
            .get(diagram.starting_position.x)
            .unwrap()
            .unwrap();
        let heading = match starting_pipe {
            Pipe::NorthSouth | Pipe::NorthEast | Pipe::NorthWest => Direction::North,
            Pipe::SouthWest | Pipe::SouthEast => Direction::South,
            Pipe::EastWest => Direction::East,
        };
        let mut visited = HashSet::new();
        visited.insert(diagram.starting_position);

        Self {
            diagram,
            heading,
            visited,
            position: diagram.starting_position,
            current_pipe: starting_pipe,
        }
    }

    fn step(&mut self) {
        let (next_position, next_heading) = match (self.current_pipe, self.heading) {
            (Pipe::NorthSouth, Direction::North)
            | (Pipe::NorthWest, Direction::East)
            | (Pipe::NorthEast, Direction::West) => (
                Position {
                    x: self.position.x,
                    y: self.position.y - 1,
                },
                Direction::North,
            ),
            (Pipe::NorthSouth, Direction::South)
            | (Pipe::SouthWest, Direction::East)
            | (Pipe::SouthEast, Direction::West) => (
                Position {
                    x: self.position.x,
                    y: self.position.y + 1,
                },
                Direction::South,
            ),
            (Pipe::EastWest, Direction::West)
            | (Pipe::NorthWest, Direction::South)
            | (Pipe::SouthWest, Direction::North) => (
                Position {
                    x: self.position.x - 1,
                    y: self.position.y,
                },
                Direction::West,
            ),
            (Pipe::EastWest, Direction::East)
            | (Pipe::NorthEast, Direction::South)
            | (Pipe::SouthEast, Direction::North) => (
                Position {
                    x: self.position.x + 1,
                    y: self.position.y,
                },
                Direction::East,
            ),
            _ => panic!(
                r"Unable to find next position: {:?} {:?}, {:?}",
                self.current_pipe, self.position, self.heading
            ),
        };

        self.current_pipe = match self
            .diagram
            .map
            .get(next_position.y)
            .and_then(|row| row.get(next_position.x))
        {
            Some(Some(pipe)) => *pipe,
            _ => panic!("Could not get next pipe"),
        };
        self.position = next_position;
        self.heading = next_heading;
        self.visited.insert(self.position);
    }

    fn is_at_starting_position(&self) -> bool {
        self.position == self.diagram.starting_position
    }
}

fn find_loop_points(diagram: &Diagram) -> HashSet<Position> {
    let mut pipe_navigator = PipeNavigator::new(&diagram);
    loop {
        pipe_navigator.step();
        if pipe_navigator.is_at_starting_position() {
            break;
        }
    }

    pipe_navigator.visited
}

pub struct Puzzle(String);

impl Puzzle {
    fn new(input: &str) -> Self {
        Self(input.to_string())
    }

    pub fn create(input: String) -> Box<dyn super::Puzzle> {
        Box::new(Self::new(&input))
    }
}

impl super::Puzzle for Puzzle {
    fn run_part_one(&self) {
        let diagram = parse_diagram(&self.0);
        let loop_points = find_loop_points(&diagram);

        let length = loop_points.len();

        println!("Part 1: {}", length / 2);
    }

    fn run_part_two(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    #[test]
    fn test_parse_sample_input() {
        let map = parse_diagram(&SAMPLE_INPUT);
        assert_eq!(map.map.len(), 5);
        assert_eq!(map.map[0].len(), 5);
    }
}
