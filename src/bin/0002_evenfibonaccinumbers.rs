use crate::utils::write_down_answer;
#[path = "../utils.rs"]
mod utils;
fn main() {
    let question_number = 2;
    println!(
        "Each new term in the Fibonacci sequence is generated by adding the previous two
terms. By starting with 1 and 2, the first 10 terms will be:
    1,2,3,5,8,13,21,34,55,89...
By considering the terms in the Fibonacci sequence whose values do not exceed
four million, find the answer of the even-valued terms."
    );

    let mut a = 1;
    let mut b = 2;
    let mut answer = 0;
    for _ in 1.. {
        if b % 2 == 0 {
            answer += b;
        }
        let temp = a;
        a = b;
        b = temp + b;
        if b > 4000000 {
            break;
        }
    }
    println!("{}", answer);
    if let Err(error) = write_down_answer(question_number, &answer.to_string()) {
        eprintln!("Error writing down answer: {}", error);
    }
}
