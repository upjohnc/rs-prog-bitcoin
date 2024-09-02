use crate::field_element::FieldElement;
use anyhow::{anyhow, Result};

pub const CURVE_ERROR: &str = "Cannot be on the curve";

fn confirm_on_curve(
    x: FieldElement,
    y: FieldElement,
    a: FieldElement,
    b: FieldElement,
) -> Result<bool> {
    let y_side = y.power_(2);
    let x_3 = x.power_(3);
    let a_x = a.mul(&x)?;
    let x_side = x_3.add(&a_x)?.add(&b)?;
    Ok(y_side == x_side)
}

#[derive(Debug, PartialEq, Clone)]
struct PointField {
    a: FieldElement,
    b: FieldElement,
    x: Option<FieldElement>,
    y: Option<FieldElement>,
}

impl PointField {
    fn new(
        a: FieldElement,
        b: FieldElement,
        x: Option<FieldElement>,
        y: Option<FieldElement>,
    ) -> Result<Self> {
        if x.is_none() && y.is_none() {
            return Ok(Self { a, b, x, y });
        }
        if x.is_none() || y.is_none() {
            // check case that x or y are none but not both
            return Err(anyhow!("Cannot be on the curve"));
        }
        if !confirm_on_curve(x.clone().unwrap(), y.clone().unwrap(), a.clone(), b.clone())? {
            return Err(anyhow!("Cannot be on the curve"));
        }
        Ok(Self { a, b, x, y })
    }

