pub struct Puzzle(String);

impl Puzzle {
    fn new(input: &str) -> Self {
        Self(input.to_string())
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
    fn sample_part_one() {
        todo!();
    }
}

