use roll::RollSequence;


pub struct DwPhrase {
    phrase: Vec<String>,
    word_cnt: usize,
    // seperator: char
}

impl DwPhrase {
    pub fn new(word_cnt: usize) -> DwPhrase {
        DwPhrase {phrase: Vec::new(), word_cnt }
    }


    pub fn add_word(&mut self, word: &str) {
        self.phrase.push(word.to_string());
    }
  

    pub fn as_string(&self) -> Option<String> {
        match self.phrase.is_empty() {
            true => None,
            _    => {
               let mut ph = self.phrase
                            .iter()
                            .map(|s| format!("{} ", s))
                            .collect();
                Some(ph) }
        }
    }
    // TODO: Maybe refactor this and the struct
    pub fn get_word_count(&self) -> usize {
        self.word_cnt
    } 

    pub fn clear_phrase(&mut self) {
        self.phrase.clear()
    }
    
} // DwPhrase 


pub mod util {
    use roll::RollSequence;
    // Lookup table to speedup searches
    const LINE_SKIPS: [usize;6] = [0, 1296, 2592, 3888, 5184, 6481];

    pub fn extract_word(word_list: &str, roll_seq: &RollSequence) -> String {
        let word: String = word_list.lines().skip(LINE_SKIPS[roll_seq.first_index()])
                                    .filter(|s| s.contains(&roll_seq.as_string()))
                                    .map(|w| w.split_whitespace())
                                    // Each line is like: 12345 aword   so we split on space, skip to the word and unwrap it
                                    .map(|w| w.skip(1).next().unwrap())
                                    .collect();
        word
    }

}