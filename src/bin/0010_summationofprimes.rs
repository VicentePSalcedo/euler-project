use crate::utils::write_down_answer;
#[path = "../utils.rs"]
mod utils;
fn main() {
    let question_number = 10;
    println!("The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
    Find the sum of all the primes below two million.");
    let answer = sum_of_primes_under_n(2000000);
    println!("{}", answer);
    if let Err(error) = write_down_answer(question_number, &answer.to_string()) {
        eprintln!("Error writing down answer: {}", error);
    }
}
fn sum_of_primes_under_n (n: usize) -> usize {
    let mut marked = vec![0; n];
    let mut sum: usize = 2;
    for prime in (3..n).step_by(2) {
        if marked[prime] == 0 {
            sum += prime;
            for multiple in ((prime * prime)..n).step_by(prime) {
                marked[multiple] = 1;
            }
        }
    }
    return sum ;
}

