fn q1(input: &str) -> i32 {
    input.chars()
         .fold(0, |acc, c| match c {
                    '(' => acc + 1,
                    ')' => acc - 1,
                    _   => panic!("Illegal char"),
                })
}

fn q2(input: &str) -> usize {
    let mut sum = 0;
    for (i, c) in input.chars().enumerate() {
        if sum == -1 {
            return i;
        } else {
            match c {
                '(' => sum += 1,
                ')' => sum -= 1,
                _   => panic!("Illegal char"),
            }
        }
    }

    0
}

fn main() {
    let input = include_str!("input");
    let a1 = q1(input);
    let a2 = q2(input);

    println!("{}", a1);
    println!("{}", a2);
}
