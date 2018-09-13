pub mod roll;
pub mod phrase;

use roll::RollSequence;
use phrase::{DwPhrase, util};

fn main() {
// Look into Macro std::include_str to solve this problem by including the file with the build process
    let file = include_str!("dw-list.txt");
    let mut roll_seq = RollSequence::new();
    let mut phrase = DwPhrase::new(3);
    for _ in 0..3 {
     // Find the word that matches the rolled numbers
        let word = util::extract_word(file);
        phrase.add_word(word);
        roll_seq.gen_new_sequence();
    }
    println!("Phrase: {}", phrase.as_string().unwrap());
}
