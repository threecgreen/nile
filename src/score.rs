use std::ops::{Add, AddAssign, Neg, Sub};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, Default)]
#[cfg_attr(test, derive(PartialEq, Eq))]
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
    /// Both `add` and `sub` should be >= 0, unless the `TurnScore` instance is
    /// part of an undo or revert option like removing a tile.
    pub fn new(add: i16, sub: i16) -> TurnScore {
        Self { add, sub }
    }
}

impl TurnScore {
    pub fn score(&self) -> i16 {
        self.add - self.sub
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn score() {
        assert_eq!(TurnScore::default().score(), 0);
        assert_eq!(TurnScore::new(86, 14).score(), 72);
        assert_eq!(TurnScore::new(48, 96).score(), -48);
    }

    #[test]
    fn add() {
        let target = TurnScore::new(100, 140);
        assert_eq!(target + TurnScore::new(100, 60), TurnScore::new(200, 200));
    }

    #[test]
    fn add_id() {
        let target = TurnScore::new(20, 15);
        let zero = TurnScore::default();
        assert_eq!(target + zero, target);
    }

    #[test]
    fn add_associative() {
        let target1 = TurnScore::new(10, 5);
        let target2 = TurnScore::new(4, 0);
        let target3 = TurnScore::new(-13, -4);
        assert_eq!((target1 + target2) + target3, target1 + (target2 + target3));
    }

    #[test]
    fn add_commutative() {
        let target1 = TurnScore::new(40, 20);
        let target2 = TurnScore::new(10, 30);
        assert_eq!(target1 + target2, target2 + target1);
    }

    #[test]
    fn sub() {
        let target = TurnScore::new(40, 48);
        assert_eq!(target - TurnScore::new(16, 32), TurnScore::new(24, 16));
    }

    #[test]
    fn neg_undoes() {
        let target = TurnScore::new(100, 48);
        assert_eq!(target + (-target), TurnScore::default());
    }

    #[test]
    fn add_assign() {
        let target = TurnScore::new(40, 8);
        let mut copy = target.clone();
        let rhs = TurnScore::new(16, 0);
        copy += rhs;
        assert_eq!(copy, target + rhs);
    }
}
