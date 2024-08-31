use crate::eliptic_curves::CURVE_ERROR;
use crate::field_element::FieldElement;
use anyhow::{anyhow, Result};

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

#[derive(Debug, PartialEq)]
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

    fn add(&self, other: &Self) -> bool {
        // fn add(self, other: Self) -> Self {
        if self.x.is_none() {
            // return other;
            return false;
        }
        //     if other.x.is_none() {
        //         return self;
        //     }
        //     if self.x == other.x && self.y != other.y {
        //         return Self::new_infinity(self.a, self.b);
        //     }
        //     if self.x != other.x {
        //         let self_x = self.x.unwrap();
        //         let other_x = other.x.unwrap();
        //         let self_y = self.y.unwrap();
        //         let other_y = other.y.unwrap();
        //         let slope = (other_y - self_y) / (other_x - self_x);
        //         let x_3 = slope.pow(2) - self_x - other_x;
        //         let y_3 = slope * (self_x - x_3) - self_y;
        //         return Self::new(x_3, y_3, self.a, self.b).unwrap();
        //     }
        //     if self == other && self.y.unwrap() == 0 {
        //         return Self::new_infinity(self.a, self.b);
        //     }
        //     if self == other {
        //         let x_1 = self.x.unwrap();
        //         let y_1 = self.y.unwrap();
        //         let slope = (3 * x_1.pow(2) + self.a) / (2 * y_1);
        //         let x_3 = slope.pow(2) - 2 * x_1;
        //         let y_3 = slope * (x_1 - x_3) - y_1;
        //         return Self::new(x_3, y_3, self.a, self.b).unwrap();
        //     }

        //     self
        // }
        true
    }
}

#[cfg(test)]
mod tests {
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
    // #[test]
    // fn test_point_add() {
    //     let prime = 223;
    //     let a = FieldElement::new(0, prime);
    //     let b = FieldElement::new(7, prime);
    //     let inputs = vec![
    //         ((Some(170), Some(142)), (Some(60), Some(139))),
    //         ((Some(47), Some(71)), (Some(17), Some(56))),
    //         ((Some(143), Some(98)), (Some(76), Some(66))),
    //     ];

    //     for ((x_1, y_1), (x_2, y_2)) in inputs {
    //         let x1_ = match x_1 {
    //             Some(x) => Some(FieldElement::new(x, prime)),
    //             _ => None,
    //         };

    //         let y1_ = match y_1 {
    //             Some(y) => Some(FieldElement::new(y, prime)),
    //             _ => None,
    //         };
    //         let x2_ = match x_2 {
    //             Some(x) => Some(FieldElement::new(x, prime)),
    //             _ => None,
    //         };
    //         let y2_ = match y_2 {
    //             Some(y) => Some(FieldElement::new(y, prime)),
    //             _ => None,
    //         };
    //         let point_1 = PointField::new(a.clone(), b.clone(), x1_, y1_).unwrap();
    //         let point_2 = PointField::new(a.clone(), b.clone(), x2_, y2_).unwrap();
    //         let result = point_1.add(&point_2);
    //         assert_eq!(result, false);
    //     }
    // }
}
