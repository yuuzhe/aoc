use std::collections::HashSet;

fn q1(input: &str) -> usize {
    let mut coordinate = (0, 0);
    let mut visited = HashSet::new();

    visited.insert(coordinate);
    for c in input.chars() {
        match c {
            '>' => coordinate.0 += 1,
            '<' => coordinate.0 -= 1,
            'v' => coordinate.1 -= 1,
            '^' => coordinate.1 += 1,
            _   => panic!("Unkonw char in the input file."),
        }
        visited.insert(coordinate);
    }

    visited.len()
}

fn q2(input: &str) -> usize {
    let mut santa_coordinate = (0, 0);
    let mut robot_santa_coordinate = (0, 0);
    let mut visited = HashSet::new();
    visited.insert(santa_coordinate);

    let mut two_char_iterator = input.chars().peekable();
    while two_char_iterator.peek().is_some() {
        let s: String = two_char_iterator.by_ref()
                                         .take(2)
                                         .collect();
        match s.chars().nth(0).unwrap() {
            '>' => santa_coordinate.0 += 1,
            '<' => santa_coordinate.0 -= 1,
            'v' => santa_coordinate.1 -= 1,
            '^' => santa_coordinate.1 += 1,
            c   => panic!("Unknown char '{}' in the input file.", c),
        }
        match s.chars().nth(1).unwrap() {
            '>' => robot_santa_coordinate.0 += 1,
            '<' => robot_santa_coordinate.0 -= 1,
            'v' => robot_santa_coordinate.1 -= 1,
            '^' => robot_santa_coordinate.1 += 1,
            c   => panic!("Unknown char '{}' in the input file.", c),
        }
        visited.insert(santa_coordinate);
        visited.insert(robot_santa_coordinate);
    }

    visited.len()
}

fn main() {
    let input = include_str!("input");
    
    let a1 = q1(input);
    let a2 = q2(input);
    println!("{}\n{}", a1, a2);
}
