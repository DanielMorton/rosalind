mod dna;
mod fasta;
mod fibonacci;
mod gc;
mod mendel;
mod motifs;
mod parse;
mod profile;
mod protein;

use crate::fasta::{pairs, read_fasta};
use crate::motifs::find_motifs;
use crate::profile::find_consensus;
use crate::protein::find_orfs;
use dna::{dna_to_rna, nucleotide_count, reverse_complement};
use fibonacci::k_fibonacci;
use gc::gc_max;
use mendel::{expected_offspring, first_law, second_law};
use motifs::motif_start;
use motifs::{hamming_distance, read_two_line};
use parse::RosalindParse;
use protein::rna_to_protein;
use protein::{protein_mass, rna_count};
use regex::Regex;
use reqwest::blocking::Client;
use std::collections::HashSet;
use std::fs;

fn main() {
    let matches = parse::parse();
    let file_type = matches.get_file_type();
    let file = matches.get_file();

    if file_type == "dna" {
        let dna = match fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
        nucleotide_count(&dna).iter().for_each(|n| print!("{} ", n));
        println!();
    } else if file_type == "rna" {
        let dna = match fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
        let rna = dna_to_rna(&dna);
        println!("{}", rna)
    } else if file_type == "revc" {
        let dna = match fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
        let revc = reverse_complement(&dna);
        println!("{}", revc)
    } else if file_type == "iprb" {
        let nums = match fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
        let mut num_split = nums.trim().split(' ');
        let (k, m, n) = (
            num_split.next().unwrap().parse::<u32>().unwrap(),
            num_split.next().unwrap().parse::<u32>().unwrap(),
            num_split.next().unwrap().parse::<u32>().unwrap(),
        );
        let p = first_law(k, m, n);
        println!("{}", p);
    } else if file_type == "fib" {
        let nums = match fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
        let mut num_split = nums.trim().split(' ');
        let (n, k) = (
            num_split.next().unwrap().parse::<u64>().unwrap(),
            num_split.next().unwrap().parse::<u64>().unwrap(),
        );
        println!("{}", k_fibonacci(n, k))
    } else if file_type == "gc" {
        let (title, gc) = gc_max(&file);
        println!("{title}");
        println!("{gc}")
    } else if file_type == "prot" {
        let rna = match fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
        let protein = rna_to_protein(&rna);
        println!("{protein}")
    } else if file_type == "subs" {
        let (dna, motif) = read_two_line(&file);
        let positions = motif_start(&dna, &motif);
        positions.iter().for_each(|p| print!("{p} "));
        println!();
    } else if file_type == "hamm" {
        let (s, t) = read_two_line(&file);
        let h = hamming_distance(&s, &t);
        println!("{h}")
    } else if file_type == "iev" {
        let nums = match fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
        let num_vec = nums
            .trim()
            .split(' ')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        println!("{}", expected_offspring(&num_vec))
    } else if file_type == "mrna" {
        let protein = match fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
        println!("{}", rna_count(&protein))
    } else if file_type == "lia" {
        let nums = match fs::read_to_string(file) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
        let mut num_split = nums.trim().split(' ');
        let (k, n) = (
            num_split.next().unwrap().parse::<u32>().unwrap(),
            num_split.next().unwrap().parse::<u32>().unwrap(),
        );
        println!("{}", second_law(k, n, 0.25))
    } else if file_type == "prtm" {
        let protein = match fs::read_to_string(file) {
            Ok(s) => s.trim().to_owned(),
            Err(e) => panic!("{:?}", e),
        };
        println!("{}", protein_mass(&protein))
    } else if file_type == "grph" {
        let edges = pairs(&file, 3);
        edges.iter().for_each(|(f1, f2)| println!("{} {}", f1, f2))
    } else if file_type == "mprt" {
        let names = match fs::read_to_string(file).map(|s| {
            s.trim()
                .split('\n')
                .map(|t| t.to_owned())
                .collect::<Vec<_>>()
        }) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
        let client = Client::new();
        let re = Regex::new("N[^P](S|T)[^P]").unwrap();
        find_motifs(&client, &names, &re)
    } else if file_type == "cons" {
        let dna_list = read_fasta(&file)
            .iter()
            .map(|f| f.dna.clone())
            .collect::<Vec<_>>();
        println!("{}", find_consensus(&dna_list))
    } else if file_type == "orf" {
        let dna = match fs::read_to_string(file).map(|s| {
            let mut split = s.split('\n');
            let _ = split.next();
            split.collect::<String>()
        }) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
        find_orfs(&dna).iter()
            .collect::<HashSet<_>>()
            .iter()
            .for_each(|orf| println!("{orf}"))
    }
}
