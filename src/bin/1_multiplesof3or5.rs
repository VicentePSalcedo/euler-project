use crate::utils::write_down_answer;
#[path = "../utils.rs"]
mod utils;
fn main() {
    let question_number = 1;
    println!(
        "If we list all the natural numbers below 10 that are multiples of
3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23. Find the sum of
all the multiples of 3 or 5 below 1000."
    );
    let mut answer = 0;
    for n in 1..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            answer += n;
        }
    }
    println!("{}", answer);
    if let Err(error) = write_down_answer(question_number, &answer.to_string()) {
        eprintln!("Error writing down answer: {}", error);
    }
}
