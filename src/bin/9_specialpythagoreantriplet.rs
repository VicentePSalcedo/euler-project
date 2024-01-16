use crate::utils::write_down_answer;
#[path = "../utils.rs"]
mod utils;
fn main() {
    let question_number = 9;
    println!(
        "
A Pithagorean triplet is a set of three natural numbers, a < b < c, for which,
    a^2 + b^2 = c^2.
For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc."
    );
    let answer = special_pythagorean_triplet();
    println!("{}", answer);
    if let Err(error) = write_down_answer(question_number, &answer.to_string()) {
        eprintln!("Error writing down answer: {}", error);
    }
}
fn special_pythagorean_triplet() -> i32 {
    let mut m = 2;
    loop {
        for n in 1..m {
            let a = m * m - n * n;
            let b = 2 * m * n;
            let c = m * m + n * n;

            if a + b + c == 1000 {
                return a * b * c;
            }
        }
        m += 1;
    }
}
