use std::fmt::Write;

#[derive(Clone, Debug)]
pub struct Bitset {
    len: usize,
    bits: Vec<u64>,
}

impl Bitset {
    pub fn new(len: usize) -> Self {
        Self {
            len,
            bits: vec![0u64; (len >> SHIFT) + 1],
        }
    }

    pub fn clear_all(&mut self) {
        self.bits.fill(0);
    }
}

const HI64: u64 = 1u64 << 63;
const SHIFT: u32 = 6;

impl Bitset {
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn capacity(&self) -> usize {
        u64::BITS as usize * self.bits.len()
    }
    pub fn bits(&self) -> &[u64] {
        let ix = (self.len >> SHIFT) + ((self.len & 63 != 0) as usize);
        &self.bits[..ix]
    }
    pub fn bits_mut(&mut self) -> &mut [u64] {
        let ix = (self.len >> SHIFT) + ((self.len & 63 != 0) as usize);
        &mut self.bits[..ix]
    }

    pub fn push(&mut self, bit: bool) {
        if self.len >> SHIFT == self.bits.len() {
            self.bits.push(0u64);
        }
        let b = HI64.wrapping_shr(self.len as u32);
        self.bits[self.len >> SHIFT] = (self.bits[self.len >> SHIFT] & !b) | bit as u64 * b;
        self.len += 1;
    }

    pub fn pop(&mut self) -> bool {
        debug_assert_ne!(self.len, 0);
        self.len -= 1;
        self.get(self.len)
    }

    #[inline]
    pub fn get(&self, ix: usize) -> bool {
        debug_assert!(ix < self.len);
        self.bits[ix >> SHIFT] & HI64.wrapping_shr(ix as u32) != 0
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, ix: usize) -> bool {
        debug_assert!(ix < self.len);
        *self.bits.get_unchecked(ix >> SHIFT) & HI64.wrapping_shr(ix as u32) != 0
    }

    #[inline]
    #[track_caller]
    pub fn set(&mut self, ix: usize) {
        debug_assert!(ix < self.len);
        self.bits[ix >> SHIFT] |= HI64.wrapping_shr(ix as u32);
    }

    #[inline]
    #[track_caller]
    pub unsafe fn set_unchecked(&mut self, ix: usize) {
        debug_assert!(ix < self.len);
        *self.bits.get_unchecked_mut(ix >> SHIFT) |= HI64.wrapping_shr(ix as u32);
    }

    pub fn set_many(&mut self, ix: usize, mask: u64) {
        assert!(ix + 64 < self.len, "ix={ix}, len={}", self.len);
        let trunc = ix & u64::BITS as usize - 1;
        if trunc == 0 {
            self.bits[ix >> SHIFT] |= mask;
        } else {
            self.bits[ix >> SHIFT] |= mask >> trunc;
            self.bits[(ix >> SHIFT) + 1] |= mask << (u64::BITS as usize - trunc);
        }
    }

    #[inline]
    pub fn clear(&mut self, ix: usize) {
        debug_assert!(ix < self.len);
        self.bits[ix >> SHIFT] &= !HI64.wrapping_shr(ix as u32);
    }

    #[inline]
    pub unsafe fn clear_unchecked(&mut self, ix: usize) {
        debug_assert!(ix < self.len);
        *self.bits.get_unchecked_mut(ix >> SHIFT) &= !HI64.wrapping_shr(ix as u32);
    }

    pub fn count_ones(&self) -> u32 {
        let [head @ .., tail] = self.bits() else {
            return 0;
        };
        let sum: u32 = head.iter().copied().map(u64::count_ones).sum();
        // make sure the out of bounds bits don't contribute to the total.
        // fails when self.len == 0, but we don't get here if that's the case.
        let mask = u64::MAX << 63 - (self.len - 1 & 63);
        sum + (tail & mask).count_ones()
    }

    pub fn count_zeros(&self) -> u32 {
        let [head @ .., tail] = self.bits() else {
            return 0;
        };
        let sum: u32 = head.iter().copied().map(u64::count_zeros).sum();
        // make sure the out of bounds bits don't contribute to the total.
        let mask = u64::MAX << 63 - (self.len - 1 & 63);
        // let tail = tail | u64::MAX.wrapping_shr(self.len as u32);
        sum + (tail | !mask).count_zeros()
    }
}

#[derive(Copy, Clone)]
pub struct DebugBitset<'a>(pub &'a Bitset, pub usize, pub usize);

impl std::fmt::Debug for DebugBitset<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut row = 0;
        while row < self.2 {
            for x in 0..self.1 {
                let ch = match (
                    self.0.get(self.1 * row + x),
                    row + 1 < self.2 && self.0.get(self.1 * (row + 1) + x),
                ) {
                    (true, true) => '█',
                    (true, false) => '▀',
                    (false, true) => '▄',
                    (false, false) => ' ',
                };
                f.write_char(ch)?;
            }
            f.write_char('\n')?;
            row += 2;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_count() {
        for i in 0..256 {
            let mut bitset = Bitset::new(i);
            bitset.bits_mut().fill(u64::MAX);
            assert_eq!(bitset.count_ones(), i as u32);
        }
    }

    #[test]
    fn test_zero_count() {
        for i in 0..256 {
            let bitset = Bitset::new(i);
            assert_eq!(bitset.count_zeros(), i as u32);
        }
    }
}

pub fn copy(dst: &mut Bitset, src: &Bitset) {
    assert_eq!(dst.len, src.len);
    unsafe { std::ptr::copy_nonoverlapping(&src.bits, &mut dst.bits, src.bits.len()) };
}

macro_rules! bitwise_impl {
    ($trait:ident, $op:ident, $binop:ident) => {
        impl<'a> $trait<&'a Bitset> for Bitset {
            fn $op(&mut self, rhs: &'a Self) {
                assert_eq!(self.len, rhs.len);
                for i in 0..self.bits.len() {
                    unsafe { u64::$op(self.bits.get_unchecked_mut(i), *rhs.bits.get_unchecked(i)) };
                }
            }
        }

        impl Bitset {
            pub fn $binop(out: &mut Bitset, lhs: &Bitset, rhs: &Bitset) {
                assert_eq!(out.len, lhs.len);
                assert_eq!(out.len, rhs.len);
                for i in 0..out.bits.len() {
                    unsafe {
                        *out.bits.get_unchecked_mut(i) =
                            u64::$binop(*lhs.bits.get_unchecked(i), *lhs.bits.get_unchecked(i));
                    };
                }
            }
        }
    };
}

mod ops {
    use super::Bitset;
    use std::ops::{BitAnd, BitOr, BitXor};
    use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign};
    bitwise_impl!(BitAndAssign, bitand_assign, bitand);
    bitwise_impl!(BitOrAssign, bitor_assign, bitor);
    bitwise_impl!(BitXorAssign, bitxor_assign, bitxor);
}
