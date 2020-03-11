use crate::BFError;
use crate::BFKindOfError;

use rand::Rng;
use std::cmp::min;
use std::fmt;

pub struct BooleanFunc {
    n_vars: usize,
    func: Vec<u32>,
}

impl BooleanFunc {
    pub fn new() -> Self {
        BooleanFunc {
            n_vars: 0,
            func: Vec::new(),
        }
    }

    pub fn gen_random(n: usize) -> Self {
        if n == 0 {
            return BooleanFunc {
                n_vars: 0,
                func: Vec::new(),
            };
        }

        let n_values = 1 << n;

        let mut rng = rand::thread_rng();
        let mut values = Vec::<u32>::new();

        for _ in 0..(n_values / 32) {
            values.push(rng.gen());
        }

        if n_values & 31 != 0 {
            values.push(rng.gen::<u32>() & ((1 << (n_values & 31)) - 1) as u32);
        }

        BooleanFunc {
            n_vars: n,
            func: values,
        }
    }

    pub fn gen_const_zero(n: usize) -> Self {
        if n == 0 {
            return BooleanFunc {
                n_vars: 0,
                func: Vec::new(),
            };
        }

        let n_values = 1 << n;

        BooleanFunc {
            n_vars: n,
            func: {
                let mut values = Vec::<u32>::new();

                for _ in 0..(n_values / 32) {
                    values.push(0);
                }

                if n_values & 31 != 0 {
                    values.push(0);
                }

                values
            },
        }
    }

    pub fn gen_const_one(n: usize) -> Self {
        if n == 0 {
            return BooleanFunc {
                n_vars: 0,
                func: Vec::new(),
            };
        }

        let n_values = 1 << n;

        BooleanFunc {
            n_vars: n,
            func: {
                let mut values = Vec::<u32>::new();

                for _ in 0..(n_values / 32) {
                    values.push(u32::max_value());
                }

                if n_values & 31 != 0 {
                    values.push(u32::max_value() & ((1 << (n_values & 31)) - 1) as u32);
                }

                values
            },
        }
    }

    pub fn from_str(s: &str) -> Result<BooleanFunc, BFError> {
        let str_size = s.len();

        if str_size == 0 {
            return Ok(BooleanFunc {
                n_vars: 0,
                func: Vec::new(),
            });
        }

        if str_size & (str_size - 1) != 0 {
            return Err(BFError::new(
                BFKindOfError::BadVectorSize,
                "Wrong vector size",
            ));
        }

        let n_vars = (str_size - 1).count_ones() as usize;

        let mut tmp: u32 = 0;
        let mut values = Vec::<u32>::new();

        for i in 0..str_size {
            let c = &s[i..i + 1];
            match c {
                "1" => tmp |= 1 << (i & 31) as u32,
                "0" => {}
                _ => {
                    return Err(BFError::new(
                        BFKindOfError::ParseError,
                        format!("Wrong symbol: `{}`", c).as_str(),
                    ))
                }
            }

            if i != 0 && i & 31 == 0 {
                values.push(tmp);
                tmp = 0;
            }
        }

        values.push(tmp);

        Ok(BooleanFunc {
            n_vars: n_vars,
            func: values,
        })
    }

    pub fn weight(&self) -> usize {
        let mut weight = 0;

        let n = self.func.len();
        let mut i = 0;
        let mut gr;
        let mut s;
        let mut x;

        while i < n {
            gr = min(i + 31, n);
            s = 0;

            for j in i..gr {
                x = self.func[j];
                x = x.overflowing_sub((x >> 1) & 0x55555555).0;
                x = (x & 0x33333333) + ((x >> 2) & 0x33333333);
                x = (x + (x >> 4)) & 0x0F0F0F0F;

                s = s + x;
            }

            s = (s & 0x00FF00FF) + ((s >> 8) & 0x00FF00FF);
            s = (s & 0x0000FFFF) + (s >> 16);

            weight += s as usize;
            i += 31;
        }

        weight
    }

    pub fn mu(&self) -> BooleanFunc {
        BooleanFunc {
            n_vars: 0,
            func: Vec::new(),
        }
    }
}

