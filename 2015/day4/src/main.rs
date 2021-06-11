use crypto::md5::Md5;
use crypto::digest::Digest;

fn q1(input: &str) -> i32 {
    let mut md5 = Md5::new();
    let goal = String::from("00000");
    let mut guess: String;
    let mut number = 1;
    
    loop {
	guess = format!("{}{}", input, number);
	md5.input(guess.as_bytes());

	let result = md5.result_str();

	if &result[..5] == goal {
	    return number
	}
	number += 1;
	md5.reset();
    }
}

fn q2(input: &str) -> i32 {
    let mut md5 = Md5::new();
    let goal = String::from("000000");
    let mut guess: String;
    let mut number = 1;
    
    loop {
	guess = format!("{}{}", input, number);
	md5.input(guess.as_bytes());

	let result = md5.result_str();

	if &result[..6] == goal {
	    return number
	}
	number += 1;
	md5.reset();
    }
}

fn main() {
    let input = "yzbqklnj";

    let a1 = q1(input);
    let a2 = q2(input);
    println!("{}\n{}", a1, a2);
}
