use crate::gc::gc_content;
use crate::util::{Error, Result};
use std::str::Chars;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Fasta {
    pub(crate) title: String,
    pub(crate) text: String,
}

impl Fasta {
    pub(crate) fn chars(&self) -> Chars {
        self.text.chars()
    }
    pub(crate) fn new(title: &str, dna: &str) -> Self {
        Self {
            title: title.into(),
            text: dna.into(),
        }
    }
    pub(crate) fn parse_single(content: &str) -> Result<Self> {
        let mut lines = content.lines().map(str::trim).filter(|l| !l.is_empty());

        let title_line = lines
            .next()
            .ok_or_else(|| Error::Parse("Missing FASTA header".to_string()))?;
        let title = title_line
            .strip_prefix('>')
            .ok_or_else(|| Error::Parse("FASTA header must start with '>'".to_string()))?
            .trim();
        if title.is_empty() {
            return Err(Error::Parse("Empty sequence ID found".to_string()));
        }

        let mut sequence = String::new();
        for line in lines {
            let cleaned: String = line.chars().filter(|c| !c.is_whitespace()).collect();
            if !cleaned
                .chars()
                .all(|c| matches!(c.to_ascii_uppercase(), 'A' | 'T' | 'G' | 'C' | 'U' | 'N'))
            {
                return Err(Error::InvalidSequence(format!(
                    "Invalid characters in sequence: {}",
                    line
                )));
            }
            sequence.push_str(&cleaned);
        }

        if sequence.is_empty() {
            return Err(Error::Parse(format!("Empty sequence for ID: {}", title)));
        }

        Ok(Fasta::new(title, &sequence))
    }

    /// Parse FASTA format content into sequences
    pub fn parse(content: &str) -> Result<Vec<Self>> {
        let mut sequences = Vec::new();
        let mut buffer = String::new();

        for line in content.lines() {
            if line.starts_with('>') {
                if !buffer.is_empty() {
                    sequences.push(Fasta::parse_single(&buffer)?);
                    buffer.clear();
                }
            }
            buffer.push_str(line);
            buffer.push('\n');
        }

        if !buffer.trim().is_empty() {
            sequences.push(Fasta::parse_single(&buffer)?);
        }

        if sequences.is_empty() {
            Err(Error::Parse("No valid FASTA sequences found".to_string()))
        } else {
            Ok(sequences)
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.text.len()
    }

    pub(crate) fn gc_content(&self) -> f64 {
        gc_content(&self.text)
    }
}

pub(crate) type Dna = Fasta;

impl Dna {
    pub(crate) fn dna(&self) -> String {
        self.text.to_owned()
    }
}

pub(crate) type Rna = Fasta;

impl Rna {
    pub(crate) fn rna(&self) -> String {
        self.text.to_owned()
    }
}
