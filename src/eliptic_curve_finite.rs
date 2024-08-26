use crate::field_element::mod_it;
use anyhow::Result;

fn confirm_on_curve(x: isize, y: isize, b: isize, base: isize) -> bool {
    let right = mod_it(x.pow(3) + b, base);
    let left = mod_it(y.pow(2), base);
    left == right
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_helper(x: isize, y: isize) -> bool {
        confirm_on_curve(x, y, 7, 223)
    }

    #[test]
    fn test_confirm_on_curve() {
        let values_to_test = vec![
            (192, 105, true),
            (17, 56, true),
            (200, 119, false),
            (1, 193, true),
            (42, 99, false),
        ];
        for (x, y, expected) in values_to_test {
            let result = test_helper(x, y);
            assert_eq!(expected, result);
        }
    }
}
