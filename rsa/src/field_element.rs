use crate::utils::egcd;
use std::ops::{Add, Mul};

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) struct FieldElement {
    pub(crate) number: i128,
    pub(crate) prime: i128,
}

impl FieldElement {
    pub(crate) fn new(number: i128, prime: i128) -> Self {
        if number < 0 {
            panic!(
                "Number for FieldElement should be non-negative. Given {}",
                number
            );
        }
        if number > prime {
            panic!(
                "Number for FieldElement should be less than prime. Given {}",
                number
            );
        } else {
            FieldElement { number, prime }
        }
    }

    pub(crate) fn pow(&self, exp: Self) -> Self {
        let mut acc = 1;
        let mut base = self.number;
        let mut aux = exp.number % (self.prime - 1);

        while aux > 0 {
            if aux % 2 == 0 {
                base = (base * base) % self.prime;
                aux /= 2;
            } else {
                acc = (acc * base) % self.prime;
                aux -= 1;
            }
        }

        FieldElement {
            number: acc,
            prime: self.prime,
        }
    }

    pub(crate) fn inv(&self) -> Self {
        let (_, x, _) = egcd(self.number, self.prime);
        let result = (x % self.prime + self.prime) % self.prime;

        FieldElement {
            number: result,
            prime: self.prime,
        }
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime == other.prime {
            FieldElement {
                number: (self.number + other.number) % self.prime,
                prime: self.prime,
            }
        } else {
            panic!(
                "Cannot add FieldElements. Different prime values {} and {}",
                self.prime, other.prime
            );
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime == other.prime {
            FieldElement {
                number: (self.number * other.number) % self.prime,
                prime: self.prime,
            }
        } else {
            panic!(
                "Cannot multiply FieldElements. Different prime values {} and {}",
                self.prime, other.prime
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Constructor ---
    #[test]
    fn new_basic() {
        let element = FieldElement::new(12, 13);

        assert_eq!(element.number, 12);
        assert_eq!(element.prime, 13);
    }

    #[test]
    #[should_panic(expected = "should be non-negative")]
    fn new_number_negative() {
        FieldElement::new(-1, 13);
    }

    #[test]
    #[should_panic(expected = "should be less than")]
    fn new_number_greater_than_prime() {
        FieldElement::new(14, 13);
    }

    // --- Addition ---
    #[test]
    fn add_basic() {
        // Same prime.
        let a = FieldElement::new(6, 13);
        let b = FieldElement::new(12, 13);
        assert_eq!(a + b, FieldElement::new(5, 13));

        // Same prime.
        let a = FieldElement::new(6, 13);
        let b = FieldElement::new(2, 13);
        assert_eq!(a + b, FieldElement::new(8, 13));
    }

    #[test]
    #[should_panic(expected = "prime values")]
    fn add_different_prime() {
        let a = FieldElement::new(1, 11);
        let b = FieldElement::new(1, 13);
        _ = a + b;
    }

    // --- Multiplication ---
    #[test]
    fn mul_basic() {
        // Same prime.
        let a = FieldElement::new(6, 13);
        let b = FieldElement::new(12, 13);
        assert_eq!(a * b, FieldElement::new(7, 13));

        // Same prime.
        let a = FieldElement::new(6, 13);
        let b = FieldElement::new(2, 13);
        assert_eq!(a * b, FieldElement::new(12, 13));
    }

    #[test]
    #[should_panic(expected = "prime values")]
    fn mul_different_prime() {
        let a = FieldElement::new(1, 11);
        let b = FieldElement::new(1, 13);
        _ = a * b;
    }

    // --- Pow ---
    #[test]
    fn pow_basic() {
        let number = FieldElement::new(4, 11);
        assert_eq!(
            number.pow(FieldElement::new(39, 97)),
            FieldElement::new(3, 11)
        )
    }

    // --- Modular Inverse ---
    #[test]
    fn inv_basic() {
        let number = FieldElement::new(3, 11);
        assert_eq!(number.inv(), FieldElement::new(4, 11));

        let number = FieldElement::new(10, 17);
        assert_eq!(number.inv(), FieldElement::new(12, 17));

        let number_a = FieldElement::new(5, 11);
        let number_b = FieldElement::new(7, 11);
        assert_eq!((number_a * number_b).inv(), FieldElement::new(6, 11));
    }
}
