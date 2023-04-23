
use clap::ValueEnum;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, ValueEnum)]
pub enum Rating {
    Awful,
    Bad,
    Neutral,
    Good,
    Great,
}

impl From<Rating> for u8 {
    fn from(value: Rating) -> Self {
        match value {
            Rating::Awful => 1,
            Rating::Bad => 2,
            Rating::Neutral => 3,
            Rating::Good => 4,
            Rating::Great => 5,
        }
    }
}

impl Distribution<Rating> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Rating {
        match rng.gen_range(0..=4) {
            0 => Rating::Awful,
            1 => Rating::Bad,
            2 => Rating::Neutral,
            3 => Rating::Good,
            _ => Rating::Great,
        }
    }
}
