use std::ops::{Add, AddAssign, Neg};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Default)]
pub struct TurnScore {
    add: i16,
    sub: i16,
}

#[wasm_bindgen]
impl TurnScore {
    pub fn add(&self) -> i16 {
        self.add
    }

    pub fn sub(&self) -> i16 {
        self.sub
    }
}

impl TurnScore {
    pub fn new(add: i16, sub: i16) -> TurnScore {
        // if add < 0 {
        //     Err("`add` must be greater than or equal to zero".to_owned())
        // } else if sub > 0 {
        //     Err("`sub` must be less than or equal to zero".to_owned())
        // }
        Self { add, sub }
    }
}

impl TurnScore {
    pub fn score(&self) -> i16 {
        self.add + self.sub
    }
}

impl Add for TurnScore {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            add: self.add + rhs.add,
            sub: self.sub + rhs.sub,
        }
    }
}

impl AddAssign for TurnScore {
    fn add_assign(&mut self, rhs: Self) {
        self.add += rhs.add;
        self.sub += rhs.sub;
    }
}

impl Neg for TurnScore {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            add: -self.sub,
            sub: -self.add,
        }
    }
}

pub fn sum_scores(scores: &Vec<TurnScore>) -> TurnScore {
    scores
        .iter()
        .fold(TurnScore::default(), |acc, ts| acc + *ts)
}
