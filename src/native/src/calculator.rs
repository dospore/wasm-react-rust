use wasm_bindgen::prelude::*;

/**
 * Recursive function to handle expressions 
 */
fn handle_expression(s: &str) -> u64 {
    let mut s = s.to_owned();
    while let Some(opening) = s.find('(') {
        // loop through each char and find corresponding closing parens
        // we cant just use find since there are many cases this will return the
        // wrong indices
        let starting_index = opening + 1;
        let mut chars = s[starting_index..].chars().enumerate();
        let mut opening_count = 0;
        let mut j = starting_index;
        while let Some((count, c)) = chars.next() {
            match c {
                '(' => {
                    opening_count = opening_count + 1;
                },
                ')' => {
                    if opening_count == 0 {
                        j = j + count;
                        break;
                    } else {
                        opening_count = opening_count - 1;
                    }
                },
                _ => ()
            }
        }
        let middle = handle_expression(&s[starting_index..j]);
        s = format!("{}{}{}", &s[..opening], middle, &s[j+1..])
    }

    // bubbles backwards, this recursive down to the multiply and computes that first
    let subtract = s.find('-');
    return match subtract {
        Some(i) => handle_expression(&s[0..i]) - handle_expression(&s[i+1..]),
        None => {
            let add = s.find('+');
            return match add {
                Some(i) => handle_expression(&s[0..i]) + handle_expression(&s[i+1..]),
                None => {
                    let divide = s.find('/');
                    match divide {
                        Some(i) => handle_expression(&s[0..i]) / handle_expression(&s[i+1..]),
                        None => {
                            let multiply = s.find('*');
                            return match multiply {
                                Some(i) => handle_expression(&s[0..i]) * handle_expression(&s[i+1..]),
                                None => {
                                    let trimmed_string: String = s.chars().filter(|c| !c.is_whitespace()).collect();
                                    match trimmed_string.parse::<u64>() {
                                        Ok(n) => n,
                                        Err(_e) => {
                                            0
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn calculate(s: &str) -> String {
    handle_expression(s).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_calculate() {
        assert_eq!(String::from("0"), calculate(""));
    }

    #[test]
    fn no_calculate() {
        assert_eq!(String::from("2"), calculate("2"));
    }

    #[test]
    fn simple_addition_calc() {
        assert_eq!(String::from("4"), calculate("2+2"));
    }
    #[test]
    fn simple_multiplication_calc() {
        assert_eq!(String::from("4"), calculate("2 * 2"));
    }
    #[test]
    fn simple_subtraction_calc() {
        assert_eq!(String::from("0"), calculate("2 - 2"));
    }
    #[test]
    fn addition_multiplication_subtraction() {
        assert_eq!(String::from("30"), calculate("2 + 10 * 3 - 2"));
    }
    #[test]
    fn multiplication_addition_subtraction() {
        assert_eq!(String::from("14"), calculate("2 * 3 + 10 - 2"));
    }
    #[test]
    fn multiplication_addition_subtraction_division() {
        assert_eq!(String::from("9"), calculate("2 * 3 + 10 / 2 - 2"));
    }
    #[test]
    fn double_multiplication() {
        assert_eq!(String::from("8"), calculate("2*2*2"));
    }
    #[test]
    fn simple_calc_with_space() {
        assert_eq!(String::from("4"), calculate("2 + 2"));
    }
    #[test]
    fn simple_calc_with_parens() {
        assert_eq!(String::from("8"), calculate("(2 + 2) * 2"));
    }

    #[test]
    fn complex_calc_with_parens() {
        assert_eq!(String::from("1"), calculate("(12) / (12) "));
        assert_eq!(String::from("1"), calculate("12 / 12"));
        assert_eq!(String::from("1"), calculate("12 / ((2 + 2) * 2 + 4)"));
        assert_eq!(String::from("6"), calculate("((2 + 2) * 2 + 4) - 6"));
        assert_eq!(String::from("18"), calculate("((2 + 2) * 2 + 4) + 6"));
        assert_eq!(String::from("12"), calculate("((2 + 2) * 2 + 4) + 6 - 6"));
        assert_eq!(String::from("1"), calculate("((2 + 2) * 2 + 4) / 12"));
        assert_eq!(String::from("10"), calculate("((5 * 2) + 0) / 1"));
        assert_eq!(String::from("1"), calculate("((5 * 2) + 2) / 12"));
        assert_eq!(String::from("1"), calculate("12 / ((2 + 2) * 2 + 4)"));
        assert_eq!(String::from("36"), calculate("(6) * (5 + 1)"));
        assert_eq!(String::from("1"), calculate("(6) / (5 + 1)"));
        assert_eq!(String::from("2"), calculate("((2 + 2) * 2 + 4) / (5 + 1)"));
        assert_eq!(String::from("1"), calculate("((2 + 2) * 2 + 4) / ((5 + 1) * 2)"));
    }

    #[test]
    fn calc_with_parens() {
        assert_eq!(String::from("12"), calculate("((2 + 2) * 2 + 4)"));
        assert_eq!(String::from("12"), calculate("(5 + 1) * 2"));
    }
}

