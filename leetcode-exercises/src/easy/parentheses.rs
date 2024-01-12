use std::collections::HashMap;

fn is_valid(s: &str) -> bool {
    let brackets = HashMap::from([
        (')', '('),
        (']', '['),
        ('}', '{'),
    ]);

    let mut stack = Vec::new();
    for letter in s.chars() {
        match brackets.get(&letter) {
            Some(value) => {
                let last_bracket = stack.pop();
                if last_bracket.is_none() {
                    return false;
                }
                if last_bracket.unwrap() != *value {
                    return false;
                }
            },
            None => stack.push(letter)
        }
    }

    return stack.is_empty();
}

fn is_valid_faster(s: &str) -> bool {
    let mut stack = Vec::new();
    for letter in s.chars() {
        match letter {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            _ => if Some(letter) != stack.pop() {
                return false;
            }
        }
    }

    return stack.is_empty();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_parentheses() {
        assert!(is_valid("()"));
        assert!(is_valid_faster("()"));
    }

    #[test]
    fn more_parentheses() {
        assert!(is_valid("()[]{}"));
        assert!(is_valid_faster("()[]{}"));
    }

    #[test]
    fn complex_parentheses() {
        assert!(is_valid("(()((())[]){})"));
        assert!(is_valid_faster("(()((())[]){})"));
    }

    #[test]
    fn invalid_pair() {
        assert!(!is_valid("(]"));
        assert!(!is_valid_faster("(]"));
    }

    #[test]
    fn invalid_not_closed() {
        assert!(!is_valid("(((((([[[[[[[{{{{{"));
        assert!(!is_valid_faster("(((((([[[[[[[{{{{{"));
    }
}
