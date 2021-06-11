fn q1(input: &str) -> usize {
    let mut grids = [[false; 1000]; 1000];
    let mut vec: Vec<&str>;
    let mut left_corner: Vec<usize>;
    let mut right_corner: Vec<usize>;
    let mut operation;

    for line in input.lines() {
        vec = line.split(' ').collect();
        right_corner = vec.pop()
                          .unwrap()
                          .split(',')
                          .map(|s| s.parse::<usize>().unwrap())
                          .collect();
        vec.pop();
        left_corner = vec.pop()
                         .unwrap()
                         .split(',')
                         .map(|s| s.parse::<usize>().unwrap())
                         .collect();
        operation = match vec.pop().unwrap() {
            "on" => |_| true,
            "off" => |_| false,
            "toggle" => |x: bool| !x,
            _ => panic!("Unkown operation")
        };

        for i in left_corner[0]..=right_corner[0] {
            for j in left_corner[1]..=right_corner[1] {
                grids[i][j] = operation(grids[i][j]);
            }
        }
    }

    grids.iter()
         .flatten()
         .fold(0, |acc, &x| if x { acc + 1 } else { acc })
}

fn q2(input: &str) -> usize {
    let mut grids = [[0; 1000]; 1000];
    let mut vec: Vec<&str>;
    let mut left_corner: Vec<usize>;
    let mut right_corner: Vec<usize>;
    let mut operation;

    for line in input.lines() {
        vec = line.split(' ').collect();
        right_corner = vec.pop()
                          .unwrap()
                          .split(',')
                          .map(|s| s.parse::<usize>().unwrap())
                          .collect();
        vec.pop();
        left_corner = vec.pop()
                         .unwrap()
                         .split(',')
                         .map(|s| s.parse::<usize>().unwrap())
                         .collect();
        operation = match vec.pop().unwrap() {
            "on" => |x: usize| x + 1,
            "off" => |x: usize| if x != 0 { x - 1 } else { 0 },
            "toggle" => |x: usize| x + 2,
            _ => panic!("Unkown operation")
        };

        for i in left_corner[0]..=right_corner[0] {
            for j in left_corner[1]..=right_corner[1] {
                grids[i][j] = operation(grids[i][j]);
            }
        }
    }

    grids.iter()
         .flatten()
         .fold(0, |acc, &x| acc + x)
}

fn main() {
    let input = include_str!("input");
    let a1 = q1(input);
    let a2 = q2(input);
    println!("{}\n{}", a1, a2);
}
