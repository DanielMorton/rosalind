#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct FASTA {
    pub(crate) title: String,
    pub(crate) text: String,
}

impl FASTA {
    pub(crate) fn new(title: &str, dna: &str) -> Self {
        FASTA {
            title: title.to_owned(),
            text: dna.to_owned(),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.text.len()
    }
}