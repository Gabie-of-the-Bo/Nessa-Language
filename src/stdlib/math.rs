use rand::Rng;
use std::cmp::Ordering;

use crate::number::*;

impl Number{

    /*
                                                      ╒══════════════════════╕
        ============================================= │  BASIC TRIGONOMETRY  │ =============================================
                                                      ╘══════════════════════╛
    */

    pub fn sin(&self) -> Result<Number, String> {
        return match self {
            Number::Float(f) => Ok(Number::Float(f.sin())),
            Number::Int(i) => Ok(Number::Float(i.to_f64().sin())),
        };
    }
    
    pub fn cos(&self) -> Result<Number, String> {
        return match self {
            Number::Float(f) => Ok(Number::Float(f.cos())),
            Number::Int(i) => Ok(Number::Float(i.to_f64().cos())),
        };
    }
    
    pub fn tan(&self) -> Result<Number, String> {
        return match self {
            Number::Float(f) => Ok(Number::Float(f.tan())),
            Number::Int(i) => Ok(Number::Float(i.to_f64().tan())),
        };
    }

    /*
                                                      ╒══════════════════════╕
        ============================================= │  SIMPLE MATHEMATICS  │ =============================================
                                                      ╘══════════════════════╛
    */
    
    pub fn floor(&self) -> Result<Number, String> {
        return match self {
            Number::Float(f) => Ok(Number::from(f.floor() as u64)),
            Number::Int(i) => Ok(Number::Int(i.clone())),
        };
    }
    
    pub fn ceil(&self) -> Result<Number, String> {
        return match self {
            Number::Float(f) => Ok(Number::from(f.ceil() as u64)),
            Number::Int(i) => Ok(Number::Int(i.clone())),
        };
    }
    
    pub fn sqrt(&self) -> Result<Number, String> {
        return match self {
            Number::Float(f) => Ok(Number::Float(f.sqrt())),
            Number::Int(i) => Ok(Number::Float(i.to_f64().sqrt())),
        };
    }

    pub fn pow(&self, exponent: &Number) -> Result<Number, String> {
        return match (self, exponent) {
            (Number::Float(b), Number::Float(e)) => Ok(Number::Float(b.powf(*e))),
            (Number::Float(b), Number::Int(e)) => Ok(Number::Float(b.powf(e.to_f64()))),
            (Number::Int(b), Number::Float(e)) => Ok(Number::Float(b.to_f64().powf(*e))),
            (Number::Int(b), Number::Int(e)) if e.negative => Ok(Number::Float(b.to_f64().powf(e.to_f64()))),
            (Number::Int(b), Number::Int(e)) => Ok(Number::Int(pow(b, e)?)),
        };
    }

    pub fn exp(&self) -> Result<Number, String> {
        return match self {
            Number::Float(f) => Ok(Number::Float(f.exp())),
            Number::Int(i) => Ok(Number::Float(i.to_f64().exp())),
        };
    }
    
    pub fn ln(&self) -> Result<Number, String> {
        return match self {
            Number::Float(f) => Ok(Number::Float(f.ln())),
            Number::Int(i) => Ok(Number::Float(i.to_f64().ln())),
        };
    }
    
    pub fn fact(&self) -> Result<Number, String> {
        return match &self {
            Number::Float(f) if f.fract() == 0.0 => Number::from(*f as u64).fact(),
            Number::Float(_) => unimplemented!("Unable to calculate factorial of fractional numbers (WIP)"),
            Number::Int(i) => {
                let one: Integer = Integer::new(false, vec!(1));
                let mut res = one.clone();
                let mut cpy = i.clone();
                
                while !cpy.is_zero() {
                    res = res * &cpy;
                    cpy = cpy - &one;
                }

                Ok(Number::from(res))
            }
        };
    }

    /*
                                                      ╒═══════════════════╕
        ============================================= │  AUX MATHEMATICS  │ =============================================
                                                      ╘═══════════════════╛
    */

    pub fn rand() -> Number {
        return Number::from(rand::thread_rng().gen_range(0f64..1f64));
    }

