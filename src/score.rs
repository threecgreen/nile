use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
use wasm_bindgen::prelude::*;

/// For scoring, it's valuable to keep display penalties (negative scores)
/// separate for tile and bonus scores. This is also how it's handled on the
/// original paper score pads.
///
/// Both `add` and `sub` should be >= 0, unless the `TurnScore` instance is
/// part of an undo or revert option like removing a tile.
#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct TurnScore {
    pub add: i16,
    pub sub: i16,
}

impl TurnScore {
    pub fn score(&self) -> i16 {
        self.add - self.sub
    }
}

impl From<i16> for TurnScore {
    fn from(i: i16) -> Self {
        if i >= 0 {
            Self { add: i, sub: 0 }
        } else {
            Self { add: 0, sub: -i }
        }
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

impl Mul<i16> for TurnScore {
    type Output = TurnScore;

    fn mul(self, rhs: i16) -> Self::Output {
        Self {
            add: self.add * rhs,
            sub: self.sub * rhs,
        }
    }
}

impl Div<i16> for TurnScore {
    type Output = TurnScore;

    fn div(self, rhs: i16) -> Self::Output {
        Self {
            add: self.add / rhs,
            sub: self.sub / rhs,
        }
    }
}

impl Neg for TurnScore {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            add: -self.add,
            sub: -self.sub,
        }
    }
}

impl Sub for TurnScore {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            add: self.add - rhs.add,
            sub: self.sub - rhs.sub,
        }
    }
}

impl PartialOrd for TurnScore {
    fn partial_cmp(&self, other: &TurnScore) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TurnScore {
    fn cmp(&self, other: &TurnScore) -> std::cmp::Ordering {
        self.score().cmp(&other.score())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn score() {
        assert_eq!(TurnScore::default().score(), 0);
        assert_eq!(TurnScore { add: 86, sub: 14 }.score(), 72);
        assert_eq!(TurnScore { add: 48, sub: 96 }.score(), -48);
    }

    #[test]
    fn add() {
        let target = TurnScore { add: 100, sub: 140 };
        assert_eq!(
            target + TurnScore { add: 100, sub: 60 },
            TurnScore { add: 200, sub: 200 }
        );
    }

    #[test]
    fn add_id() {
        let target = TurnScore { add: 20, sub: 15 };
        let zero = TurnScore::default();
        assert_eq!(target + zero, target);
    }

    #[test]
    fn add_associative() {
        let target1 = TurnScore { add: 10, sub: 5 };
        let target2 = TurnScore { add: 4, sub: 0 };
        let target3 = TurnScore { add: -13, sub: -4 };
        assert_eq!((target1 + target2) + target3, target1 + (target2 + target3));
    }

    #[test]
    fn add_commutative() {
        let target1 = TurnScore { add: 40, sub: 20 };
        let target2 = TurnScore { add: 10, sub: 30 };
        assert_eq!(target1 + target2, target2 + target1);
    }

    #[test]
    fn sub() {
        let target = TurnScore { add: 40, sub: 48 };
        assert_eq!(
            target - TurnScore { add: 16, sub: 32 },
            TurnScore { add: 24, sub: 16 }
        );
    }

    #[test]
    fn neg_undoes() {
        let target = TurnScore { add: 100, sub: 48 };
        assert_eq!(target + (-target), TurnScore::default());
    }

    #[test]
    fn add_assign() {
        let target = TurnScore { add: 40, sub: 8 };
        let mut copy = target.clone();
        let rhs = TurnScore { add: 16, sub: 0 };
        copy += rhs;
        assert_eq!(copy, target + rhs);
    }

    // Need custom ordering implementation. This is especially important
    // to the AIs for choosing the correct move
    #[test]
    fn negative_scores_sorted_less_than_positive() {
        let left = TurnScore::from(40);
        let right = TurnScore::from(-80);
        assert_eq!(left.max(right), left);
    }
}
