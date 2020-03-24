use std::cmp::min;
use std::fmt;

use rand::Rng;

use crate::BFError;
use crate::BFKindOfError;

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

        let n_values: usize = 1 << n;

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

        let n_values: usize = 1 << n;

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

        let n_values: usize = 1 << n;

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

    pub fn from_str(s: &str) -> Result<Self, BFError> {
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
            let c = &s[i..=i];
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
            n_vars,
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
                x = x.overflowing_sub((x >> 1) & 0x5555_5555).0;
                x = (x & 0x3333_3333) + ((x >> 2) & 0x3333_3333);
                x = (x + (x >> 4)) & 0x0F0F_0F0F;

                s += x;
            }

            s = (s & 0x00FF_00FF) + ((s >> 8) & 0x00FF_00FF);
            s = (s & 0x0000_FFFF) + (s >> 16);

            weight += s as usize;
            i += 31;
        }

        weight
    }

    pub fn mu(&self) -> BooleanFunc {
        fn word_process(word: u32) -> u32 {
            let mut result = word ^ ((word << 1) & 0xAAAA_AAAA);
            result ^= (result << 2) & 0xCCCC_CCCC;
            result ^= (result << 4) & 0xF0F0_F0F0;
            result ^= (result << 8) & 0xFF00_FF00;
            result ^= (result << 16) & 0xFFFF_0000;

            result
        }

        if self.n_vars == 0 {
            return BooleanFunc {
                n_vars: 0,
                func: Vec::new(),
            };
        }

        let mut mu_values = self.func.clone();

        if self.n_vars > 5 {
            for i in 0..self.func.len() {
                mu_values[i] = word_process(mu_values[i]);
            }

            let mut step: usize = 1;
            let mut i: usize;

            while step <= (self.func.len() / 2) {
                i = 0;

                while i < self.func.len() {
                    for j in i..i + step {
                        mu_values[j + step] ^= mu_values[j];
                    }
                    i += step * 2;
                }

                step *= 2;
            }

            return BooleanFunc {
                n_vars: self.n_vars,
                func: mu_values,
            };
        }

        mu_values[0] = word_process(mu_values[0]) & ((1 << (1 << self.n_vars)) - 1) as u32;

        BooleanFunc {
            n_vars: self.n_vars,
            func: mu_values,
        }
    }

    pub fn anf(&self) -> String {
        fn get_index(n: usize) -> String {
            let mut index = String::new();
            let mut n = n;

            loop {
                match n % 10 {
                    0 => index = "₀".to_string() + &index,
                    1 => index = "₁".to_string() + &index,
                    2 => index = "₂".to_string() + &index,
                    3 => index = "₃".to_string() + &index,
                    4 => index = "₄".to_string() + &index,
                    5 => index = "₅".to_string() + &index,
                    6 => index = "₆".to_string() + &index,
                    7 => index = "₇".to_string() + &index,
                    8 => index = "₈".to_string() + &index,
                    9 => index = "₉".to_string() + &index,
                    _ => {}
                }

                n /= 10;

                if n == 0 {
                    break;
                }
            }

            index
        }

        let mu = self.mu();

        let mut monoms = Vec::new();

        let n_bits: usize = 1 << self.n_vars;
        let mut curr_bit: usize = 0;

        // u32 block
        'outer: for i in 0..mu.func.len() {
            let v = mu.func[i];

            // mu function values in a block
            for j in 0..32 {
                if v >> j as u32 & 1 == 1 {
                    let mut part = String::new();

                    // x variables values
                    for k in 0..self.n_vars {
                        if curr_bit >> k & 1 == 1 {
                            part = format!("{}x{}", part, get_index(k));
                        }
                    }

                    if part.is_empty() {
                        monoms.push("1".to_string());
                    } else {
                        monoms.push(part);
                    }
                }

                curr_bit += 1;

                if curr_bit == n_bits {
                    break 'outer;
                }
            }
        }

        if monoms.is_empty() {
            return String::from("<empty>");
        }

        monoms.join(" ⊕ ")
    }

    pub fn deg(&self) -> usize {
        let mu = self.mu();

        for values in (0..(1 << self.n_vars)).rev() {
            let v = mu.func[values / 32];
            if v >> (values & 31) as u32 & 1 == 1 {
                return values.count_ones() as usize;
            }
        }

        0
    }

    pub fn wht(&self) -> Vec<i32> {
        let mut wht = Vec::new();

        if self.n_vars == 0 {
            return wht;
        }

        let n_bits: usize = 1 << self.n_vars;

        for i in 0..n_bits {
            let v = self.func[i / 32];
            match v >> (i & 31) as u32 & 1 {
                0 => wht.push(1),
                1 => wht.push(-1),
                _ => {}
            }
        }

        let mut step: usize = 1;
        let mut i: usize;

        while step <= (wht.len() / 2) {
            i = 0;

            while i < wht.len() {
                for j in i..i + step {
                    let tmp = wht[j];
                    wht[j] += wht[j + step];
                    wht[j + step] = tmp - wht[j + step];
                }
                i += step * 2;
            }

            step *= 2;
        }

        wht
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
}

