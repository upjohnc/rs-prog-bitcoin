use anyhow::{anyhow, Result};

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
        if !self.prime == right.prime {
            return Err(anyhow!("Prime base not the same between two FieldElement"));
        };

        let adder = self.number + right.number;
        let new_number = mod_it(adder, self.prime);
        Ok(Self {
            number: new_number,
            prime: self.prime,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
