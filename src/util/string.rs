use super::error::Result;
use std::fs;
use std::str::FromStr;

pub(crate) fn read_num_list<T: FromStr>(file: &str, sep: char) -> Result<Vec<T>> {
    let text = read_string(file)?;
    Ok(text
        .trim()
        .split(sep)
        .flat_map(|s| s.parse::<T>())
        .collect::<Vec<_>>())
}

pub(crate) fn read_string(file: &str) -> Result<String> {
    Ok(fs::read_to_string(file)?)
}

pub(crate) fn read_two_line(file: &str) -> Result<(String, String)> {
    let s = read_string(file)?;
    let mut split = s.split('\n');
    Ok((
        split.next().unwrap().to_owned(),
        split.next().unwrap().to_owned(),
    ))
}

pub(crate) fn read_vec(file: &str, sep: char) -> Result<Vec<String>> {
    let text = read_string(file)?;
    Ok(text
        .trim()
        .split(sep)
        .map(|t| t.to_owned())
        .collect::<Vec<_>>())
}

pub(crate) fn read_lines(file: &str) -> Result<Vec<String>> {
    read_vec(file, '\n')
}
