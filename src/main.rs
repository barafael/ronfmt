#![allow(clippy::redundant_closure)] // blame pest

mod ast;

use clap::clap_app;
use pest::Parser;
use pest_derive::*;

static mut TAB_SIZE: usize = 4;
static mut MAX_LINE_WIDTH: usize = 40;

#[derive(Parser)]
#[grammar = "ron.pest"]
struct RonParser;

fn main() {
    let matches = matches();

    let target_path = matches.value_of("INPUT").unwrap();

    if let Some(mlw) = matches.value_of("MAX_LINE_WIDTH") {
        unsafe { MAX_LINE_WIDTH = str::parse(mlw).unwrap() }
    }
    if let Some(ts) = matches.value_of("TAB_SIZE") {
        unsafe { TAB_SIZE = str::parse(ts).unwrap() }
    }

    let file = std::fs::read_to_string(&target_path).expect("unable to read file");

    let ron = RonParser::parse(Rule::ron_file, &file)
        .expect("unable to parse RON")
        .next()
        .unwrap();

    if matches.is_present("debug") {
        println!("{}", ast::RonFile::parse_from(ron));
    } else {
        std::fs::copy(&target_path, format!("{}.bak", &target_path))
            .expect("unable to create backup file");

        std::fs::write(&target_path, format!("{}", ast::RonFile::parse_from(ron)))
            .expect("unable to overwrite target file");
    }
}

fn matches<'a>() -> clap::ArgMatches<'a> {
    clap_app!(app =>
        (version: "0.1.0")
        (author: "Anton F. <a.filippov@protonmail.com>")
        (about: "Utility for autoformatting RON files.")
        (@arg INPUT: +required "Sets which file to format")
        (@arg MAX_LINE_WIDTH: -w +takes_value "Sets soft max line width for formatting heuristics")
        (@arg TAB_SIZE: -t +takes_value "Sets indentation size in spaces")
        (@arg debug: -d "Prints output to console instead of overwriting the input file")
    ).get_matches()
}
