use anyhow::{anyhow, Result};

#[derive(PartialEq, Debug, Clone)]
struct Point {
    a: isize,
    b: isize,
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize, a: isize, b: isize) -> Result<Self> {
        if y.pow(2) != x.pow(3) + a * x + b {
            return Err(anyhow!("Cannot be on the curve"));
        }
        Ok(Self { a, b, x, y })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exception_raised() {
        let result = Point::new(5, 7, 5, 7);
        let error = result.unwrap_err();
        assert_eq!(error.to_string(), "Cannot be on the curve");
    }

    #[test]
    fn test_what() {
        assert_eq!(1, 1);
    }
}
