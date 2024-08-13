use std::fs;

pub(crate) fn read_two_line(file: &str) -> (String, String) {
    let text = match fs::read_to_string(file) {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e),
    };
    let mut split = text.split('\n');
    (
        split.next().unwrap().to_owned(),
        split.next().unwrap().to_owned(),
    )
}
