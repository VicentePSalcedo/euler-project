use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn write_down_answer(question_num: i32, text: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("answer.txt")?;
    writeln!(&mut file, "{}. {}", question_num, text)?;

    Ok(())
}
