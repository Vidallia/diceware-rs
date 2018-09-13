#[macro_use]
extern crate clap;
use clap::App;

pub mod roll;
pub mod phrase;
pub mod cli;

use roll::RollSequence;
use phrase::{DwPhrase, util};

fn main() {
    let file = include_str!("dw-list.txt");
    let yaml = load_yaml!("config/cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    

    let mut roll_seq = RollSequence::new();
    let mut phrase = DwPhrase::new(matches.value_of("AMOUNT")
                                        .unwrap()
                                        .parse::<usize>()
                                        .unwrap());

    let phrases_generated = if matches.is_present("number") {
        matches.value_of("number").unwrap().parse::<usize>().unwrap()
    } else {
        1
    };

    println!("Generated Phrase(s):");
    for _ in 0..phrases_generated {
        for _ in 0..phrase.get_word_count() {
        // Find the word that matches the rolled numbers
            let word = util::extract_word(file);
            phrase.add_word(word);
            roll_seq.gen_new_sequence();
        }
        println!("{}", phrase.as_string().unwrap());
        phrase.clear_phrase();
    }
}
