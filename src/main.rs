extern crate reqwest;
extern crate select;

use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::vec::Vec;

mod data_parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem_no = args[1].parse::<u32>().unwrap();
    let parser = data_parser::DataParser::new();
    // let path = Path::new("/Users/code-wonderkids/workspace/stay/algorithm/acmicpc/1991.py");
    // let path = Path::new(&code_path);
    // run_code(path, &test_case);
}

fn run_code(path: &Path, test_case: &Vec<(PathBuf, PathBuf)>) {
    let mut index = 1;
    for (input, answer) in test_case {
        let input_data = fs::read_to_string(input).unwrap();
        let answer_data = fs::read_to_string(answer).unwrap();
        let mut process = Command::new("python3")
            .arg("-u")
            .arg(path.as_os_str())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let child_stdin = process
            .stdin
            .as_mut()
            .unwrap()
            .write_all(input_data.as_bytes())
            .unwrap();
        drop(child_stdin);
        let result = process.wait_with_output().unwrap();
        let output_data = std::str::from_utf8(&result.stdout).unwrap();
        print_result(index, &output_data, &answer_data);
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
