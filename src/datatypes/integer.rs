/*******************************************************************************
 * INTEGER OF ARBITARY LENGTH
 *
 * https://en.wikipedia.org/wiki/Arbitrary-precision_arithmetic
 *
 ******************************************************************************/

use std::ops::{Add/*, Mul*/};
use std::fmt::{ Display, Result, Formatter };

#[derive(Debug)]
pub struct Integer {
    is_negative: bool,
    data: Vec<u8>,
}

impl Integer {
    pub fn from_string(input: String) -> Integer {
        assert!(input.len() > 0);

        let is_negative = input.chars().next().unwrap() == '-';
        let (mut result, mut last_vec_byte) = (Integer { is_negative: is_negative, data: vec![0; 1] }, 0);

        for chr in input.chars().skip(if is_negative { 1 } else { 0 }) {
            assert!(chr.is_digit(10));

            let digit = chr.to_digit(10).unwrap();
            let new_value: u32 = result.data[last_vec_byte] as u32 * 10 + digit;
            //println!("Digit: {:?}, sum {:?}", digit, new_value);
            // Write less significant byte
            result.data[last_vec_byte] = new_value as u8; 

            // If we can shift vector element one more time, it should be at lease two time less than 256.
            if new_value > u8::max_value() as u32 {
                // Write more significant byte
                last_vec_byte += 1;
                result.data.push((new_value >> 8) as u8);
            }
        }
        result.data = result.data.into_iter().rev().collect();
        result
    }
    fn add_value_at(&mut self, mut value: u64, mut pos: usize) {
        let mut carry = 0_u64;
        loop {
            let sum_result: u64 = self.data[pos] as u64 + (value & 0xff) + carry;
            self.data[pos] = sum_result as u8;
            carry = sum_result >> 8;
            value >>= 8;

            if value == 0 && carry == 0 {
                break;
            }

            pos += 1;
            if pos == self.data.len() {
                self.data.push(0)
            }
        }
    }
}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut result = String::new();

        let mut digit = 0u16;
        for byte in &self.data {
            digit = digit * 256 + *byte as u16;
            while digit > 10 {
                // println!("Digit: {:?}", digit);
                result += &(digit % 10).to_string();
                digit /= 10
            }
        }
        result += &(digit).to_string();
        result += if self.is_negative { "-" } else { "" };
        let _ = write!(f, "{}", result.chars().rev().collect::<String>());

        Ok(())
    }
}

impl<T> Add<T> for Integer
    where i64: From<T>
{
    type Output = Integer;

    fn add(mut self, rhs: T) -> Self {
        let value = i64::from(rhs);
        self.is_negative = value < 0;

        self.add_value_at(value.abs() as u64, 0);

        self
    }
}

/*impl<T> Mul<T> for Integer where u64: From<T> {
    type Output = Integer;

    fn mul(self, rhs: T) -> Self {
        let mut result = Integer { 
            is_negative: self.is_negative && 
            data: vec![0; self.data.len()]};
        let (mut digit, _) = (u64::from(rhs), 0_u64);

        let mut i = 0;
        for byte in self.data {
            loop {
                let mul_result: u64 = byte as u64 * digit;
                result = <Integer as Add<u64>>::add(result, mul_result << (8 * i));

                digit >>= 8;
                if digit == 0 { break }
            }
            i += 1;
        }
        result
    }
}*/
