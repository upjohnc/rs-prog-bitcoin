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

#[derive(Debug)]
struct Point {
    a: FieldElement,
    b: FieldElement,
    x: FieldElement,
    y: FieldElement,
}

impl Point {
    fn new(a: FieldElement, b: FieldElement, x: FieldElement, y: FieldElement) -> Result<Self> {
        if !confirm_on_curve(x.clone(), y.clone(), a.clone(), b.clone())? {
            return Err(anyhow!("Cannot be on the curve"));
        }
        Ok(Self { a, b, x, y })
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
            let result = Point::new(
                a.clone(),
                b.clone(),
                FieldElement::new(x, prime_base),
                FieldElement::new(y, prime_base),
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

        let values_to_test = vec![(192, 105, true), (17, 56, true), (1, 193, true)];
        for (x, y, expected) in values_to_test {
            let result = Point::new(
                a.clone(),
                b.clone(),
                FieldElement::new(x, prime_base),
                FieldElement::new(y, prime_base),
            );
            assert!(result.is_ok());
        }
    }
}
