use anyhow::{anyhow, Error, Result};

fn mod_it(left: isize, right: isize) -> isize {
    let left = match left < 0 {
        true => right + left,
        _ => left,
    };

    left % right
}

#[derive(Debug, PartialEq, Clone)]
struct FieldElement {
    number: isize,
    prime: isize,
}

impl FieldElement {
    fn add(self, right: Self) -> Result<Self> {
        let temp = self.clone();
        if !temp.same_base(right.clone()) {
            return Err(anyhow!("Prime base not the same between two FieldElement"));
        };

        let adder = self.number + right.number;
        let new_number = mod_it(adder, self.prime);
        Ok(Self {
            number: new_number,
            prime: self.prime,
        })
    }

    fn mul(self, right: Self) -> Result<Self> {
        let temp = self.clone();
        if !temp.same_base(right.clone()) {
            return Err(anyhow!("Prime base not the same between two FieldElement"));
        };

        let multiplier = self.number * right.number;
        let new_number = mod_it(multiplier, self.prime);

        Ok(Self {
            number: new_number,
            prime: self.prime,
        })
    }

    fn same_base(self, right: Self) -> bool {
        self.prime == right.prime
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exception_raised() {
        let a = FieldElement {
            number: 44,
            prime: 57,
        };
        let b = FieldElement {
            number: 33,
            prime: 58,
        };
        let result = a.add(b);
        let error = result.unwrap_err();
        assert_eq!(
            error.to_string(),
            "Prime base not the same between two FieldElement"
        );
    }

    #[test]
    fn test_positive_mod_1() {
        let result = mod_it(9, 19);
        assert_eq!(result, 9);
    }

    #[test]
    fn test_positive_mod_2() {
        let result = mod_it(22, 19);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_negative_mod_1() {
        let result = mod_it(-9, 19);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_negative_mod_2() {
        let result = mod_it(-7, 19);
        assert_eq!(result, 12);
    }

    #[test]
    fn test_add_1() {
        let a = FieldElement {
            number: 44,
            prime: 57,
        };
        let b = FieldElement {
            number: 33,
            prime: 57,
        };

        let result = a.add(b).unwrap();
        let expected = FieldElement {
            number: 20,
            prime: 57,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_2() {
        let a = FieldElement {
            number: 9,
            prime: 57,
        };
        let b = FieldElement {
            number: -29,
            prime: 57,
        };

        let result = a.add(b).unwrap();
        let expected = FieldElement {
            number: 37,
            prime: 57,
        };
        assert_eq!(result, expected);
    }
    #[test]
    fn test_add_3() {
        let a = FieldElement {
            number: 17,
            prime: 57,
        };
        let b = FieldElement {
            number: 42,
            prime: 57,
        };
        let c = FieldElement {
            number: 49,
            prime: 57,
        };

        let result = a.add(b).unwrap().add(c).unwrap();
        let expected = FieldElement {
            number: 51,
            prime: 57,
        };
        assert_eq!(result, expected);
    }
    #[test]
    fn test_add_4() {
        let a = FieldElement {
            number: 52,
            prime: 57,
        };
        let b = FieldElement {
            number: -30,
            prime: 57,
        };
        let c = FieldElement {
            number: -38,
            prime: 57,
        };

        let result = a.add(b).unwrap().add(c).unwrap();
        let expected = FieldElement {
            number: 41,
            prime: 57,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_equal() {
        let a = FieldElement {
            number: 7,
            prime: 13,
        };
        let b = FieldElement {
            number: 7,
            prime: 13,
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_not_equal() {
        let a = FieldElement {
            number: 8,
            prime: 13,
        };
        let b = FieldElement {
            number: 7,
            prime: 13,
        };
        assert_ne!(a, b);
    }

    #[test]
    fn test_mul_1() {
        let a = FieldElement {
            number: 5,
            prime: 19,
        };
        let b = FieldElement {
            number: 3,
            prime: 19,
        };
        let result = a.mul(b).unwrap();
        let expected = FieldElement {
            number: 15,
            prime: 19,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mul_2() {
        let a = FieldElement {
            number: 8,
            prime: 19,
        };
        let b = FieldElement {
            number: 17,
            prime: 19,
        };
        let result = a.mul(b).unwrap();
        let expected = FieldElement {
            number: 3,
            prime: 19,
        };
        assert_eq!(result, expected);
    }
}
