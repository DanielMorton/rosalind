use std::fs;

pub(crate) fn read_edges(file: &str) -> Vec<(u32, u32)> {
    fs::read_to_string(file)
        .map(|s| s.trim().split('\n')
            .map(|s| {
                let mut split = s.split(' ');
                (split.next().map(|c| c.parse::<u32>()).unwrap().unwrap(),
                 split.next().map(|c| c.parse::<u32>()).unwrap().unwrap())
            }).collect::<Vec<_>>()).unwrap()[1..].to_owned()
}