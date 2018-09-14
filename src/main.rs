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
    
    let fpath = matches.value_of("file").unwrap_or("out.txt");
    let mut fout = FileOut::new(String::from(fpath), String::from(""));
      

    println!("Generated Phrase(s):\n");
    for _ in 0..phrases_generated {
        for _ in 0..phrase.get_word_count() {
            let word = util::extract_word(file, &roll_seq);
            phrase.add_word(&word);
            fout.append_str(&word);
            fout.append_str(" ");
            roll_seq.gen_new_sequence();
        }
        fout.append_str("\n");
        if let Some(p) = phrase.as_string() {
            println!("{}", p)
        } else {
            break
        }
        phrase.clear_phrase();
    }
    fout.write()
}
