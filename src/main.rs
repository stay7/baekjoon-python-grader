extern crate reqwest;
extern crate select;

use scraper::{Html, Selector};
use std::env;
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::vec::Vec;

fn main() {
    let args: Vec<String> = env::args().collect();
    let problem_no = args[1].parse::<u32>().unwrap();
    let sample_data = get_sample_data(problem_no);
    let test_case = save_sample_data(problem_no, sample_data);
    let path = Path::new("/Users/code-wonderkids/workspace/stay/algorithm/acmicpc/1991.py");
    // let path = Path::new(&code_path);
    run_code(path, &test_case);
}

fn save_sample_data(
    problem_no: u32,
    sample_data: (Vec<String>, Vec<String>),
) -> Vec<(PathBuf, PathBuf)> {
    let (input_cases, answer_cases) = sample_data;
    let (input_dir, answer_dir) = check_data_dir();

    let mut test_case: Vec<(PathBuf, PathBuf)> = Vec::new();

    for i in 0..input_cases.len() {
        let file_name = format!("{}_{}.txt", problem_no, i + 1);
        let input_path = Path::new(".").join(input_dir.as_path()).join(&file_name);
        let answer_path = Path::new(".").join(answer_dir.as_path()).join(&file_name);
        let mut input_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&input_path)
            .unwrap();
        let mut output_file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&answer_path)
            .unwrap();
        input_file.write_all(input_cases[i].as_bytes()).unwrap();
        output_file.write_all(answer_cases[i].as_bytes()).unwrap();
        test_case.push((input_path, answer_path));
    }
    test_case
}

fn check_data_dir() -> (PathBuf, PathBuf) {
    let input_dir = Path::new("input");
    if !input_dir.exists() {
        fs::create_dir("input").unwrap();
    };
    let answer_dir = Path::new("output");
    if !answer_dir.exists() {
        fs::create_dir("output").unwrap();
    }
    (input_dir.to_path_buf(), answer_dir.to_path_buf())
}

fn get_sample_data(num: u32) -> (Vec<String>, Vec<String>) {
    let url = format!("https://www.acmicpc.net/problem/{}", num);
    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());
    let body = resp.text().unwrap();
    let document = Html::parse_document(&body);
    let sampledata = Selector::parse(".sampledata").unwrap();

    let mut input = Vec::new();
    let mut output = Vec::new();

    for (i, data) in document.select(&sampledata).enumerate() {
        let text: Vec<_> = data.text().collect();
        let test_data = String::from(text[0]);

        if i % 2 == 0 {
            input.push(test_data)
        } else {
            output.push(test_data)
        };
    }
    assert_eq!(input.len(), output.len());
    (input, output)
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
