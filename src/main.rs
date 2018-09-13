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
    let mut phrase = DwPhrase::new(matches.value_of("length")
                                        .unwrap()
                                        .parse::<usize>()
                                        .unwrap());

    let phrases_generated =  matches.value_of("AMOUNT").unwrap().parse::<usize>().unwrap();
  

    println!("Generated Phrase(s):");
    for _ in 0..phrases_generated {
        for _ in 0..phrase.get_word_count() {
            let word = util::extract_word(file, &roll_seq);
            phrase.add_word(word);
            roll_seq.gen_new_sequence();
        }
        println!("{}", phrase.as_string().unwrap());
        phrase.clear_phrase();
    }
}