impl Clone for BooleanFunc {
    fn clone(&self) -> Self {
        BooleanFunc {
            n_vars: self.n_vars,
            func: self.func.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.n_vars = source.n_vars;
        self.func = source.func.clone();
    }
}

impl PartialEq for BooleanFunc {
    fn eq(&self, other: &Self) -> bool {
        self.n_vars == other.n_vars && self.func == other.func
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for BooleanFunc {}

impl fmt::Display for BooleanFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.n_vars == 0 {
            return write!(f, "<empty>");
        }

        let mut s = String::new();
        let n_bits = 1 << self.n_vars;

        for i in 0..n_bits {
            let v = self.func[i / 32];
            match v >> (i % 32) as u32 & 1 {
                0 => s += "0",
                1 => s += "1",
                _ => {}
            }
        }

        write!(f, "{}", s)
    }
}

impl fmt::Debug for BooleanFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{n_vars: {}, func: {}}}", self.n_vars, self)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_create_from_str() {
        let bf = BooleanFunc::from_str("01").unwrap();

        assert_eq!(format!("{}", bf).as_str(), "01");
    }

    #[test]
    fn test_create_from_empty_str() {
        let bf1 = BooleanFunc::new();
        let bf2 = BooleanFunc::from_str("").unwrap();

        assert_eq!(bf1, bf2);
    }

    #[test]
    #[should_panic(expected = "Wrong vector size")]
    fn test_create_wrong_len_str() {
        BooleanFunc::from_str("011").unwrap();
    }

    #[test]
    #[should_panic(expected = "Wrong symbol: `2`")]
    fn test_create_wrong_str() {
        BooleanFunc::from_str("02").unwrap();
    }

    #[test]
    fn test_gen_random() {
        let bf1 = BooleanFunc::new();
        let bf2 = BooleanFunc::gen_random(0);

        assert_eq!(bf1, bf2);
    }

    #[test]
    fn test_create_const_zero() {
        let bf = BooleanFunc::gen_const_zero(5);
        assert_eq!(
            format!("{}", bf).as_str(),
            "00000000000000000000000000000000"
        );

        let bf = BooleanFunc::gen_const_zero(3);
        assert_eq!(format!("{}", bf).as_str(), "00000000");

        assert_eq!(BooleanFunc::gen_const_zero(0), BooleanFunc::new());
    }

    #[test]
    fn test_create_const_one() {
        let bf = BooleanFunc::gen_const_one(5);
        assert_eq!(
            format!("{}", bf).as_str(),
            "11111111111111111111111111111111"
        );

        let bf = BooleanFunc::gen_const_one(3);
        assert_eq!(format!("{}", bf).as_str(), "11111111");

        assert_eq!(BooleanFunc::gen_const_one(0), BooleanFunc::new());
    }

    #[test]
    fn test_calc_weight() {
        let bf1 = BooleanFunc::from_str("01010101").unwrap();
        let bf2 = BooleanFunc::from_str("1111").unwrap();
        let bf3 = BooleanFunc::from_str(
            "00110110100111001110111100000111\
            00111111111000000011110101010101",
        )
        .unwrap();

        assert_eq!(bf1.weight(), 4);
        assert_eq!(bf2.weight(), 4);
        assert_eq!(bf3.weight(), 36);
    }

    #[test]
    fn test_clone() {
        let bf1 = BooleanFunc::from_str("01").unwrap();
        let bf2 = bf1.clone();

        assert_eq!(bf1, bf2);
    }

    #[test]
    fn test_clone_from() {
        let bf1 = BooleanFunc::from_str("01").unwrap();
        let mut bf2 = BooleanFunc::new();

        bf2.clone_from(&bf1);

        assert_eq!(bf1, bf2);
    }

    #[test]
    fn test_partialeq_eq() {
        let bf1 = BooleanFunc::from_str("01").unwrap();
        let bf2 = bf1.clone();

        assert!(bf1 == bf2);
    }

    #[test]
    fn test_partialeq_ne() {
        let bf1 = BooleanFunc::from_str("01").unwrap();
        let bf2 = BooleanFunc::from_str("10").unwrap();

        assert!(bf1 != bf2);
    }

    #[test]
    fn test_display() {
        let values = "01011010";
        let bf = BooleanFunc::from_str(values).unwrap();

        assert_eq!(format!("{}", bf).as_str(), values);
    }

    #[test]
    fn test_display_empty() {
        let bf = BooleanFunc::new();

        assert_eq!(format!("{}", bf).as_str(), "<empty>");
    }
}
