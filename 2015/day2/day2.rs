fn q1(numbers: &Vec<i32>) -> i32 {
    let (l, w, h) = (numbers[0], numbers[1], numbers[2]);
    let mut vec = Vec::new();
    vec.push(l*w);
    vec.push(l*h);
    vec.push(w*h);
    
    let min = match vec.iter().min() {
        Some(num) => *num,
        None => 0
    };
    let sum: i32 = vec.iter().sum(); 
    sum*2 + min
}

fn q2(numbers: &mut Vec<i32>) -> i32 {
    let max = match numbers.iter().max() {
        Some(num) => *num,
        None => 0
    };
    let product: i32 = numbers.iter().product();

    for (index, num) in numbers.iter().enumerate() {
        if *num == max {
            numbers.remove(index);
            break;
        }
    }
    let addition_product = 2*(numbers[0] + numbers[1]);
    product + addition_product 
}

fn main() {
    let mut a1 = 0;
    let mut a2 = 0;

    let input = include_str!("input");

    for line in input.lines() {
        let mut numbers: Vec<i32> = line.split("x")
                                        .map(|s| s.parse::<i32>().unwrap())
                                        .collect();
        a1 += q1(&numbers);
        a2 += q2(&mut numbers);
    }
    println!("{}\n{}", a1, a2);
}