    pub fn rand_int_range(from: &Number, to: &Number) -> Result<Number, String> {
        let mut rng = rand::thread_rng();

        let mut rand_limbs_range = |f: &Vec<u64>, t: &Vec<u64>| {
            let max_bits = t.len();
            let min_bits = f.len();

            let mut res = vec!(0; max_bits);
            
            // First number
            if max_bits > min_bits {
                res[max_bits - 1] = rng.gen_range(0..=*t.last().unwrap());
            
            } else {
                res[max_bits - 1] = rng.gen_range(*f.last().unwrap()..=*t.last().unwrap());
            }

            let mut zeroed = res[max_bits - 1] == 0;

            // First zeroes
            for i in min_bits..(max_bits - 1) {
                let digit = rng.gen::<u64>();
                zeroed &= digit == 0;

                res[i] = digit;
            }

            // Next zeroes
            if zeroed {
                for i in 0..(min_bits - 1) {
                    res[i] = rng.gen_range(f[i]..=u64::MAX);
                }

            } else {
                for i in 0..(min_bits - 1) {
                    res[i] = rng.gen::<u64>();
                }
            }

            return res;
        };

        if from >= to {
            return Err(format!("Invalid range for random generation [{}, {}]", String::from(from), String::from(to)));
        }

        return match (from, to) {
            (Number::Float(f), Number::Float(t)) if f.fract() == 0.0 && t.fract() == 0.0 => Ok(Number::from(rng.gen_range(*f..*t))),
            (Number::Float(f), Number::Int(t)) if f.fract() == 0.0 => Ok(Number::from(rng.gen_range(*f..t.to_f64()))),
            (Number::Int(f), Number::Float(t)) if t.fract() == 0.0 => Ok(Number::from(rng.gen_range(f.to_f64()..*t))),
            (Number::Int(f), Number::Int(t)) => match (f.negative, t.negative) {
                (false, false) => Ok(Number::from(Integer::new(false, rand_limbs_range(&f.limbs, &t.limbs)))),
                (true, true) => Ok(Number::from(Integer::new(true, rand_limbs_range(&t.limbs, &f.limbs)))),
                (true, false) => {
                    let limbs;

                    if let Ordering::Less = comp_limbs(&f.limbs, &t.limbs) {
                        limbs = rand_limbs_range(&vec!(0), &t.limbs);

                    } else {
                        limbs = rand_limbs_range(&vec!(0), &f.limbs);
                    }

                    let mut res = Integer::new(false, limbs);

                    // Set as negative uniformly
                    if &res > t {
                        res.negative = true;
                        
                    } else if &res <= f {
                        res.negative = rng.gen::<bool>();
                    }

                    Ok(Number::from(res))
                },

                _ => unreachable!()
            },

            _ => Err(format!("Invalid range for random generation"))
        };
    }
}

/*
                                                  ╒═════════╕
    ============================================= │  TESTS  │ =============================================
                                                  ╘═════════╛
*/

#[cfg(test)]
mod tests {
    use crate::number::*;
    use num_bigint::{BigInt};

    use rand::{
        distributions::{
            Distribution, 
            Uniform
        }
    };

    #[test]
    fn integer_exponentiation() {
        let mut rng = rand::thread_rng();
        let distribution = Uniform::from(1..5);

        for _ in 0..100{
            let a = Integer::rand_with_size(distribution.sample(&mut rng), false);
            let b = Integer::rand_with_size(distribution.sample(&mut rng), false);

            let a2 = a.to_string().parse::<BigInt>().unwrap();

            assert_eq!(pow(&a, &b).unwrap().to_string(), a2.pow(b.limbs[0] as u32).to_string());
        }
    }

    /*
    TODO: check this
    #[test]
    fn integer_modular_exponentiation() {
        let mut rng = rand::thread_rng();
        let distribution = Uniform::from(1..30);

        for _ in 0..10000{
            let a = Integer::rand_with_size(distribution.sample(&mut rng), false);
            let mut b = Integer::rand_with_size(distribution.sample(&mut rng), false);
            let mut m = Integer::rand_with_size(distribution.sample(&mut rng), false);

            while b.is_zero() {
                b = Integer::rand_with_size(distribution.sample(&mut rng), false);
            }

            while m.is_zero() {
                m = Integer::rand_with_size(distribution.sample(&mut rng), false);
            }

            println!("{}, {}, {}", a, b, m);

            let a2 = a.to_string().parse::<BigInt>().unwrap();
            let b2 = b.to_string().parse::<BigInt>().unwrap();
            let m2 = m.to_string().parse::<BigInt>().unwrap();

            assert_eq!(modpow(&a, &b, &m).unwrap().to_string(), a2.modpow(&b2, &m2).to_string());
        }
    }*/
}