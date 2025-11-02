// I can see why someone would have thought this lint
// would be a good idea, but.. what if I want to
// implement Mul for a polynomial??
#![allow(clippy::suspicious_arithmetic_impl)]

use auto_ops::impl_op_ex;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BivariatePolynomial {
    coeffs: HashMap<(u16, u16), usize>,
}

pub type BP = BivariatePolynomial;

impl BivariatePolynomial {
    pub fn new(list: &[((u16, u16), usize)]) -> Self {
        BivariatePolynomial {
            coeffs: list.iter().filter(|(_, v)| v != &0).cloned().collect(),
        }
    }

    pub fn constant(n: usize) -> Self {
        Self::new(&[((0, 0), n)])
    }

    pub fn x() -> Self {
        Self::new(&[((1, 0), 1)])
    }

    pub fn y() -> Self {
        Self::new(&[((0, 1), 1)])
    }

    pub fn get_constant(&self) -> Option<usize> {
        if let Some((k, v)) = self.coeffs.iter().next() {
            if self.coeffs.len() == 1 && *k == (0, 0) {
                Some(*v)
            } else {
                // Is not constant.
                None
            }
        } else {
            // No coefficients available => constant zero.
            Some(0)
        }
    }

    // Also returns true for constants.
    pub fn is_linear(&self) -> bool {
        for ((x, y), v) in self.coeffs.iter() {
            if *v != 0 && x + y > 1 {
                return false;
            }
        }

        true
    }

    pub fn evaluate(&self, x: usize, y: usize) -> usize {
        let mut out: usize = 0;
        // Not using Horner's scheme here because the distribution
        // of non-zero terms might be sparse.
        for ((px, py), v) in self.coeffs.iter() {
            out += x.pow(*px as u32) * y.pow(*py as u32) * v;
        }

        out
    }

    pub fn get_coeff(&self, pow_x: u16, pow_y: u16) -> usize {
        *self.coeffs.get(&(pow_x, pow_y)).unwrap_or(&0)
    }
}

impl_op_ex!(+ |a: &BivariatePolynomial, b: &BivariatePolynomial| -> BivariatePolynomial {
    let mut out = a.clone();

    for (k, v) in &b.coeffs {
        *out.coeffs.entry(*k).or_insert(0) += v;
    };

    out
});

// I guess rustfmt detects the * as a deref, which tells me that maybe
// the impl_op_ex macro in its current form is not ideal. Anyway, I
// would like to keep that space between `* |` thank you very much.
#[rustfmt::skip]
impl_op_ex!(* |a: &BivariatePolynomial, b: &BivariatePolynomial| -> BivariatePolynomial {
        let mut out = BivariatePolynomial::new(&[]);

        for ((xa, ya), va) in &a.coeffs {
            for ((xb, yb), vb) in &b.coeffs {
                let k = (xa + xb, ya + yb);
                *out.coeffs.entry(k).or_insert(0) += va * vb;
            }
        }

        out
    }
);

const SUPERSCRIPT: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];

fn superscript_number(num: u16) -> String {
    num.to_string()
        .chars()
        .map(|c| SUPERSCRIPT[c.to_digit(10).unwrap() as usize])
        .collect()
}

impl fmt::Display for BivariatePolynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;

        for (i, ((x, y), v)) in self.coeffs.iter().sorted().enumerate() {
            if i > 0 {
                write!(f, " + ")?;
            }
            write!(
                f,
                "{} x{}y{}",
                v,
                superscript_number(*x),
                superscript_number(*y)
            )?;
        }

        write!(f, ")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_add() {
        let zero = BP::new(&[]);
        let one = BP::new(&[((0, 0), 1)]);
        let x = BP::new(&[((1, 0), 1)]);

        assert_eq!(&zero + &zero, zero);
        assert_eq!(&zero + &one, one);
        assert_eq!(&one + &zero, one);

        let r = &one + &x;
        assert_eq!(r, BP::new(&[((0, 0), 1), ((1, 0), 1)]));

        assert_eq!(&r + &zero, r);

        assert_eq!(&r + &r, BP::new(&[((0, 0), 2), ((1, 0), 2)]));
    }

    #[test]
    fn test_polynomial_mul() {
        let zero = BP::new(&[]);
        let one = BP::new(&[((0, 0), 1)]);
        let x = BP::new(&[((1, 0), 1)]);
        let y = BP::new(&[((0, 1), 1)]);

        assert_eq!(&zero * &zero, zero);
        assert_eq!(&zero * &one, zero);
        assert_eq!(&zero * &x, zero);
        assert_eq!(&one * &one, one);
        assert_eq!(&one * &x, x);
        assert_eq!((&one + &x) * &one, (&one + &x));
        assert_eq!(&x * &y, BP::new(&[((1, 1), 1)]));
    }

    #[test]
    fn test_fmt() {
        let zero = BP::new(&[]);
        let one = BP::new(&[((0, 0), 1)]);
        let x = BP::new(&[((1, 0), 1)]);
        let y = BP::new(&[((0, 1), 1)]);

        assert_eq!(format!("{}", &zero), "()");
        assert_eq!(format!("{}", &one), "(1 x⁰y⁰)");
        assert_eq!(format!("{}", &x), "(1 x¹y⁰)");
        assert_eq!(format!("{}", &one + &x), "(1 x⁰y⁰ + 1 x¹y⁰)");
        assert_eq!(format!("{}", &x * &y), "(1 x¹y¹)");
    }

    #[test]
    fn test_init() {
        let zero = BP::new(&[]);
        let one = BP::new(&[((0, 0), 1)]);
        let x = BP::new(&[((1, 0), 1)]);
        let y = BP::new(&[((0, 1), 1)]);

        assert_eq!(zero, BP::constant(0));
        assert_eq!(one, BP::constant(1));
        assert_eq!(x, BP::x());
        assert_eq!(y, BP::y());
    }

    #[test]
    fn test_superscript() {
        assert_eq!(superscript_number(0), "⁰");
        assert_eq!(superscript_number(123), "¹²³");
    }

    #[test]
    fn test_get_constant() {
        assert_eq!(BP::constant(0).get_constant(), Some(0));
        assert_eq!(BP::constant(1).get_constant(), Some(1));
        assert_eq!(BP::new(&[((1, 0), 1)]).get_constant(), None);
    }

    #[test]
    fn test_is_linear() {
        assert_eq!(BP::x().is_linear(), true);
        assert_eq!(BP::y().is_linear(), true);
        assert_eq!(BP::constant(3).is_linear(), true);
        assert_eq!((BP::x() + BP::y() + BP::constant(3)).is_linear(), true);
        assert_eq!((BP::x() * BP::y()).is_linear(), false);
        assert_eq!((BP::x() * BP::x()).is_linear(), false);
    }

    #[test]
    fn test_evaluate() {
        assert_eq!((BP::x() + BP::y()).evaluate(2, 4), 6);
        assert_eq!((BP::x() * BP::y() + BP::y()).evaluate(2, 4), 12);
        assert_eq!((BP::constant(3)).evaluate(2, 4), 3);
    }
}
