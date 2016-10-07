
/******************************************************************************
 * BITSET
 *
 * Bitset implemented without bit ops
 *
 * https://en.wikipedia.org/wiki/Bit_array
 * 
 ******************************************************************************/

use std::iter::{FromIterator, IntoIterator};
use std::cmp::Ordering;
use super::super::utils::{zip_all, zip_all_with_default};

#[derive(Debug)]
pub struct BitSet {
    size: usize,
    data: Vec<u8>
}

#[inline]
fn bytes_to_fit_bits(bits: usize) -> usize { bits / 8 + if bits % 8 != 0 { 1 } else { 0 }}

#[test]
fn test_btfb() {
    assert_eq!(bytes_to_fit_bits(0), 0);
    assert_eq!(bytes_to_fit_bits(1), 1);
    assert_eq!(bytes_to_fit_bits(8), 1);
    assert_eq!(bytes_to_fit_bits(9), 2);
}

// NOTE: we use little endianness
impl BitSet {
    pub fn new(size: usize) -> BitSet {
        BitSet {
            size: size,
            data: vec![0; bytes_to_fit_bits(size)]
        }
    }

    pub fn not(&self) -> BitSet {
        BitSet::from_iter(self.into_iter().map(|x| if x == 0 { 1 } else { 0 }))
    }

    pub fn or(&self, other: &Self) -> BitSet {
        BitSet::from_iter(zip_all_with_default(self, other, 0)
                          .map(|(this, that)| if this == 1 || that == 1 { 1 } else { 0 }))
    }

    pub fn xor(&self, other: &Self) -> BitSet {
        BitSet::from_iter(zip_all_with_default(self, other, 0)
                          .map(|(this, that)| if this != that { 1 } else { 0 }))
    }

    pub fn and(&self, other: &Self) -> BitSet {
        BitSet::from_iter(zip_all_with_default(self, other, 0)
                         .map(|(this, that)| if this == 1 && that == 1 { 1 } else { 0 })
        )
    }

    pub fn len(&self) -> usize {
        self.size
    }

    /// Returns a bit at position `at`
    pub fn bit(&self, at: usize) -> u8 {
        assert!(at < self.size, "Bit index is of bounds. Index: {}, bitset size: {}", at, self.size);

        if self.data.len() * 8 < at { 0 }
        else {
            let (byte, bit_in_byte) = (self.data[at /8], (at % 8) as u32);
            if (byte / 2u8.pow(bit_in_byte)) % 2 == 0 { 0 } else { 1 }
        }
    }

    /// Sets a bit at position `at`
    pub fn set_bit(&mut self, at: usize, value: u8) {
        assert!(value == 1 || value == 0, "Invalid value for a bit. Can be only `0` or `1`, got: `{}`", value);
        assert!(at < self.size, "Bit index is of bounds. Index: {}, bitset size: {}", at, self.size);

        // If data vector is too short, push some zeroes
        if self.data.len() * 8 < at + 1 {
            for _ in self.data.len()..bytes_to_fit_bits(at + 1) {
                self.data.push(0)
            }
        }

        let (byte, bit_in_byte) = (self.data[at /8], (at % 8) as u32);
        let bit = (byte / 2u8.pow(bit_in_byte)) % 2;

        if bit != value {
            match bit {
                0 => self.data[at /8] += 2u8.pow(bit_in_byte),
                _ => self.data[at /8] -= 2u8.pow(bit_in_byte),
            }
        }
    }
}

impl PartialEq for BitSet {
    fn eq(&self, other: &BitSet) -> bool
    {
        self.size == other.size && self.data == other.data
    }
}

impl PartialOrd for BitSet {
    fn partial_cmp(&self, other: &BitSet) -> Option<Ordering> {
        let mut result = Ordering::Equal;
        for (this, that) in zip_all(self, other) {
            match (this, that) {
                (None, Some(r)) => if r != 0 { result = Ordering::Less },
                (Some(l), None) => if l != 0 { result = Ordering::Greater },
                (Some(l), Some(r)) => if l != r { result = (l.partial_cmp(&r)).unwrap() },
                _ => panic!("Invalid `zip_all` work. Got (None, None)")
            }
        }

        Some(result)
    }
}

pub struct BitIterator<'a> {
    index: usize,
    size: usize,
    data: &'a BitSet
}

impl<'a> Iterator for BitIterator<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.size {
            let result = Some(self.data.bit(self.index));
            self.index += 1;
            result
            }
        else { None }
    }
}

impl<'a> IntoIterator for &'a BitSet {
    type Item = u8;
    type IntoIter = BitIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BitIterator {
            index: 0,
            size: self.size,
            data: self
        }
    }
}

impl<'a> FromIterator<&'a u8> for BitSet {
    fn from_iter<T>(iter: T) -> BitSet where T: IntoIterator<Item=&'a u8> {
        let mut result = BitSet {
            size: 0,
            data: vec![]
        };

        for element in iter.into_iter() {
            let index = result.size;
            result.size +=1;
            result.set_bit(index, *element)
        }
        result 
    }
}

impl FromIterator<u8> for BitSet {
    fn from_iter<T>(iter: T) -> BitSet where T: IntoIterator<Item=u8> {
        let mut result = BitSet {
            size: 0,
            data: vec![]
        };

        for element in iter.into_iter() {
            let index = result.size;
            result.size +=1;
            result.set_bit(index, element)
        }
        result 
    }
}