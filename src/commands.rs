use clap::{Args, Parser, Subcommand};

#[derive(Args)]
pub struct FileArgs {
    /// Input file containing the problem data
    #[arg(short, long)]
    pub file: String,
}

#[derive(Parser)]
#[command(name = "rosalind-solver")]
#[command(about = "A CLI tool for solving bioinformatics problems from Rosalind")]
#[command(version = "1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Count DNA nucleotides
    Dna(FileArgs),
    /// Introduction problem (same as DNA)
    Ini(FileArgs),
    /// Transcribe DNA to RNA
    Rna(FileArgs),
    /// Reverse complement of DNA
    Revc(FileArgs),
    /// Mendel's First Law
    Iprb(FileArgs),
    /// Rabbits and Recurrence Relations
    Fib(FileArgs),
    /// Computing GC Content
    Gc(FileArgs),
    /// Translating RNA into Protein
    Prot(FileArgs),
    /// Finding a Motif in DNA
    Subs(FileArgs),
    /// Counting Point Mutations
    Hamm(FileArgs),
    /// Calculating Expected Offspring
    Iev(FileArgs),
    /// Inferring mRNA from Protein
    Mrna(FileArgs),
    /// Independent Alleles
    Lia(FileArgs),
    /// Calculating Protein Mass
    Prtm(FileArgs),
    /// Overlap Graphs
    Grph(FileArgs),
    /// Finding a Protein Motif
    Mprt(FileArgs),
    /// Consensus and Profile
    Cons(FileArgs),
    /// Open Reading Frames
    Orf(FileArgs),
    /// RNA Splicing
    Splc(FileArgs),
    /// Finding a Shared Motif
    Lcsm(FileArgs),
    /// Enumerating Gene Orders
    Perm(FileArgs),
    /// Completing a Tree
    Tree(FileArgs),
    /// Counting Phylogenetic Ancestors
    Inod(FileArgs),
    /// Partial Permutations
    Pper(FileArgs),
    /// Introduction to Random Strings
    Prob(FileArgs),
    /// Locating Restriction Sites
    Revp(FileArgs),
    /// Perfect Matchings and RNA Secondary Structures
    Pmch(FileArgs),
    /// Maximum Matchings and RNA Secondary Structures
    Mmch(FileArgs),
    /// Creating a Distance Matrix
    Pdst(FileArgs),
    /// Genome Assembly as Shortest Superstring
    Long(FileArgs),
    /// Finding a Spliced Motif
    Sseq(FileArgs),
    /// Transitions and Transversions
    Tran(FileArgs),
    /// Enumerating k-mers Lexicographically
    Lexf(FileArgs),
    /// Longest Increasing Subsequence
    Lgis(FileArgs),
    /// Enumerating Oriented Gene Orderings
    Sign(FileArgs),
    /// Catalan Numbers and RNA Secondary Structures
    Cat(FileArgs),
    /// Speeding Up Motif Finding
    Kmp(FileArgs),
    /// k-Mer Composition
    Kmer(FileArgs),
    /// Degree Array
    Deg(FileArgs),
    /// Binary Search
    Bins(FileArgs),
    /// Merge Two Sorted Arrays
    Mer(FileArgs),
    /// Merge Sort
    Ms(FileArgs),
    /// Counting Inversions
    Inv(FileArgs),
    /// 2SUM
    #[command(name = "2sum")]
    TwoSum(FileArgs),
    Fibd(FileArgs),
}
