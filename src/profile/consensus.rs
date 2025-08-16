use crate::fasta::Fasta;
use crate::util::read_string;
use crate::Result;

#[derive(Debug)]
struct ProfileMatrix {
    a_counts: Vec<usize>,
    c_counts: Vec<usize>,
    g_counts: Vec<usize>,
    t_counts: Vec<usize>,
    length: usize,
}

impl ProfileMatrix {
    fn new(length: usize) -> Self {
        ProfileMatrix {
            a_counts: vec![0; length],
            c_counts: vec![0; length],
            g_counts: vec![0; length],
            t_counts: vec![0; length],
            length,
        }
    }

    fn add_sequence(&mut self, sequence: &str) {
        for (i, nucleotide) in sequence.bytes().enumerate() {
            match nucleotide {
                b'A' => self.a_counts[i] += 1,
                b'C' => self.c_counts[i] += 1,
                b'G' => self.g_counts[i] += 1,
                b'T' => self.t_counts[i] += 1,
                _ => panic!("Invalid nucleotide: {}", nucleotide as char),
            }
        }
    }

    fn get_consensus(&self) -> String {
        let mut consensus = String::with_capacity(self.length);

        for i in 0..self.length {
            let max_count = [
                self.a_counts[i],
                self.c_counts[i],
                self.g_counts[i],
                self.t_counts[i],
            ]
            .iter()
            .max()
            .copied()
            .unwrap();

            // Find the first nucleotide with max count (in ACGT order for consistency)
            let consensus_nucleotide = if self.a_counts[i] == max_count {
                'A'
            } else if self.c_counts[i] == max_count {
                'C'
            } else if self.g_counts[i] == max_count {
                'G'
            } else {
                'T'
            };

            consensus.push(consensus_nucleotide);
        }

        consensus
    }

    fn print_profile(&self) {
        print!("A:");
        for count in &self.a_counts {
            print!(" {}", count);
        }
        println!();

        print!("C:");
        for count in &self.c_counts {
            print!(" {}", count);
        }
        println!();

        print!("G:");
        for count in &self.g_counts {
            print!(" {}", count);
        }
        println!();

        print!("T:");
        for count in &self.t_counts {
            print!(" {}", count);
        }
        println!();
    }
}

fn solve_consensus_profile(fasta_input: &[Fasta]) -> Result<(String, ProfileMatrix)> {
    if fasta_input.is_empty() {
        panic!("No sequences found");
    }

    let sequence_length = fasta_input[0].len();

    // Verify all sequences have the same length
    for seq in fasta_input {
        if seq.len() != sequence_length {
            panic!("All sequences must have the same length");
        }
    }

    let mut profile = ProfileMatrix::new(sequence_length);

    // Build profile matrix by counting nucleotides at each position
    for sequence in fasta_input {
        profile.add_sequence(&sequence.text);
    }

    let consensus = profile.get_consensus();

    Ok((consensus, profile))
}

pub fn execute_cons(input_file: &str) -> Result<()> {
    let data = read_string(input_file)?;
    let dna_list = Fasta::parse(&data)?;
    let (consensus, profile) = solve_consensus_profile(&dna_list)?;
    println!("{}", consensus);
    profile.print_profile();
    Ok(())
}
