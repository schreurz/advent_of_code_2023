pub mod setup {
    pub fn read_file(path: &String) -> Result<String, Box<dyn std::error::Error>> {
        let res = std::fs::read_to_string(&path)?;
        Ok(res)
    }
}

pub mod day_one {
    fn parse_document_line(line: &str) -> Option<usize> {
        let mut res = None;
        line.chars().into_iter().for_each(|char| {
            if let Ok(v) = char.to_string().parse::<usize>() {
                res = match res {
                    None => Some((v,v)),
                    Some((a, _)) => Some((a, v))
                };
            }
        });
        match res {
            Some((a, b)) => Some(a * 10 + b),
            _ => None
        }
    }

    fn parse_trebuchet_calibration(document: &String) -> usize {
        let calibration = document.split("\n").into_iter().filter_map(|line| {
            parse_document_line(line)
        }).collect::<Vec<usize>>();

        calibration.iter().fold(0, |acc, &x| 
            acc + x
        )
    }

    pub fn run(input: &String) -> () {
        let res = parse_trebuchet_calibration(input);
        println!("{}", res);
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_parse_line() {
            assert_eq!(parse_document_line("12"), Some(12));
            assert_eq!(parse_document_line("a1b2c"), Some(12));
            assert_eq!(parse_document_line("a1b"), Some(11));
            assert_eq!(parse_document_line("ab"), None);
        }
    }
}