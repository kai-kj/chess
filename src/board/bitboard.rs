// general bit operations: https://www.chessprogramming.org/General_Setwise_Operations

macro_rules! bitboard {
    ($r0:expr, $r1:expr, $r2:expr, $r3:expr, $r4:expr, $r5:expr, $r6:expr, $r7:expr $(,)?) => {
        paste::paste! { ([<0b0 $r0 $r1 $r2 $r3 $r4 $r5 $r6 $r7 u64>] as Bitboard) }
    };
}

pub(crate) use bitboard;

pub type Bitboard = u64;

pub trait BitboardTrait {
    const ZERO: Bitboard = u64::MIN;
    const ONE: Bitboard = u64::MAX;

    fn get_bit(self, row: usize, col: usize) -> bool;
    fn set_bit(&mut self, row: usize, col: usize);
    fn unset_bit(&mut self, row: usize, col: usize);
    fn store_bit(&mut self, row: usize, col: usize, value: bool);
    // fn gen_shift(self, s: i64) -> Self;
    fn to_vec(self) -> Vec<bool>;
    fn to_bb_string(self) -> String;
}

impl BitboardTrait for Bitboard {
    fn get_bit(self, row: usize, col: usize) -> bool {
        let n = 63 - (row * 8 + col);
        (self >> n) & 1 != 0
    }

    fn set_bit(&mut self, row: usize, col: usize) {
        let n = 63 - (row * 8 + col);
        *self |= 1 << n
    }

    fn unset_bit(&mut self, row: usize, col: usize) {
        let n = 63 - (row * 8 + col);
        *self &= !(1 << n)
    }

    fn store_bit(&mut self, row: usize, col: usize, value: bool) {
        let n = 63 - (row * 8 + col);
        *self = *self & !(1 << n) | ((value as u64) << n);
    }

    // fn gen_shift(self, s: i64) -> Self {
    //     if s > 0 {
    //         self >> s as u64
    //     } else {
    //         self << -s as u64
    //     }
    // }

    fn to_vec(self) -> Vec<bool> {
        (0..64).map(|n| (self << n) & (1 << 63) != 0).collect()
    }

    fn to_bb_string(self) -> String {
        self.to_vec()
            .iter()
            .enumerate()
            .map(|(idx, &bit)| match idx % 8 {
                7 => format!("{}\n", bit as u64),
                _ => format!("{}", bit as u64),
            })
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const A: u64 = bitboard! {
        10000000,
        11000000,
        11100000,
        11110000,
        11111000,
        11111100,
        11111110,
        11111111,
    };

    const B: u64 = bitboard! {
        10101010,
        01010101,
        10101010,
        01010101,
        10101010,
        01010101,
        10101010,
        01010101,
    };

    #[test]
    fn zero() {
        assert_eq!(Bitboard::ZERO, 0x0000000000000000);
    }

    #[test]
    fn one() {
        assert_eq!(Bitboard::ONE, 0xffffffffffffffff);
    }

    #[test]
    fn get_bit() {
        for i in 0..8 {
            for j in 0..8 {
                if i >= j {
                    assert_eq!(A.get_bit(i, j), true);
                } else {
                    assert_eq!(A.get_bit(i, j), false);
                }
            }
        }
    }

    #[test]
    fn set_bit() {
        let mut bb = Bitboard::ZERO;
        for i in 0..8 {
            for j in 0..8 {
                if i >= j {
                    bb.set_bit(i, j);
                }
            }
        }
        assert_eq!(bb, A);
    }

    #[test]
    fn unset_bit() {
        let mut bb = Bitboard::ONE;
        for i in 0..8 {
            for j in 0..8 {
                if i < j {
                    bb.unset_bit(i, j);
                }
            }
        }
        assert_eq!(bb, A);
    }

    #[test]
    fn toggle_bit() {
        let mut bb = Bitboard::ZERO;
        for i in 0..8 {
            for j in 0..8 {
                bb.store_bit(i, j, i % 2 == j % 2);
            }
        }
        assert_eq!(bb, B);
        for i in 0..8 {
            for j in 0..8 {
                bb.store_bit(i, j, i >= j);
            }
        }
        assert_eq!(bb, A);
    }

    #[test]
    fn string() {
        assert_eq!(
            A.to_bb_string(),
            "\
                10000000\n\
                11000000\n\
                11100000\n\
                11110000\n\
                11111000\n\
                11111100\n\
                11111110\n\
                11111111\n\
            "
        );
    }
}
