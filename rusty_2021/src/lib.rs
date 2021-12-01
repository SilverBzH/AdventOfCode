use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

pub fn read_input(file_path: &str) -> String {
    let mut input = String::new();
    let mut file = File::open(file_path).unwrap_or_else(|err| {
        eprintln!("Failed to open file: {}", err);
        exit(1);
    });
    file.read_to_string(&mut input).unwrap_or_else(|err| {
        eprintln!("Failed to read file: {}", err);
        exit(1);
    });
    input
}

mod test {

    #[test]
    fn input_day01_2020() {
        // let inputi: String = ["1721", "979", "366", "299", "675", "1456"];
        let input_test: String = "1721\n979\n366\n299\n675\n1456\n".to_string();
        let input = super::read_input("input/input_day01_2020.txt");

        assert_eq!(input_test, input);
    }
}
