use core::mem;

pub struct Bits([u8]);

impl Bits {
    #[inline]
    pub fn iter(&self) -> Iter { Iter { bits: self, pos: 0 } }
}

impl<'a> From<&'a [u8]> for &'a Bits {
    #[inline]
    fn from(bs: &'a [u8]) -> Self { unsafe { mem::transmute(bs) } }
}

impl<'a> From<&'a mut [u8]> for &'a mut Bits {
    #[inline]
    fn from(bs: &'a mut [u8]) -> Self { unsafe { mem::transmute(bs) } }
}

pub struct Iter<'a> {
    bits: &'a Bits,
    pos: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = bool;
    #[inline]
    fn next(&mut self) -> Option<bool> {
        let (m, n) = (self.pos & 7, self.pos >> 3);
        self.pos += 1;
        self.bits.0.get(n).map(|x| 0 != x & (1 << m))
    }
}
