use modinverse::egcd;
use nalgebra::Vector2;

#[derive(Debug, PartialEq, Eq)]
pub struct Lattice {
    pub offset: Vector2<i64>,
    pub step: Vector2<i64>,
}

impl Lattice {
    pub fn is_solution(&self, x: i64, y: i64) -> bool {
        let [dx, dy]: [i64; 2] = (Vector2::new(x, y) - self.offset).into();
        let [sx, sy]: [i64; 2] = self.step.into();

        if dx == 0 {
            return dy % sy == 0;
        }

        dx % sx == 0 && dy == dx / sx * sy
    }
}

pub fn linear_diophantine_equation(a: i64, b: i64, c: i64) -> Option<Lattice> {
    let (d, x_d, y_d) = egcd(a, b);

    if c % d != 0 {
        return None;
    }

    Some(Lattice {
        offset: [c / d * x_d, c / d * y_d].into(),
        step: [b / d, -a / d].into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diophantine() {
        assert!(linear_diophantine_equation(4, 5, 0).unwrap().is_solution(0, 0));
        assert!(linear_diophantine_equation(4, 5, 4).unwrap().is_solution(-4, 4));
        assert!(linear_diophantine_equation(4, 5, 4).unwrap().is_solution(1, 0));
        assert!(linear_diophantine_equation(4, 0, 4).unwrap().is_solution(1, 0));
        assert!(!linear_diophantine_equation(4, 5, 4).unwrap().is_solution(1, 2));
        assert_eq!(linear_diophantine_equation(6, 9, 4), None);
    }
}
