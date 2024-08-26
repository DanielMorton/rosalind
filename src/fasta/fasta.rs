use std::fs::read_to_string;
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
        Fasta {
            title: title.to_owned(),
            text: dna.to_owned(),
        }
    }
    pub(crate) fn read(text: &str) -> Self {
        let mut read = text.split('\n');
        let title = read.next().unwrap().replace('>', "");
        let dna = read.collect::<String>();
        Self::new(&title, &dna)
    }

    pub(crate) fn read_file(file: &str) -> Self {
        match read_to_string(file).map(|s| Self::read(s.trim())) {
            Ok(f) => f,
            Err(e) => panic!("{:?}", e),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.text.len()
    }
}

pub(crate) type dna = Fasta;

impl dna {
    pub(crate) fn dna(&self) -> String {
        self.text.to_owned()
    }
}

pub(crate) type rna = Fasta;

impl rna {
    pub(crate) fn rna(&self) -> String {
        self.text.to_owned()
    }
}
