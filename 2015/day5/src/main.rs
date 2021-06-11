use fancy_regex::Regex;

fn q1(input: &str) -> usize {
    let mut continuous_char = false;
    let mut prev_char = '0';
    let mut vowels_count = 0;
    let mut count = 0;

    for line in input.lines() {
        for c in line.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => {
                    vowels_count += 1;
                }
                'b' => {
                    if prev_char == 'a' {
                    continuous_char = false;
                    break;
                    }
                }
                'd' => {
                    if prev_char == 'c' {
                    continuous_char = false;
                    break;
                    }
                }
                'q' => {
                    if prev_char == 'p' {
                    continuous_char = false;
                    break;
                    }
                }
                'y' => {
                    if prev_char == 'x' {
                    continuous_char = false;
                    break;
                    }
                }
                _ => ()
            }

            if prev_char == c {
                continuous_char = true;
            }
            prev_char = c;
        }

        if vowels_count >= 3 && continuous_char {
            count += 1;
        }
        continuous_char = false;
        vowels_count = 0;
        prev_char = '0';
    }

    count
}

fn q2(input: &str) -> usize {
    let re1 = Regex::new(r"(..).*\1").unwrap();
    let re2 = Regex::new(r"(.).\1").unwrap();

    input.lines()
         .filter(|line| re1.is_match(line).unwrap() && 
                        re2.is_match(line).unwrap())
         .collect::<Vec<&str>>()
         .len()
}

fn main() {
    let input = include_str!("input");
    let a1 = q1(input);
    let a2 = q2(input);
    println!("{}\n{}", a1, a2);
}
