use crate::utils::write_down_answer;
#[path = "../utils.rs"]
mod utils;
fn main() {
    let question_number = 4;
    println!("A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009=91x99.
Find the largest palindrome made from the product of two 3-digit numbers.");
    let answer = largest_palindrome();
    println!("{}", answer);
    if let Err(error) = write_down_answer(question_number, &answer.to_string()) {
        eprintln!("Error writing down answer: {}", error);
    }
}
fn largest_palindrome() -> i32 {
    let mut largest = 0;
    for i in 100..999 {
        for j in 100..999 {
            let num = i * j;
            if check_palindrome(num) && num > largest {
                largest = num;
            }
        }
    }
    return largest;
}
fn check_palindrome(mut num: i32) -> bool {
    let mut temp = 0;
    let backup = num;
    while num > 0 {
        temp = temp * 10 + num % 10;
        num = num / 10;
    }
    if temp == backup {
        return true;
    }
    return false;
}