impl Eq for BooleanFunc {}

impl fmt::Display for BooleanFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.n_vars == 0 {
            return write!(f, "<empty>");
        }

        let mut s = String::new();
        let n_bits: usize = 1 << self.n_vars;

        for i in 0..n_bits {
            let v = self.func[i / 32];
            match v >> (i & 31) as u32 & 1 {
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
    #[ignore]
    fn test_gen_random() {
        let bf1 = BooleanFunc::new();
        let bf2 = BooleanFunc::gen_random(0);

        assert_eq!(bf1, bf2);

        let mut bf: BooleanFunc;
        let mut stat: f64 = 0.0;
        let mut count: usize = 0;

        for _ in 0..10 {
            for n in 1..=30 {
                bf = BooleanFunc::gen_random(n);
                stat += bf.weight() as f64 / (1 << n) as f64;
                count += 1;
            }
        }

        stat = stat as f64 / count as f64;

        assert!(0.47 < stat && stat < 0.51);
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
    fn test_mu_func() {
        let bf = BooleanFunc::gen_random(10);

        assert_eq!(bf, bf.mu().mu());
    }

    #[test]
    fn test_anf_func() {
        let bf = BooleanFunc::from_str("0001000100011110").unwrap();
        assert_eq!("x₀x₁ ⊕ x₂x₃", bf.anf());

        let bf = BooleanFunc::from_str(
            "0001000100011110000100010001111000010001000111101110111011100001",
        )
        .unwrap();
        assert_eq!("x₀x₁ ⊕ x₂x₃ ⊕ x₄x₅", bf.anf());
    }

    #[test]
    fn test_deg_func() {
        let bf = BooleanFunc::from_str("0001000100011110").unwrap();
        assert_eq!(2, bf.deg());

        let bf = BooleanFunc::from_str(
            "0001000100011110000100010001111000010001000111101110111011100001",
        )
        .unwrap();
        assert_eq!(2, bf.deg());
    }

    #[test]
    fn test_wht_func() {
        let bf = BooleanFunc::from_str("01110101").unwrap();
        assert_eq!(vec![-2, 6, 2, 2, -2, -2, 2, 2], bf.wht());

        let bf = BooleanFunc::from_str("0001000100011110").unwrap();
        assert_eq!(
            vec![4, 4, 4, -4, 4, 4, 4, -4, 4, 4, 4, -4, -4, -4, -4, 4],
            bf.wht()
        );

        let bf = BooleanFunc::from_str(
            "0001000100011110000100010001111000010001000111101110111011100001",
        )
        .unwrap();
        assert_eq!(
            vec![
                8, 8, 8, -8, 8, 8, 8, -8, 8, 8, 8, -8, -8, -8, -8, 8, 8, 8, 8, -8, 8, 8, 8, -8, 8,
                8, 8, -8, -8, -8, -8, 8, 8, 8, 8, -8, 8, 8, 8, -8, 8, 8, 8, -8, -8, -8, -8, 8, -8,
                -8, -8, 8, -8, -8, -8, 8, -8, -8, -8, 8, 8, 8, 8, -8
            ],
            bf.wht()
        );
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

        assert_eq!(bf1, bf2);
    }

    #[test]
    fn test_partialeq_ne() {
        let bf1 = BooleanFunc::from_str("01").unwrap();
        let bf2 = BooleanFunc::from_str("10").unwrap();

        assert_ne!(bf1, bf2);
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
