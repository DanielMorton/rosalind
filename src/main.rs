mod dna;
mod fasta;
mod fibonacci;
mod gc;
mod gene;
mod graph;
mod mendel;
mod motifs;
mod parse;
mod profile;
mod protein;
mod util;

use crate::dna::rna_nucleotide_count;
use crate::fasta::{pairs, read_fasta, transition_transversion_ratio, FASTA};
use crate::gene::{longest_decreasing_sequence, longest_increasing_sequence};
use crate::graph::{
    align, catalan_number, edges_to_degrees, inner_nodes, read_edges, tree_edge_fill,
};
use crate::mendel::{dna_prob, factorial, npr, permutation_list, permute};
use crate::motifs::{
    build_failure_array, find_motifs, get_subsequence, kmer_count, lcs, make_dictionary,
    reverse_palindrome,
};
use crate::profile::find_consensus;
use crate::protein::{find_orfs, rna_splice};
use crate::util::{binary_search, merge, merge_sort, read_lines, read_num_list, read_string, read_two_line, read_vec, DNA, inversion_count};
use dna::{dna_nucleotide_count, dna_to_rna, reverse_complement};
use fibonacci::k_fibonacci;
use gc::gc_max;
use mendel::{expected_offspring, first_law, second_law};
use motifs::hamming_distance;
use motifs::motif_start;
use parse::RosalindParse;
use protein::rna_to_protein;
use protein::{protein_mass, rna_count};
use regex::Regex;
use reqwest::blocking::Client;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;

