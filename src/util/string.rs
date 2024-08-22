use std::fs;
use std::str::FromStr;

pub(crate) fn read_num_list<T: FromStr>(file: &str, sep: char) -> Vec<T> {
    read_string(file)
        .trim()
        .split(sep)
        .flat_map(|s| s.parse::<T>())
        .collect::<Vec<_>>()
}

pub(crate) fn read_string(file: &str) -> String {
    match fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e),
    }
}

pub(crate) fn read_two_line(file: &str) -> (String, String) {
    let s = read_string(file);
    let mut split = s.split('\n');
    (
        split.next().unwrap().to_owned(),
        split.next().unwrap().to_owned(),
    )
}

pub(crate) fn read_vec(file: &str, sep: char) -> Vec<String> {
    let text = read_string(file);
    text.trim()
        .split(sep)
        .map(|t| t.to_owned())
        .collect::<Vec<_>>()
}
