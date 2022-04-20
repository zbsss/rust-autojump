use std::collections::HashSet;
use std::fs::{OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::exit;
use clap::{Arg, Command};
use std::io::prelude::*;
use rayon::prelude::*;

const PARTIAL_DATABASE_PATH: &str = ".rustautojump/database.txt";

fn get_database_path(filename: &str) -> PathBuf {
    let home_path = home::home_dir().unwrap();
    let relative_file_path = std::path::Path::new(filename);
    home_path.join(relative_file_path)
}

fn load_database() -> Vec<String> {
    let database_path = get_database_path(PARTIAL_DATABASE_PATH);

    let database_folder = database_path.parent().unwrap();
    if !database_folder.exists() {
        std::fs::create_dir_all(database_folder).unwrap();
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(database_path);

    BufReader::new(file.unwrap())
    .lines()
    .map(|line| line.unwrap())
    .collect()
}

fn append_database(line: String) {
    let database_path = get_database_path(PARTIAL_DATABASE_PATH);

    let path_set: HashSet<String> = HashSet::from_iter(load_database());

    if !path_set.contains(&line) {
        let mut file = OpenOptions::new()
            .append(true)
            .open(database_path)
            .unwrap();
        file.write((line + "\n").as_bytes()).unwrap();
    }
}

fn get_ngrams(string: &String, n: usize) -> Vec<String> {
    let lower = string.to_lowercase();
    let size = lower.chars().count() - n + 1;

    let mut ngrams = vec![];
    for i in 0..size {
        let ngram = lower[i..i+n].to_string();
        ngrams.push(ngram);
    }
    return ngrams;
}

fn string_similarity(a: &String, b: &String, n: usize) -> f64 {
    let a_ngrams = get_ngrams(&a, n);
    let b_ngrams = get_ngrams(&b, n);

    let mut hit_count = 0;
    let total_length = a_ngrams.len() + b_ngrams.len();

    for a_ngram in a_ngrams.iter() {
        for b_ngram in b_ngrams.iter() {
            if a_ngram == b_ngram {
                hit_count += 1;
                break;
            }
        }
    }
    return (2.0 * hit_count as f64) / total_length as f64;
}

fn find_matches(search_phrase: String) -> Vec<(f64, String)> {
    let n: usize = if search_phrase.contains('/') || search_phrase.chars().count() == 2 { 2 } else { 3 };

    let lines = load_database();

    let mut scores: Vec<(f64, String)>  = lines.par_iter().map(|line| {
        (string_similarity(&line, &search_phrase, n), line.clone())
    }).collect();
    scores.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    return scores;
}

fn find_best_match(search_phrase: String) -> Option<String> {
    match find_matches(search_phrase).last() {
        Some(best_match) => Some((&best_match).1.to_string()),
        None => None
    }
}

fn error(error: &str) {
    eprintln!("{}", error);
    exit(1);
}

fn main() {
    let matches = Command::new("rust-autojump")
        .version("0.1.0")
        .author("Michał Kurleto")
        .about("Basic Rust implementation of autojump")
        .arg(Arg::new("search_phrase")
                .short('s')
                .long("search")
                .takes_value(true)
                .help("TODO:"))
        .arg(Arg::new("add")
                .short('a')
                .long("add")
                .takes_value(true)
                .help("TODO:"))
        .get_matches();

    if matches.is_present("add") {
        let path_to_add = matches.value_of("add").unwrap().to_string();
        append_database(path_to_add);
    } else {
        match matches.value_of("search_phrase") {
            Some(search_phrase) => 
                match find_best_match(search_phrase.to_string()) {
                    Some(best_path) => println!("{}", best_path),
                    None => error("No paths found")
            },
            None => error("Invalid search phrase")
        };
    }
}
