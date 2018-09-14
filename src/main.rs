#[macro_use]
extern crate clap;
use clap::App;

pub mod roll;
pub mod phrase;
pub mod writef;

use roll::RollSequence;
use phrase::{DwPhrase, util};
use writef::FileOut;

fn main() {
    let file = include_str!("dw-list.txt");
    let yaml = load_yaml!("config/cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut roll_seq = RollSequence::new();
    let mut phrase = DwPhrase::new(matches.value_of("length")
                                        .unwrap_or("1")
                                        .parse::<usize>()
                                        .unwrap());

    let phrases_generated =  matches.value_of("AMOUNT").unwrap().parse::<usize>().unwrap();
    let mut cache = Vec::new();
    
    println!("Generated Phrase(s):\n");
    for _ in 0..phrases_generated {
        for _ in 0..phrase.get_word_count() {
            let word = util::extract_word(file, &roll_seq);
            phrase.add_word(&word);
            roll_seq.gen_new_sequence();
        }
        if let Some(p) = phrase.as_string() {
            if matches.is_present("file") {
                cache.push(p);
            } else {
                println!("{}", p);
            }
        } else {
            break
        }
        phrase.clear_phrase();
    }
    if let Some(fp) = matches.value_of("file") {
        let out = FileOut::new(fp.to_string(), cache);
        out.write(&phrase)
    }
}
