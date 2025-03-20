use std::{
    ops::{BitAnd, BitOr, BitXor, Not, BitOrAssign, BitAndAssign, BitXorAssign},
    cmp::{PartialEq, Eq}
};

#[derive(Clone)]
pub struct Bitset {
    data: Vec<u32>,
    len: usize
}

impl Bitset {
    pub fn new(n: usize) -> Bitset {
        let len = (n + 31) / 32;
        Bitset {
            data: vec![0; len],
            len: n
        }
    }

    pub fn set(&mut self, index: usize) {
        assert!(index < self.len, "Index out of Bounds");
        let word_index = index / 32;
        let bit_index = index % 32;
        self.data[word_index] |= 1 << bit_index;
    }

    pub fn reset(&mut self, index: usize) {
        assert!(index < self.len, "Index out of Bounds");
        let word_index = index / 32;
        let bit_index = index % 32;
        self.data[word_index] &= !(1 << bit_index);
    }

    pub fn flip(&mut self, index: usize) {
        assert!(index < self.len, "Index out of Bounds");
        let word_index = index / 32 ;
        let bit_index = index % 32;
        self.data[word_index] ^= 1 << bit_index;
    }

    pub fn test(&self, index: usize) -> bool {
        assert!(index < self.len, "Index out of Bounds");
        let word_index = index / 32;
        let bit_index = index % 32;
        (self.data[word_index] & (1 << bit_index)) != 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn to_string(&self) -> String {
        (0..self.len)
            .rev()
            .map(|index| {
                if self.test(index) {'1'} else {'0'}
            })
            .collect()
    }
}

// operations
impl BitAnd for Bitset {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        assert_eq!(self.len, rhs.len, "Bitset Lengths must Match"); 

        let data = self.data
            .into_iter()
            .zip(rhs.data.into_iter())
            .map(|(a, b)| a & b)
            .collect();
        Self {
            data,
            len: self.len
        }
    }
}

impl BitOr for Bitset {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        assert_eq!(self.len, rhs.len, "Bitset Lengths must Match"); 

        let data = self.data
            .into_iter()
            .zip(rhs.data.into_iter())
            .map(|(a, b)| a | b)
            .collect();
        Self {
            data,
            len: self.len
        }
    }
}

impl BitXor for Bitset {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        assert_eq!(self.len, rhs.len, "Bitset Lengths must Match"); 

        let data = self.data
            .into_iter()
            .zip(rhs.data.into_iter())
            .map(|(a, b)| a ^ b)
            .collect();
        Self {
            data,
            len: self.len
        }
    }
}

impl Not for Bitset {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        // invert all bits
        self.data.iter_mut().for_each(|a| *a = !*a);
        self
    }
}

impl BitAndAssign for Bitset {
    
    fn bitand_assign(&mut self, rhs: Self) {
        assert_eq!(self.len, rhs.len, "Bitset Lengths must Match");
        for (a, b) in self.data.iter_mut().zip(rhs.data) {
            *a &= b
        }
    }
}

impl BitOrAssign for Bitset {
    fn bitor_assign(&mut self, rhs: Self) {
        assert_eq!(self.len, rhs.len, "Bitset Lengths must Match");
        for (a, b) in self.data.iter_mut().zip(rhs.data) {
            *a |= b
        }
    }
}

impl BitXorAssign for Bitset {
    fn bitxor_assign(&mut self, rhs: Self) {
        assert_eq!(self.len, rhs.len, "Bitset Lengths must Match");
        for (a, b) in self.data.iter_mut().zip(rhs.data) {
            *a ^= b
        }
    }
}

impl PartialEq for Bitset {
    fn eq(&self, other: &Self) -> bool {
        self.len == self.len && self.data == other.data
    }
}

impl Eq for Bitset {}

#[cfg(test)]
mod test {
    use crate::bitset::Bitset;

    #[test]
    fn test_set_reset_flip_and_test() {
        let mut bs = Bitset::new(64);
        // Initially, all bits should be false.
        for i in 0..bs.len() {
            assert!(!bs.test(i), "Bit {} should initially be false", i);
        }

        // Test set
        bs.set(10);
        assert!(bs.test(10), "Bit 10 should be true after setting");

        // Test reset
        bs.reset(10);
        assert!(!bs.test(10), "Bit 10 should be false after resetting");

        // Test flip: flipping an unset bit should set it.
        bs.flip(20);
        assert!(bs.test(20), "Bit 20 should be true after flipping from false");
        // Flipping again should clear it.
        bs.flip(20);
        assert!(!bs.test(20), "Bit 20 should be false after flipping again");
    }

