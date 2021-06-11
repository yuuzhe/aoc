use std::collections::HashMap;

fn q(input: &str) -> u16 {
    let mut table = HashMap::new();
    let mut iter;
    let mut src: Vec<&str>;
    let mut dst: &str;

    while table.get("a") == None {
        for line in input.lines() {
            iter = line.split(" -> ").into_iter();
            src = iter.next().unwrap().split(' ').collect();

            dst = iter.next().unwrap();
            if table.get(dst) != None {
                continue;
            }

            let src_len = src.len();
            let src1_v: u16;
            let src2_v: u16;
        
            match src_len {
                3 => {
                    match src[0].parse::<u16>() {
                        Ok(v) => src1_v = v,
                        Err(_) => {
                            match table.get(src[0]) {
                                Some(v) => src1_v = *v,
                                None => continue
                            };
                        }
                    };
                    match src[2].parse::<u16>() {
                        Ok(v) => src2_v = v,
                        Err(_) => {
                            match table.get(src[2]) {
                                Some(v) => src2_v = *v,
                                None => continue
                            };
                        }
                    };
                    match src[1] {
                        "AND" => table.insert(dst, src1_v & src2_v),
                        "OR" => table.insert(dst, src1_v | src2_v),
                        "LSHIFT" => table.insert(dst, src1_v << src2_v),
                        "RSHIFT" => table.insert(dst, src1_v >> src2_v),
                        op => panic!("Unknown bitwise operation {}", op)
                    };
                }
                2 => {
                    match src[1].parse::<u16>() {
                        Ok(v) => src1_v = !v,
                        Err(_) => {
                            match table.get(src[1]) {
                                Some(v) => src1_v = !*v,
                                None => continue
                            };
                        }
                    };
                    table.insert(dst, src1_v);
                }
                1 => {
                    match src[0].parse::<u16>() {
                        Ok(v) => src1_v = v,
                        Err(_) => match table.get(src[0]) {
                            Some(v) => src1_v = *v,
                            None    => continue
                        }
                    };
                    table.insert(dst, src1_v);
                }
                _ => panic!("Split src token failed\n")
            }
        }
    }

    *table.get("a").unwrap()
}

fn main() {
    let input = include_str!("input");
    let input2 = include_str!("input2");
    let a1 = q(input);
    let a2 = q(input2);
    println!("{}\n{}", a1, a2);
}
