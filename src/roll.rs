extern crate rand;
use roll::rand::prelude::*;

const MAX_SEQUENCE_LEN: usize = 5;

pub struct RollSequence {
    first: u32,
    sequence: [u32; MAX_SEQUENCE_LEN],
    rng: StdRng
}

impl RollSequence {
    pub fn new() -> RollSequence {
        let mut rng = StdRng::from_entropy();
        let seq: [u32; MAX_SEQUENCE_LEN] = RollSequence::create_sequence(&mut rng);
        RollSequence {first: seq[0], sequence: seq, rng }
    }

    fn roll(randg: &mut StdRng) -> u32 {
        let val = randg.gen_range(1,7);
        val
    }

    fn create_sequence(randg: &mut StdRng) -> [u32; MAX_SEQUENCE_LEN] {
        let mut seq: [u32; MAX_SEQUENCE_LEN] = [0; MAX_SEQUENCE_LEN];
        for n in 0..MAX_SEQUENCE_LEN {
            seq[n] = RollSequence::roll(randg);
        }
        seq
    }

    pub fn gen_new_sequence(&mut self) {
        self.first = Self::roll(&mut self.rng);
        self.sequence[0] = self.first;
        for n in 1..MAX_SEQUENCE_LEN {
            self.sequence[n] = Self::roll(&mut self.rng);
        }
    }

    pub fn as_string(&self) -> String {
        let seq: String = self.sequence
                              .iter()
                              .map(|&n| n.to_string())
                              .collect();
        seq
    }

    pub fn first_index(&self) -> usize {
        (self.first - 1) as usize
    }
}
