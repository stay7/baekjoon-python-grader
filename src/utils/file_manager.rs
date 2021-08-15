use regex;
use std::fs;
use std::path::{Path, PathBuf};

pub fn create_dir() {
    let input_dir = Path::new("input");
    if !input_dir.exists() {
        fs::create_dir("input").unwrap();
    };
    let answer_dir = Path::new("output");
    if !answer_dir.exists() {
        fs::create_dir("output").unwrap();
    }
}

pub fn open_file(path: &Path) -> fs::File {
    let file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create_new(true)
        .open(&path)
        .unwrap();
    file
}

pub fn create_file(filename: &String) {
    let path = Path::new(filename);
    if path.exists() {
        println!("🚨 이미 존재하는 파일입니다.")
    } else {
        fs::File::create(filename).unwrap();
    }
}

pub enum CaseType {
    Input,
    Output,
}

pub fn read_test_case_by_type(case_type: CaseType, problem_no: u32) -> Vec<String> {
    let regex_template =
        regex::Regex::new(format!(r"^{}_\d{{1}}.txt", problem_no).as_str()).unwrap();
    let dir_name = match case_type {
        CaseType::Input => "./input",
        CaseType::Output => "./output",
    };
    let entries = fs::read_dir(dir_name).unwrap();

    let mut file_paths: Vec<PathBuf> = Vec::new();
    let mut input_cases: Vec<String> = Vec::new();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if regex_template.is_match(file_name) {
            file_paths.push(path);
        };
    }

    for path in file_paths {
        let data = fs::read_to_string(path).unwrap();
        input_cases.push(data);
    }
    input_cases
}
