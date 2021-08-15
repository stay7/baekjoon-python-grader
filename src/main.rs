extern crate reqwest;
extern crate select;

mod utils {
    pub mod data_parser;
    pub mod file_manager;
    pub mod tester;
}

use std::env;
use std::path::Path;
use std::vec::Vec;

use utils::data_parser;
use utils::file_manager;
use utils::file_manager::CaseType;
use utils::tester;

const COMMAND_GET: &str = "get";
const COMMAND_TEST: &str = "test";
const COMMAND_HELP: &str = "help";
const COMMAND_TOUCH: &str = "touch";
const COMMAND_START: &str = "start";

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    match command.as_ref() {
        COMMAND_GET => get_problem(&args[2]),
        COMMAND_TEST => test_problem(&args[2]),
        COMMAND_START => start_problem(&args[2]),
        COMMAND_TOUCH => touch_code_file(&args[2]),
        COMMAND_HELP => show_help(),
        _ => show_error(),
    }
}

fn get_problem(arg: &String) {
    let problem_no = arg.parse::<u32>().expect("problem number not found!");
    let mut parser = data_parser::DataParser::new();
    file_manager::create_dir();
    parser.load_test_case(problem_no);
    parser.save_test_case(problem_no);
}

fn test_problem(arg: &String) {
    let problem_no = arg.parse::<u32>().expect("problem number not found!");
    let input_cases = file_manager::read_test_case_by_type(CaseType::Input, problem_no);
    let output_cases = file_manager::read_test_case_by_type(CaseType::Output, problem_no);
    let code_path = format!("{}.py", problem_no);
    tester::run_test_case(Path::new(&code_path), input_cases, output_cases);
}

fn start_problem(arg: &String) {
    get_problem(arg);
    touch_code_file(arg);
}

fn touch_code_file(arg: &String) {
    let filename = format!("{}.{}", arg, "py");
    file_manager::create_file(&filename);
}

fn show_help() {
    println!("show help");
}
fn show_error() {
    println!("argument not found!");
}
