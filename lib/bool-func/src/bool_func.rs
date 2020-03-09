use crate::BFError;
use crate::BFKindOfError;

use std::fmt;

pub struct BooleanFunc {
    n_vars: usize,
    func: Vec<u32>,
}

impl BooleanFunc {
    pub fn new() -> BooleanFunc {
        BooleanFunc {
            n_vars: 0,
            func: Vec::new(),
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

        let mut tmp: u32 = 0;
        let mut values: Vec<u32> = Vec::new();

        for i in 0..str_size {
            let c = &s[i..i + 1];
            match c {
                "0" => tmp |= 0 << (i % 32) as u32,
                "1" => tmp |= 1 << (i % 32) as u32,
                _ => {
                    return Err(BFError::new(
                        BFKindOfError::ParseError,
                        format!("Wrong symbol: `{}`", c).as_str(),
                    ))
                }
            }

            if i != 0 && i % 32 == 0 {
                values.push(tmp);
                tmp = 0;
            }
        }

        values.push(tmp);

        Ok(BooleanFunc {
            n_vars: str_size,
            func: values,
        })
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
        let mut s = String::new();

        for i in 0..self.n_vars {
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
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_create_from_empty_str() {
        let bf1 = BooleanFunc::new();
        let bf2 = BooleanFunc::from_str("").unwrap();

        assert_eq!(bf1, bf2);
    }

    #[test]
    fn test_create_from_str() {
        BooleanFunc::from_str("01").unwrap();
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
}
