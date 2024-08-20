use std::fs::read_to_string;
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct FASTA {
    pub(crate) title: String,
    pub(crate) text: String,
}

impl FASTA {
    fn new(title: &str, dna: &str) -> Self {
        FASTA {
            title: title.to_owned(),
            text: dna.to_owned(),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.text.len()
    }
}

pub(crate) fn read_fasta(file: &str) -> Vec<FASTA> {
    match read_to_string(file).map(|s| {
        s.trim()
            .split("\n>")
            .map(|s| {
                let mut read = s.split('\n');
                let title = read.next().unwrap().replace('>', "");
                let dna = read.collect::<String>();
                FASTA::new(&title, &dna)
            })
            .collect::<Vec<_>>()
    }) {
        Ok(f) => f,
        Err(e) => panic!("{:?}", e),
    }
}
