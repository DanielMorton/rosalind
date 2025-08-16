mod commands;
mod dna;
mod fasta;
mod fibonacci;
mod gc;
mod gene;
mod graph;
mod mendel;
mod motifs;
mod profile;
mod protein;
mod util;

use crate::dna::{execute_dna, rna_nucleotide_count, transcribe_dna_to_rna};
use crate::fasta::{execute_grph, transition_transversion_ratio, Fasta};
use crate::gene::{longest_decreasing_sequence, longest_increasing_sequence};
use crate::graph::{
    align, catalan_number, edges_to_degrees, inner_nodes, read_edges, tree_edge_fill,
};
use crate::mendel::{dna_prob, execute_iev, execute_iprb, factorial, npr, permutation_list, permute};
use crate::motifs::{
    build_failure_array, execute_hamm, execute_subs, find_motifs, get_subsequence, kmer_count, lcs,
    make_dictionary, reverse_palindrome,
};
use crate::profile::execute_cons;
use crate::protein::{execute_prot, execute_splc, find_orfs};
use crate::util::{
    binary_search, inversion_count, merge, merge_sort, read_lines, read_string,
    read_vec, two_sum, DNA,
};
use dna::reverse_complement;
use mendel::second_law;
use motifs::hamming_distance;
use protein::{protein_mass, rna_count};

