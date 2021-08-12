use std::io::prelude::*;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn run_test_case(code_path: &Path, input_case: Vec<String>, output_case: Vec<String>) {
    let mut index = 1;

    for i in 0..input_case.len() {
        let mut process = Command::new("python3")
            .arg("-u")
            .arg(code_path.as_os_str())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let child_stdin = process
            .stdin
            .as_mut()
            .unwrap()
            .write_all(input_case[i].as_bytes())
            .unwrap();
        drop(child_stdin);

        let code_output = process.wait_with_output().unwrap();
        let code_output_str = std::str::from_utf8(&code_output.stdout).unwrap();
        print_result(index, &output_case[i], &code_output_str);
        index += 1;
    }
}

fn print_result(index: u32, output: &str, answer: &str) {
    if output == answer {
        println!("◯◯◯◯◯◯ 예제 {}번 정답입니다! ◯◯◯◯◯◯", index);
    } else {
        println!("✗✗✗✗✗ 예제 {}번 오답입니다! ✗✗✗✗✗", index);
        println!("정답: ");
        println!("{}", output);
        println!("출력: ");
        println!("{}", answer);
    };
}
