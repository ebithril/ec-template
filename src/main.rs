use std::fs;

fn part1(input_path: &str) -> String {
    let content = fs::read_to_string(input_path).expect("Expected input.txt");
    let lines = content.lines();

    let result = "".to_string();
    result
}

fn part2(input_path: &str) -> String {
    let content = fs::read_to_string(input_path).expect("Expected input.txt");
    let lines = content.lines();

    let result = "".to_string();
    result
}

fn part3(input_path: &str) -> String {
    let content = fs::read_to_string(input_path).expect("Expected input.txt");
    let lines = content.lines();

    let result = "".to_string();
    result
}

fn main() {
    println!("Part1: {}", part1("input1.txt"));
    println!("Part2: {}", part2("input2.txt"));
    println!("Part3: {}", part3("input3.txt"));
}

#[cfg(test)]
mod test {
    use crate::{part1, part2, part3};

    #[test]
    fn part1_test() {
        assert_eq!("".to_string(), part1("example1.txt"));
    }

    #[test]
    fn part2_test() {
        assert_eq!("".to_string(), part2("example2.txt"));
    }

    #[test]
    fn part3_test() {
        assert_eq!("".to_string(), part3("example3.txt"));
    }
}
