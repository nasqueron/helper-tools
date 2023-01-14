use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::process::exit;

use enontekio::parser;

fn main() {
    if !Path::new("query.csvv").is_file() {
        println!("Run fantoir-datasource wikidata and save the stderr output to wikidata-import.log file.");
        exit(1);
    }

    let choices = parse_import_log("wikidata-import.log");

    if !Path::new("query.csv").is_file() {
        println!("Let's fetch the labels of the items we've to import.");
        println!("Run this SPARQL query and save result to query.csv (without the first line):");
        println!();
        print_choices_query(&choices);
        exit(2);
    }

    print_choices_conflict_resolution(&choices);
}

/////////// Tasks

fn print_choices_query (choices: &HashSet<Vec<String>>) {
    let terms: HashSet<_> = choices
        .iter()
        .flat_map(|choice| choice)
        .map(|item| format!("wd:{}", item))
        .collect();

    println!("
SELECT DISTINCT ?item ?itemLabel
WHERE
{{
  VALUES ?item {{");

    for term in terms {
        println!("    {}", term);
    }

    println!(r#"  }}
  SERVICE wikibase:label {{ bd:serviceParam wikibase:language "fr" . }}
}}
    "#);
}

fn print_choices_conflict_resolution (choices: &HashSet<Vec<String>>) {
    let terms = parse_terms_file("query.csv");

    for choice in choices {
        for item in choice {
            let spaces = " ".repeat(12 - item.len());
            println!("        \"{}\",{}// {}", &item, spaces, terms[item])
        }
        println!();
    }
}

/////// Parsers

fn parse_import_log(filename: &str) -> HashSet<Vec<String>> {
    let choices: Vec<_> = parser::parse_file_by_line(filename,
        |line| parse_line(&line.unwrap())
    ).expect("Can't read Wikidata import log");

    choices
        .into_iter()
        .filter(|choice| choice.is_some())
        .map(|choice| choice.unwrap())
        .collect()
}

fn parse_line (expression: &str) -> Option<Vec<String>> {
    if !expression.starts_with("Can't determine P31 winner amongst ") {
        return None;
    }

    let pos = expression.find("]")
        .expect("Expression should contain ] to close the list.");

    let mut choices: Vec<_> = expression[36..pos]
        .split(", ")
        .map(|item| item.replace("\"", ""))
        .collect();

    choices.sort();

    Some(choices)
}

fn parse_terms_file(filename: &str) -> HashMap<String, String> {
    parser::parse_file_by_line(filename, |line| {
        let line = line.unwrap();
        let parts: Vec<_> = line.split(",").collect();

        (parts[0].replace("http://www.wikidata.org/entity/", ""), parts[1].to_string())
    }).unwrap()
}
