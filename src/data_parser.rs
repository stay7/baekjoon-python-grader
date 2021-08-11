use scraper::{Html, Selector};
use std::io::prelude::*;
use std::path::Path;

pub mod file_manager {
    use std::fs;
    use std::path::Path;

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
        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        file
    }
}

pub struct DataParser {
    input_list: Vec<String>,
    answer_list: Vec<String>,
}

impl DataParser {
    pub fn new() -> DataParser {
        DataParser {
            input_list: Vec::new(),
            answer_list: Vec::new(),
        }
    }

    pub fn load_sample_data(&mut self, problem_no: u32) {
        let url = format!("https://www.acmicpc.net/problem/{}", problem_no);
        let resp = reqwest::blocking::get(url).unwrap();
        assert!(resp.status().is_success());

        let body = resp.text().unwrap();
        let document = Html::parse_document(&body);
        let sampledata = Selector::parse(".sampledata").unwrap();

        for (i, data) in document.select(&sampledata).enumerate() {
            let text: Vec<_> = data.text().collect();
            let test_data = String::from(text[0]);
            if i % 2 == 0 {
                self.input_list.push(test_data)
            } else {
                self.answer_list.push(test_data)
            };
        }
        assert_eq!(self.input_list.len(), self.answer_list.len());
    }

    pub fn save_sample_data(&mut self, problem_no: u32) {
        if self.input_list.len() == 0 || self.answer_list.len() == 0 {
            self.load_sample_data(problem_no);
        };

        for i in 0..self.input_list.len() {
            let file_name = format!("{}_{}.txt", problem_no, i + 1);
            let input_file_path = Path::new("./input").join(&file_name);
            let output_file_path = Path::new("./output").join(&file_name);

            let mut input_file = file_manager::open_file(&input_file_path);
            let mut output_file = file_manager::open_file(&output_file_path);
            input_file.write_all(self.input_list[i].as_bytes()).unwrap();
            output_file
                .write_all(self.answer_list[i].as_bytes())
                .unwrap();
        }
    }
}