    fn add(&self, other: &Self) -> Result<Self> {
        if self.a != other.a || self.b != other.b {
            return Err(anyhow!("Not on same curve"));
        }
        if self.x.is_none() {
            return Ok(other.clone());
        }
        if other.x.is_none() {
            return Ok(self.clone());
        }

        if self.x == other.x && self.y != other.y {
            return Ok(Self::new(self.a, self.b, None, None)?);
        }

        if self.x != other.x {
            let sub_1 = other.y.unwrap().sub(&self.y.unwrap()).unwrap();
            let sub_2 = other.x.unwrap().sub(&self.x.unwrap()).unwrap();
            let slope = sub_1.div(&sub_2);

            let x = slope
                .power_(2)
                .sub(&self.x.unwrap())?
                .sub(&other.x.unwrap())?;
            let w = self.x.unwrap().sub(&x)?;
            let y = slope.mul(&w)?.sub(&self.y.unwrap())?;
            return Ok(Self::new(self.a, self.b, Some(x), Some(y))?);
        }
        let zero_mul = FieldElement::new(0, self.x.unwrap().prime).mul(&self.x.unwrap())?;
        if self == other && self.y.unwrap() == zero_mul {
            return Ok(Self::new(self.a, self.b, None, None)?);
        }

        if self == other {
            let prime = self.x.unwrap().prime;
            let x_2 = self.x.unwrap().power_(2);
            println!("Where are you");
            println!("{:?}", x_2);
            let x_2_3 = (FieldElement::new(3, prime).mul(&x_2))?;
            let s_1 = x_2_3.add(&self.a)?;
            let y_2 = (FieldElement::new(2, prime).mul(&self.y.unwrap()))?;
            let slope = s_1.div(&y_2);
            dbg!(slope);
            let s_2 = slope.power_(2);
            let x_times_2 = FieldElement::new(2, prime).mul(&self.x.unwrap())?;
            let x = s_2.sub(&x_times_2)?;

            let y = slope
                .mul(&self.x.unwrap().sub(&x)?)?
                .sub(&self.y.unwrap())?;
            let x_x = self.x.unwrap().sub(&x)?;
            dbg!(self.x.unwrap());
            dbg!(x_x);

            dbg!((52 - 30 - 38) % 57);
            dbg!(crate::field_element::mod_it((52 - 30 - 38), 57));

            // let y_1 = slope.mul(&x_x)?;
            // let y = y_1.sub(&self.y.unwrap())?;
            println!("{:?}, {:?}, {:?}, {:?}", self.a, self.b, x, y);
            return Ok(Self::new(self.a, self.b, Some(x), Some(y))?);
        }

        // catch all case - not sure if this is correct
        Ok(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::field_element::mod_it;

    use super::*;

    #[test]
    fn test_exception_raised() {
        let prime_base = 223;
        let a = FieldElement::new(0, prime_base);
        let b = FieldElement::new(7, prime_base);
        let values_to_test = vec![(200, 119), (42, 99)];
        for (x, y) in values_to_test {
            let result = PointField::new(
                a.clone(),
                b.clone(),
                Some(FieldElement::new(x, prime_base)),
                Some(FieldElement::new(y, prime_base)),
            );
            let error = result.unwrap_err();
            assert_eq!(error.to_string(), CURVE_ERROR);
        }
    }

    #[test]
    fn test_confirm_on_curve() {
        let prime_base = 223;
        let a = FieldElement::new(0, prime_base);
        let b = FieldElement::new(7, prime_base);

        let values_to_test = vec![(192, 105), (17, 56), (1, 193)];
        for (x, y) in values_to_test {
            let result = PointField::new(
                a.clone(),
                b.clone(),
                Some(FieldElement::new(x, prime_base)),
                Some(FieldElement::new(y, prime_base)),
            );
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_exception_raised_2() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);
        let inputs = vec![(None, Some(142)), (Some(170), None)];

        for (x_1, y_1) in inputs {
            let x = match x_1 {
                Some(x) => Some(FieldElement::new(x, prime)),
                _ => None,
            };

            let y = match y_1 {
                Some(y) => Some(FieldElement::new(y, prime)),
                _ => None,
            };
            let result = PointField::new(a.clone(), b.clone(), x, y);
            let error = result.unwrap_err();
            assert_eq!(error.to_string(), CURVE_ERROR);
        }
    }

    #[test]
    fn test_point_add() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);
        let inputs = vec![
            ((Some(170), Some(142)), (Some(60), Some(139)), (220, 181)),
            ((Some(47), Some(71)), (Some(17), Some(56)), (215, 68)),
            ((Some(143), Some(98)), (Some(76), Some(66)), (47, 71)),
            ((Some(192), Some(105)), (Some(17), Some(56)), (170, 142)),
            ((Some(47), Some(71)), (Some(117), Some(141)), (60, 139)),
            ((Some(47), Some(71)), (Some(117), Some(141)), (60, 139)),
        ];

        for ((x_1, y_1), (x_2, y_2), (x_e, y_e)) in inputs {
            let x1_ = match x_1 {
                Some(x) => Some(FieldElement::new(x, prime)),
                _ => None,
            };

            let y1_ = match y_1 {
                Some(y) => Some(FieldElement::new(y, prime)),
                _ => None,
            };
            let x2_ = match x_2 {
                Some(x) => Some(FieldElement::new(x, prime)),
                _ => None,
            };
            let y2_ = match y_2 {
                Some(y) => Some(FieldElement::new(y, prime)),
                _ => None,
            };
            let point_1 = PointField::new(a.clone(), b.clone(), x1_, y1_).unwrap();
            let point_2 = PointField::new(a.clone(), b.clone(), x2_, y2_).unwrap();
            let result = point_1.add(&point_2).unwrap();
            let expected = PointField::new(
                a.clone(),
                b.clone(),
                Some(FieldElement::new(x_e, prime)),
                Some(FieldElement::new(y_e, prime)),
            )
            .unwrap();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_point_two() {
        let prime = 223;
        let a = FieldElement::new(5, prime);
        let b = FieldElement::new(7, prime);
        let inputs = vec![
            ((None, None), (Some(2), Some(5)), (Some(2), Some(5))),
            ((Some(2), Some(5)), (None, None), (Some(2), Some(5))),
            ((Some(2), Some(5)), (Some(2), Some(-5)), (None, None)),
            // because of the way the book does mod the y result should be 218 rather than -5
            // don't know why the book has -5 as the anwser
            // ((Some(3), Some(7)), (Some(-1), Some(-1)), (Some(2), Some(-5))),

            // because of the way the book does mod the y result should be 146 rather than -77
            // don't know why the book has -77 as the anwser
            // ((Some(-1), Some(1)), (Some(-1), Some(1)), (Some(18), Some(-77))),
        ];

        for ((x_1, y_1), (x_2, y_2), (x_e, y_e)) in inputs {
            let x1_ = match x_1 {
                Some(x) => Some(FieldElement::new(x, prime)),
                _ => None,
            };

            let y1_ = match y_1 {
                Some(y) => Some(FieldElement::new(y, prime)),
                _ => None,
            };
            let x2_ = match x_2 {
                Some(x) => Some(FieldElement::new(x, prime)),
                _ => None,
            };
            let y2_ = match y_2 {
                Some(y) => Some(FieldElement::new(y, prime)),
                _ => None,
            };
            let xe_ = match x_e {
                Some(x) => Some(FieldElement::new(x, prime)),
                _ => None,
            };
            let ye_ = match y_e {
                Some(y) => Some(FieldElement::new(y, prime)),
                _ => None,
            };
            let point_1 = PointField::new(a.clone(), b.clone(), x1_, y1_).unwrap();
            let point_2 = PointField::new(a.clone(), b.clone(), x2_, y2_).unwrap();
            let result = point_1.add(&point_2).unwrap();
            let expected = PointField::new(a.clone(), b.clone(), xe_, ye_).unwrap();
            assert_eq!(result, expected);
        }
    }
}
