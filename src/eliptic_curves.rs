use anyhow::{anyhow, Result};

pub const CURVE_ERROR: &str = "Cannot be on the curve";

#[derive(PartialEq, Debug, Clone)]
struct Point {
    a: isize,
    b: isize,
    x: Option<isize>,
    y: Option<isize>,
}

impl Point {
    fn new(x: isize, y: isize, a: isize, b: isize) -> Result<Self> {
        if y.pow(2) != x.pow(3) + a * x + b {
            return Err(anyhow!(CURVE_ERROR));
        }
        Ok(Self {
            a,
            b,
            x: Some(x),
            y: Some(y),
        })
    }

    fn new_infinity(a: isize, b: isize) -> Self {
        Self {
            a,
            b,
            x: None,
            y: None,
        }
    }

    fn add(self, other: Self) -> Self {
        if self.x.is_none() {
            return other;
        }
        if other.x.is_none() {
            return self;
        }
        if self.x == other.x && self.y != other.y {
            return Self::new_infinity(self.a, self.b);
        }
        if self.x != other.x {
            let self_x = self.x.unwrap();
            let other_x = other.x.unwrap();
            let self_y = self.y.unwrap();
            let other_y = other.y.unwrap();
            let slope = (other_y - self_y) / (other_x - self_x);
            let x_3 = slope.pow(2) - self_x - other_x;
            let y_3 = slope * (self_x - x_3) - self_y;
            return Self::new(x_3, y_3, self.a, self.b).unwrap();
        }
        if self == other && self.y.unwrap() == 0 {
            return Self::new_infinity(self.a, self.b);
        }
        if self == other {
            let x_1 = self.x.unwrap();
            let y_1 = self.y.unwrap();
            let slope = (3 * x_1.pow(2) + self.a) / (2 * y_1);
            let x_3 = slope.pow(2) - 2 * x_1;
            let y_3 = slope * (x_1 - x_3) - y_1;
            return Self::new(x_3, y_3, self.a, self.b).unwrap();
        }

        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ne() {
        let a = Point::new(3, -7, 5, 7).unwrap();
        let b = Point::new(18, 77, 5, 7).unwrap();
        assert_ne!(a, b);
        assert_eq!(a, a);
    }

    #[test]
    fn test_exception_raised() {
        let result = Point::new(5, 7, 5, 7);
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), CURVE_ERROR);
    }

    #[test]
    fn test_add_0() {
        let a = Point::new_infinity(5, 7);
        let b = Point::new(2, 5, 5, 7).unwrap();
        let c = Point::new(2, -5, 5, 7).unwrap();

        let a_ = a.clone();
        assert_eq!(a_.add(b.clone()), b.clone());
        let b_ = b.clone();
        assert_eq!(b_.add(a.clone()), b.clone());
        let b_ = b.clone();
        assert_eq!(b_.add(c.clone()), a.clone());
    }

    #[test]
    fn test_add1() {
        let a = Point::new(3, 7, 5, 7).unwrap();
        let b = Point::new(-1, -1, 5, 7).unwrap();
        assert_eq!(a.add(b), Point::new(2, -5, 5, 7).unwrap());
    }

    #[test]
    fn test_add2() {
        let a = Point::new(-1, -1, 5, 7).unwrap();
        let a_ = a.clone();
        assert_eq!(a_.add(a), Point::new(18, 77, 5, 7).unwrap());
    }
}
