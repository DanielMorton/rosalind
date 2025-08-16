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
        let mut current_title: Option<String> = None;
        let mut current_sequence = String::new();

        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();

            // Skip empty lines
            if line.is_empty() {
                continue;
            }

            if line.starts_with('>') {
                // If we have a previous sequence, save it
                if let Some(title) = current_title.take() {
                    if !current_sequence.is_empty() {
                        sequences.push(Fasta::new(&title, &current_sequence));
                        current_sequence.clear();
                    }
                }

                // Start new sequence
                let title = line
                    .strip_prefix('>')
                    .ok_or_else(|| Error::Parse("FASTA header must start with '>'".to_string()))?
                    .trim();

                if title.is_empty() {
                    return Err(Error::Parse("Empty sequence ID found".to_string()));
                }

                current_title = Some(title.to_string());
            } else {
                // This is sequence data
                if current_title.is_none() {
                    return Err(Error::Parse(format!(
                        "Sequence data found before header at line {}: '{}'",
                        line_num + 1,
                        line
                    )));
                }

                // Clean and validate the sequence line
                let cleaned: String = line
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .map(|c| c.to_ascii_uppercase())
                    .collect();

                if !cleaned
                    .chars()
                    .all(|c| matches!(c, 'A' | 'T' | 'G' | 'C' | 'U' | 'N'))
                {
                    return Err(Error::InvalidSequence(format!(
                        "Invalid characters in sequence: {} (cleaned: {})",
                        line, cleaned
                    )));
                }
                current_sequence.push_str(&cleaned);
            }
        }

        // Don't forget the last sequence
        if let Some(title) = current_title {
            if current_sequence.is_empty() {
                return Err(Error::Parse(format!("Empty sequence for ID: {}", title)));
            }
            sequences.push(Fasta::new(&title, &current_sequence));
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