fn main() {
    let matches = parse::parse();
    let file_type = matches.get_file_type();
    let file = matches.get_file();

    if file_type == "dna" || file_type == "ini" {
        let dna = read_string(&file);
        dna_nucleotide_count(&dna)
            .iter()
            .for_each(|n| print!("{} ", n));
        println!();
    } else if file_type == "rna" {
        let dna = read_string(&file);
        let rna = dna_to_rna(&dna);
        println!("{}", rna)
    } else if file_type == "revc" {
        let dna = read_string(&file);
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
        let rna = read_string(&file);
        let protein = rna_to_protein(&rna);
        println!("{protein}")
    } else if file_type == "subs" {
        let (dna, motif) = read_two_line(&file);
        motif_start(&dna, &motif)
            .iter()
            .for_each(|p| print!("{p} "));
        println!();
    } else if file_type == "hamm" {
        let (s, t) = read_two_line(&file);
        println!("{}", hamming_distance(&s, &t))
    } else if file_type == "iev" {
        let nums = read_num_list::<u32>(&file, ' ');
        println!("{}", expected_offspring(&nums))
    } else if file_type == "mrna" {
        let protein = read_string(&file);
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
        let protein = read_string(&file);
        println!("{}", protein_mass(&protein))
    } else if file_type == "grph" {
        let edges = pairs(&file, 3);
        edges.iter().for_each(|(f1, f2)| println!("{} {}", f1, f2))
    } else if file_type == "mprt" {
        let names = read_vec(&file, '\n');
        let client = Client::new();
        let re = Regex::new("N[^P](S|T)[^P]").unwrap();
        find_motifs(&client, &names, &re)
    } else if file_type == "cons" {
        let dna_list = read_fasta(&file);
        println!("{}", find_consensus(&dna_list))
    } else if file_type == "orf" {
        let fasta = read_fasta(&file);
        let dna = fasta[0].text.clone();
        find_orfs(&dna)
            .iter()
            .collect::<HashSet<_>>()
            .iter()
            .for_each(|orf| println!("{orf}"))
    } else if file_type == "splc" {
        let fasta = read_fasta(&file);
        let dna = fasta[0].text.clone();
        let introns = fasta[1..]
            .iter()
            .map(|f| f.text.clone())
            .collect::<Vec<_>>();
        println!("{}", rna_splice(&dna, &introns))
    } else if file_type == "lcsm" {
        let fasta = read_fasta(&file);
        println!("{}", lcs(&fasta))
    } else if file_type == "perm" {
        let n = match fs::read_to_string(file).map(|f| f.trim().parse::<usize>().unwrap()) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
        println!("{}", factorial(n));
        permute(n).into_iter().for_each(|v| {
            v.into_iter().for_each(|p| print!("{} ", p));
            println!()
        })
    } else if file_type == "tree" {
        let (n, edges) = match fs::read_to_string(file).map(|s| {
            let mut split = s.trim().split('\n');
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
        }) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
        println!("{}", tree_edge_fill(n, &edges))
    } else if file_type == "inod" {
        let n = match fs::read_to_string(file).map(|f| f.trim().parse::<usize>().unwrap()) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
        println!("{}", inner_nodes(n))
    } else if file_type == "pper" {
        let (n, r) = match fs::read_to_string(file).map(|s| {
            let mut split = s.trim().split(' ');
            (
                split.next().unwrap().parse::<u64>().unwrap(),
                split.next().unwrap().parse::<u64>().unwrap(),
            )
        }) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
        println!("{}", npr(n, r))
    } else if file_type == "prob" {
        let (dna, probs) = match fs::read_to_string(file).map(|s| {
            let mut split = s.trim().split('\n');
            let dna = split.next().unwrap().to_owned();
            let probs = split
                .next()
                .unwrap()
                .split(' ')
                .map(|f| f.parse::<f64>().unwrap())
                .collect::<Vec<_>>();
            (dna, probs)
        }) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };
        probs
            .into_iter()
            .for_each(|p| print!("{} ", dna_prob(&dna, p)));
        println!()
    } else if file_type == "revp" {
        let fasta = FASTA::read_file(&file);
        let palindromes = reverse_palindrome(&fasta, 4, 12);
        palindromes
            .iter()
            .for_each(|(s, e)| println!("{} {}", s, e))
    } else if file_type == "pmch" {
        let fasta = read_fasta(&file)[0].clone();
        let count = rna_nucleotide_count(&fasta.text);
        println!("{:?}", count);
        println!("{}", factorial(count[0]) * factorial(count[1]))
    } else if file_type == "mmch" {
        let fasta = read_fasta(&file)[0].clone();
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
        )
    } else if file_type == "pdst" {
        let fasta = read_fasta(&file);
        fasta.iter().for_each(|f1| {
            fasta
                .iter()
                .map(|f2| hamming_distance(&f1.text, &f2.text))
                .for_each(|d| print!("{} ", (d as f64) / (f1.text.len() as f64)));
            println!()
        })
    } else if file_type == "long" {
        let fasta = read_fasta(&file);
        let alignment = align(&fasta);
        println!("{}", alignment);
    } else if file_type == "sseq" {
        let fasta = read_fasta(&file);
        get_subsequence(&fasta[0], &fasta[1])
            .iter()
            .for_each(|s| print!("{} ", s));
        println!()
    } else if file_type == "tran" {
        let fasta = read_fasta(&file);
        let tt = transition_transversion_ratio(&fasta[0], &fasta[1]);
        println!("{}", tt);
    } else if file_type == "lexf" {
        let (letters, n) = fs::read_to_string(file)
            .map(|s| {
                let mut split = s.trim().split('\n');
                (
                    split
                        .next()
                        .unwrap()
                        .split(' ')
                        .map(|c| c.to_owned())
                        .collect::<Vec<_>>(),
                    split.next().unwrap().parse::<usize>().unwrap(),
                )
            })
            .unwrap();
        make_dictionary(&letters.iter().map(|l| l.as_str()).collect::<Vec<_>>(), n)
            .iter()
            .for_each(|d| println!("{}", d));
    } else if file_type == "lgis" {
        let nums = fs::read_to_string(file)
            .iter()
            .flat_map(|s| {
                let mut split = s.trim().split('\n');
                let _ = split.next();
                let line = split.collect::<String>();
                line.split(' ')
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let increase = longest_increasing_sequence(&nums);
        increase.iter().for_each(|n| print!("{} ", n));
        println!();
        let decrease = longest_decreasing_sequence(&nums);
        decrease.iter().for_each(|n| print!("{} ", n));
        println!()
    } else if file_type == "sign" {
        let n = fs::read_to_string(file)
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap();
        let p_list = permutation_list(n);
        println!("{}", p_list.len());
        p_list.iter().for_each(|v| {
            v.iter().for_each(|p| print!("{} ", p));
            println!();
        })
    } else if file_type == "cat" {
        let fasta = FASTA::read_file(&file);
        println!("{}", catalan_number(&fasta))
    } else if file_type == "kmp" {
        let fasta = read_fasta(&file);
        build_failure_array(&fasta[0].text.chars().collect::<Vec<_>>())
            .iter()
            .for_each(|n| print!("{} ", n));
        println!();
    } else if file_type == "kmer" {
        let fasta = FASTA::read_file(&file);
        let dna_dict = make_dictionary(DNA, 4);
        let counts = kmer_count(&fasta.text, 4);
        dna_dict
            .iter()
            .for_each(|d| print!("{} ", counts.get(d).unwrap_or(&0)));
        println!()
    } else if file_type == "deg" {
        let edges = read_edges(&file);
        let degrees = edges_to_degrees(&edges);
        degrees.iter().for_each(|d| print!("{} ", d));
        println!()
    } else if file_type == "bins" {
        let text = read_string(&file);
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
        println!()
    } else if file_type == "mer" {
        let text = read_lines(&file);
        let lines = text
            .iter()
            .enumerate()
            .filter(|(i, s)| i % 2 == 1)
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
    } else if file_type == "ms" {
        let lines = read_lines(&file);
        let arr = lines[1]
            .split(' ')
            .flat_map(|s| s.parse::<i32>())
            .collect::<Vec<_>>();
        merge_sort(&arr).iter().for_each(|s| print!("{} ", s));
        println!()
    } else if file_type == "inv" {
        let lines = read_lines(&file);
        let arr = lines[1]
            .split(' ')
            .flat_map(|s| s.parse::<i32>())
            .collect::<Vec<_>>();
        println!("{}", inversion_count(&arr))
    }
}
