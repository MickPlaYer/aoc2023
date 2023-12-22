pub fn process_part1(input: String) -> Option<usize> {
    None
}

pub fn process_part2(input: String) -> Option<usize> {
    None
}

#[cfg(test)]
mod tests {
    const DAY_NUMBER: usize = ();
    use super::*;
    use shared::{read_input, read_sample};

    #[test]
    fn process_part1_with_sample_1s() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(0), answer);
    }

    #[test]
    fn process_part1_with_input_1i() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part1(content);
        assert_eq!(Some(0), answer);
    }

    #[test]
    fn process_part2_with_sample_2s() {
        let content = read_sample(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(0), answer);
    }

    #[test]
    fn process_part2_with_input_2i() {
        let content = read_input(DAY_NUMBER);
        let answer = process_part2(content);
        assert_eq!(Some(0), answer);
    }
}