    #[test]
    fn test_bitand_operator() {
        let mut bs1 = Bitset::new(64);
        let mut bs2 = Bitset::new(64);
        bs1.set(5);
        bs1.set(10);
        bs2.set(10);
        bs2.set(15);

        let bs3 = bs1 & bs2;
        // Only bit 10 should be set (common to both).
        for i in 0..bs3.len() {
            if i == 10 {
                assert!(bs3.test(i), "Bit {} should be set in AND result", i);
            } else {
                assert!(!bs3.test(i), "Bit {} should be unset in AND result", i);
            }
        }
    }

    #[test]
    fn test_bitor_operator() {
        let mut bs1 = Bitset::new(64);
        let mut bs2 = Bitset::new(64);
        bs1.set(5);
        bs2.set(10);

        let bs3 = bs1 | bs2;
        // Bits 5 and 10 should be set.
        for i in 0..bs3.len() {
            if i == 5 || i == 10 {
                assert!(bs3.test(i), "Bit {} should be set in OR result", i);
            } else {
                assert!(!bs3.test(i), "Bit {} should be unset in OR result", i);
            }
        }
    }

    #[test]
    fn test_bitxor_operator() {
        let mut bs1 = Bitset::new(64);
        let mut bs2 = Bitset::new(64);
        bs1.set(5);
        bs1.set(10);
        bs2.set(10);
        bs2.set(15);

        let bs3 = bs1 ^ bs2;
        // Bit 10 is common, so only bits 5 and 15 should be set.
        for i in 0..bs3.len() {
            if i == 5 || i == 15 {
                assert!(bs3.test(i), "Bit {} should be set in XOR result", i);
            } else {
                assert!(!bs3.test(i), "Bit {} should be unset in XOR result", i);
            }
        }
    }

    #[test]
    fn test_not_operator() {
        let mut bs = Bitset::new(64);
        // Set a couple of bits.
        bs.set(1);
        bs.set(63);
        let bs_not = !bs;
        // In the NOT version, bits 1 and 63 should be false; all others true.
        for i in 0..bs_not.len() {
            if i == 1 || i == 63 {
                assert!(!bs_not.test(i), "Bit {} should be false in NOT result", i);
            } else {
                assert!(bs_not.test(i), "Bit {} should be true in NOT result", i);
            }
        }
    }

    #[test]
    fn test_bitand_assign_operator() {
        let mut bs1 = Bitset::new(64);
        let mut bs2 = Bitset::new(64);
        bs1.set(5);
        bs1.set(10);
        bs2.set(10);
        bs2.set(15);

        bs1 &= bs2;
        // After &=, only bit 10 should remain set.
        for i in 0..bs1.len() {
            if i == 10 {
                assert!(bs1.test(i), "Bit {} should be set after &= operation", i);
            } else {
                assert!(!bs1.test(i), "Bit {} should be unset after &= operation", i);
            }
        }
    }

    #[test]
    fn test_bitor_assign_operator() {
        let mut bs1 = Bitset::new(64);
        let mut bs2 = Bitset::new(64);
        bs1.set(5);
        bs2.set(10);

        bs1 |= bs2;
        // After |=, bits 5 and 10 should be set.
        for i in 0..bs1.len() {
            if i == 5 || i == 10 {
                assert!(bs1.test(i), "Bit {} should be set after |= operation", i);
            } else {
                assert!(!bs1.test(i), "Bit {} should be unset after |= operation", i);
            }
        }
    }

    #[test]
    fn test_bitxor_assign_operator() {
        let mut bs1 = Bitset::new(64);
        let mut bs2 = Bitset::new(64);
        bs1.set(5);
        bs1.set(10);
        bs2.set(10);
        bs2.set(15);

        bs1 ^= bs2;
        // After ^=, bits 5 and 15 should be set; bit 10 should be cleared.
        for i in 0..bs1.len() {
            if i == 5 || i == 15 {
                assert!(bs1.test(i), "Bit {} should be set after ^= operation", i);
            } else {
                assert!(!bs1.test(i), "Bit {} should be unset after ^= operation", i);
            }
        }
    }
}
