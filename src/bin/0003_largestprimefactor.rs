use crate::utils::write_down_answer;
#[path = "../utils.rs"]
mod utils;
fn main() {
    let question_number = 3;
    println!(
        "The prime factors of 13195 are 5, 7, 13 and 29.
What is the largest prime factor of the number 600851475143?"
    );
    let num = 600851475143;
    let answer = max_prime_factors(num);
    println!("{}", answer);
    if let Err(error) = write_down_answer(question_number, &answer.to_string()) {
        eprintln!("Error writing down answer: {}", error);
    }
}
fn max_prime_factors(mut num: i64) -> i64 {
    let mut largest_prime = -1;
    let mut i = 2;
    while i * i <= num {
        while num % i == 0 {
            largest_prime = i;
            num = num / i;
        }
        i = i + 1;
    }
    if num > 1 {
        largest_prime = num;
    }
    return largest_prime;
}