use crate::commands::{Cli, Commands};
use crate::fibonacci::{execute_fib, execute_fibd};
use crate::gc::execute_gc;
use clap::Parser;
use regex::Regex;
use reqwest::blocking::Client;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;
use util::Result;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Dna(args) | Commands::Ini(args) => execute_dna(&args.file),
        Commands::Rna(args) => transcribe_dna_to_rna(&args.file),
        Commands::Revc(args) => reverse_complement(&args.file),
        Commands::Iprb(args) => execute_iprb(&args.file),
        Commands::Fib(args) => execute_fib(&args.file),
        Commands::Gc(args) => execute_gc(&args.file),
        Commands::Prot(args) => execute_prot(&args.file),
        Commands::Subs(args) => execute_subs(&args.file),
        Commands::Hamm(args) => execute_hamm(&args.file),
        Commands::Fibd(args) => execute_fibd(&args.file),
        Commands::Iev(args) => execute_iev(&args.file),
        Commands::Mrna(args) => {
            let protein = read_string(&args.file)?;
            println!("{}", rna_count(&protein));
            Ok(())
        }
        Commands::Lia(args) => {
            let nums = fs::read_to_string(&args.file).expect("Failed to read file");
            let mut num_split = nums.trim().split(' ');
            let (k, n) = (
                num_split.next().unwrap().parse::<u32>().unwrap(),
                num_split.next().unwrap().parse::<u32>().unwrap(),
            );
            println!("{}", second_law(k, n, 0.25));
            Ok(())
        }
        Commands::Prtm(args) => {
            let protein = read_string(&args.file)?;
            println!("{}", protein_mass(&protein));
            Ok(())
        }
        Commands::Grph(args) => execute_grph(&args.file),
        Commands::Mprt(args) => {
            let names = read_vec(&args.file, '\n')?;
            let client = Client::new();
            let re = Regex::new("N[^P](S|T)[^P]").unwrap();
            find_motifs(&client, &names, &re);
            Ok(())
        }
        Commands::Cons(args) => execute_cons(&args.file),
        Commands::Orf(args) => {
            let fasta = Fasta::parse(&args.file)?;
            let dna = fasta[0].text.clone();
            find_orfs(&dna)?
                .iter()
                .collect::<HashSet<_>>()
                .iter()
                .for_each(|orf| println!("{orf}"));
            Ok(())
        }
        Commands::Splc(args) => execute_splc(&args.file),
        Commands::Lcsm(args) => {
            let fasta = Fasta::parse(&args.file)?;
            println!("{}", lcs(&fasta));
            Ok(())
        }
        Commands::Perm(args) => {
            let n = fs::read_to_string(&args.file)
                .expect("Failed to read file")
                .trim()
                .parse::<usize>()
                .unwrap();
            println!("{}", factorial(n));
            permute(n).into_iter().for_each(|v| {
                v.into_iter().for_each(|p| print!("{} ", p));
                println!();
            });
            Ok(())
        }
        Commands::Tree(args) => {
            let (n, edges) = {
                let content = fs::read_to_string(&args.file).expect("Failed to read file");
                let mut split = content.trim().split('\n');
                let n = split.next().unwrap().parse::<usize>().unwrap();
                let edges = split
                    .map(|e| {
                        let mut e_split = e.split(' ');
                        (
                            e_split.next().unwrap().parse::<usize>().unwrap(),
                            e_split.next().unwrap().parse::<usize>().unwrap(),
                        )
                    })
                    .collect::<Vec<_>>();
                (n, edges)
            };
            println!("{}", tree_edge_fill(n, &edges));
            Ok(())
        }
        Commands::Inod(args) => {
            let n = fs::read_to_string(&args.file)
                .expect("Failed to read file")
                .trim()
                .parse::<usize>()
                .unwrap();
            println!("{}", inner_nodes(n));
            Ok(())
        }
        Commands::Pper(args) => {
            let (n, r) = {
                let content = fs::read_to_string(&args.file).expect("Failed to read file");
                let mut split = content.trim().split(' ');
                (
                    split.next().unwrap().parse::<u64>().unwrap(),
                    split.next().unwrap().parse::<u64>().unwrap(),
                )
            };
            println!("{}", npr(n, r));
            Ok(())
        }
        Commands::Prob(args) => {
            let (dna, probs) = {
                let content = fs::read_to_string(&args.file).expect("Failed to read file");
                let mut split = content.trim().split('\n');
                let dna = split.next().unwrap().to_owned();
                let probs = split
                    .next()
                    .unwrap()
                    .split(' ')
                    .map(|f| f.parse::<f64>().unwrap())
                    .collect::<Vec<_>>();
                (dna, probs)
            };
            probs
                .into_iter()
                .for_each(|p| print!("{} ", dna_prob(&dna, p)));
            println!();
            Ok(())
        }
        Commands::Revp(args) => {
            let fasta = Fasta::parse_single(&args.file)?;
            let palindromes = reverse_palindrome(&fasta, 4, 12);
            palindromes
                .iter()
                .for_each(|(s, e)| println!("{} {}", s, e));
            Ok(())
        }
        Commands::Pmch(args) => {
            let fasta = Fasta::parse_single(&args.file)?;
            let count = rna_nucleotide_count(&fasta.text);
            println!("{:?}", count);
            println!("{}", factorial(count[0]) * factorial(count[1]));
            Ok(())
        }
        Commands::Mmch(args) => {
            let fasta = Fasta::parse_single(&args.file)?;
            let count = rna_nucleotide_count(&fasta.text);
            println!("{:?}", count);
            println!(
                "{}",
                npr(
                    max(count[0], count[3]) as u64,
                    min(count[0], count[3]) as u64
                ) * npr(
                    max(count[1], count[2]) as u64,
                    min(count[1], count[2]) as u64
                )
            );
            Ok(())
        }
        Commands::Pdst(args) => {
            let fasta = Fasta::parse(&args.file)?;
            fasta.iter().for_each(|f1| {
                fasta
                    .iter()
                    .map(|f2| hamming_distance(&f1.text, &f2.text))
                    .for_each(|d| print!("{} ", (d as f64) / (f1.text.len() as f64)));
                println!();
            });
            Ok(())
        }
        Commands::Long(args) => {
            let fasta = Fasta::parse(&args.file)?;
            let alignment = align(&fasta);
            println!("{}", alignment);
            Ok(())
        }
        Commands::Sseq(args) => {
            let fasta = Fasta::parse(&args.file)?;
            get_subsequence(&fasta[0], &fasta[1])
                .iter()
                .for_each(|s| print!("{} ", s));
            println!();
            Ok(())
        }
        Commands::Tran(args) => {
            let fasta = Fasta::parse(&args.file)?;
            let tt = transition_transversion_ratio(&fasta[0], &fasta[1]);
            println!("{}", tt);
            Ok(())
        }
        Commands::Lexf(args) => {
            let (letters, n) = {
                let content = fs::read_to_string(&args.file).expect("Failed to read file");
                let mut split = content.trim().split('\n');
                (
                    split
                        .next()
                        .unwrap()
                        .split(' ')
                        .map(|c| c.to_owned())
                        .collect::<Vec<_>>(),
                    split.next().unwrap().parse::<usize>().unwrap(),
                )
            };
            make_dictionary(&letters.iter().map(|l| l.as_str()).collect::<Vec<_>>(), n)
                .iter()
                .for_each(|d| println!("{}", d));
            Ok(())
        }
        Commands::Lgis(args) => {
            let nums = {
                let content = fs::read_to_string(&args.file).expect("Failed to read file");
                let mut split = content.trim().split('\n');
                let _ = split.next();
                let line = split.collect::<String>();
                line.split(' ')
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            };
            let increase = longest_increasing_sequence(&nums);
            increase.iter().for_each(|n| print!("{} ", n));
            println!();
            let decrease = longest_decreasing_sequence(&nums);
            decrease.iter().for_each(|n| print!("{} ", n));
            println!();
            Ok(())
        }
        Commands::Sign(args) => {
            let n = fs::read_to_string(&args.file)
                .expect("Failed to read file")
                .trim()
                .parse::<i32>()
                .unwrap();
            let p_list = permutation_list(n);
            println!("{}", p_list.len());
            p_list.iter().for_each(|v| {
                v.iter().for_each(|p| print!("{} ", p));
                println!();
            });
            Ok(())
        }
        Commands::Cat(args) => {
            let fasta = Fasta::parse_single(&args.file)?;
            println!("{}", catalan_number(&fasta));
            Ok(())
        }
        Commands::Kmp(args) => {
            let fasta = Fasta::parse(&args.file)?;
            build_failure_array(&fasta[0].text.chars().collect::<Vec<_>>())
                .iter()
                .for_each(|n| print!("{} ", n));
            println!();
            Ok(())
        }
        Commands::Kmer(args) => {
            let fasta = Fasta::parse_single(&args.file)?;
            let dna_dict = make_dictionary(DNA, 4);
            let counts = kmer_count(&fasta.text, 4);
            dna_dict
                .iter()
                .for_each(|d| print!("{} ", counts.get(d).unwrap_or(&0)));
            println!();
            Ok(())
        }
        Commands::Deg(args) => {
            let edges = read_edges(&args.file);
            let degrees = edges_to_degrees(&edges);
            degrees.iter().for_each(|d| print!("{} ", d));
            println!();
            Ok(())
        }
        Commands::Bins(args) => {
            let text = read_string(&args.file)?;
            let split = text.trim().split('\n').collect::<Vec<_>>();
            let nums = split[2]
                .split(' ')
                .flat_map(|s| s.parse::<i32>())
                .collect::<Vec<_>>();
            let values = split[3]
                .split(' ')
                .flat_map(|s| s.parse::<i32>())
                .collect::<Vec<_>>();
            values
                .into_iter()
                .for_each(|v| print!("{} ", binary_search(&nums, v)));
            println!();
            Ok(())
        }
        Commands::Mer(args) => {
            let text = read_lines(&args.file)?;
            let lines = text
                .iter()
                .enumerate()
                .filter(|(i, _)| i % 2 == 1)
                .map(|(_, s)| s)
                .collect::<Vec<_>>();
            let arr1 = lines[0]
                .split(' ')
                .flat_map(|s| s.parse::<i32>())
                .collect::<Vec<_>>();
            let arr2 = lines[1]
                .split(' ')
                .flat_map(|s| s.parse::<i32>())
                .collect::<Vec<_>>();
            let arr = merge(&arr1, &arr2);
            arr.iter().for_each(|n| print!("{} ", n));
            println!();
            Ok(())
        }
        Commands::Ms(args) => {
            let lines = read_lines(&args.file)?;
            let arr = lines[1]
                .split(' ')
                .flat_map(|s| s.parse::<i32>())
                .collect::<Vec<_>>();
            merge_sort(&arr).iter().for_each(|s| print!("{} ", s));
            println!();
            Ok(())
        }
        Commands::Inv(args) => {
            let lines = read_lines(&args.file)?;
            let arr = lines[1]
                .split(' ')
                .flat_map(|s| s.parse::<i32>())
                .collect::<Vec<_>>();
            println!("{}", inversion_count(&arr));
            Ok(())
        }
        Commands::TwoSum(args) => {
            let lines = read_lines(&args.file)?[1..]
                .iter()
                .map(|l| {
                    l.split(' ')
                        .flat_map(|s| s.parse::<i32>())
                        .collect::<Vec<_>>()
                })
                .map(|v| two_sum(&v))
                .collect::<Vec<_>>();
            lines.iter().for_each(|v| {
                v.iter().for_each(|a| print!("{} ", a));
                println!();
            });
            Ok(())
        }
    }
}
