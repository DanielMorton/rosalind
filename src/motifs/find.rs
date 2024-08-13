use regex::Regex;
use reqwest::blocking::Client;

pub(crate) fn find_motifs(client: &Client, names: &[String], re: &Regex) {
    names.iter().for_each(|name| {
        let matches = protein_motif(client, name, re);
        if !matches.is_empty() {
            println!("{}", name);
            matches.iter().for_each(|m| print!("{} ", m + 1));
            println!();
        }
    })
}

fn protein_motif(client: &Client, name: &str, re: &Regex) -> Vec<usize> {
    let code = name.split('_').next().unwrap();
    match client
        .get(format!("https://rest.uniprot.org/uniprotkb/{}.fasta", code))
        .send()
        .and_then(|r| r.text())
        .map(|t| {
            let mut split = t.split('\n');
            let title = split.next().unwrap().to_owned();
            (title, split.collect::<String>())
        })
        .map(|(_, protein)| {
            let mut start = 0;
            let mut matches = Vec::new();
            let mut p = &protein[start..];
            loop {
                match re.find(p) {
                    Some(m) => {
                        let s = m.start();
                        matches.push(start + s);
                        start = start + s + 1;
                        p = &protein[start..];
                    }
                    None => {
                        break;
                    }
                }
            }
            matches
        }) {
        Ok(m) => m,
        Err(e) => panic!("{:?}", e),
    }
}
