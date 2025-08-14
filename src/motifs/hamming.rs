use crate::util::{read_string, Error, Result};
pub(crate) fn hamming_distance(s: &str, t: &str) -> usize {
    let sb = s.as_bytes();
    let tb = t.as_bytes();

    assert_eq!(sb.len(), tb.len(), "Strings must be of equal length");

    sb.iter().zip(tb.iter()).filter(|(&a, &b)| a != b).count()
}

pub(crate) fn execute_hamm(input_file: &str) -> Result<()> {
    let text = read_string(input_file)?;
    let mut text_split = text.split_whitespace();

    let s = text_split
        .next()
        .ok_or_else(|| Error::Parse("Missing first DNA string".to_string()))?;
    let t = text_split
        .next()
        .ok_or_else(|| Error::Parse("Missing second DNA string".to_string()))?;

    println!("{}", hamming_distance(s, t));
    Ok(())
}
