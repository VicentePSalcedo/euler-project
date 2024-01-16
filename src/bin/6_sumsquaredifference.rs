use crate::utils::write_down_answer;
#[path = "../utils.rs"]
mod utils;
fn main() {
    let question_number = 6;
    println!(
"The sum of the squares of the first ten natural numbers is,
    1^2 + 2^2 + 10^2 = 385.
The square of the sum of the first ten natural numbers is,
    (1 + 2 + ... + 10)^2 = 55^2 = 3025
Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640.
Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum."
);
    let answer = sum_square_difference();
    println!("{}", answer);
    if let Err(error) = write_down_answer(question_number, &answer.to_string()) {
        eprintln!("Error writing down answer: {}", error);
    }
}
fn sum_square_difference() -> i32 {
    let mut sum = 0;
    let mut square = 0;
    for n in 1..101 {
        sum = sum + i32::pow(n, 2);
        square = square + n;
    }
    square = i32::pow(square, 2);
    square - sum
}
