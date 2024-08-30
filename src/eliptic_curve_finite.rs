use crate::field_element::mod_it;
use crate::field_element::FieldElement;
use anyhow::Result;

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

struct Point {
    a: FieldElement,
    b: FieldElement,
    x: FieldElement,
    y: FieldElement,
}

impl Point {
    fn new(a: FieldElement, b: FieldElement, x: FieldElement, y: FieldElement) -> Self {
        todo!("add check for on the curve x y");
        Self { a, b, x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confirm_on_curve() {
        let prime_base = 223;
        let a = FieldElement {
            prime: prime_base,
            number: 0,
        };
        let b = FieldElement {
            number: 7,
            prime: prime_base,
        };

        let values_to_test = vec![
            (192, 105, true),
            (17, 56, true),
            (200, 119, false),
            (1, 193, true),
            (42, 99, false),
        ];
        for (x, y, expected) in values_to_test {
            let x_f = FieldElement {
                number: x,
                prime: prime_base,
            };
            let y_f = FieldElement {
                number: y,
                prime: prime_base,
            };
            let result = confirm_on_curve(x_f, y_f, a.clone(), b.clone()).unwrap();
            assert_eq!(expected, result);
        }
    }
}
