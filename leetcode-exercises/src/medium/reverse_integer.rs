fn reverse_string(num: i32) -> i32 {
    let abs_num = num.abs();
    let num_str: String = abs_num.to_string().chars().rev().collect();
    let rev_abs_num = num_str.parse::<i32>();
    if rev_abs_num.is_err() {
        return 0;
    }
    rev_abs_num.unwrap() * num.signum()
}

fn reverse_stack(num: i32) -> i32 {
    let mut abs_num = num.abs();
    let mut digits = Vec::new();

    while abs_num != 0 {
        let rem = abs_num % 10;
        digits.push(rem);
        abs_num = (abs_num - rem) / 10;
    }

    let mut i = 0;
    let mut result: i32 = 0;

    while let Some(digit) = digits.pop() {
        match result.checked_add(digit * 10i32.pow(i)) {
            Some(val) => result = val,
            _ => return 0
        }
        i += 1;
    }

    result * num.signum()
}

fn reverse_maths(num: i32) -> i32 {
    let abs_num = num.abs();
    if abs_num < 10 {
        return num;
    }

    let digits = abs_num.ilog10();

    let mut sum: i32 = 0;
    for k in 1..digits+1 {
        sum += abs_num / 10i32.pow(k) * 10i32.pow(digits - k);
    }

    (abs_num * 10i32.pow(digits) - 99 * sum) * num.signum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_reverse() {
        assert_eq!(0, reverse_string(0));
        assert_eq!(0, reverse_stack(0));
        assert_eq!(0, reverse_maths(0));
    }

    #[test]
    fn one_digit_reverse() {
        assert_eq!(5, reverse_string(5));
        assert_eq!(5, reverse_stack(5));
        assert_eq!(5, reverse_maths(5));
    }

    #[test]
    fn two_digit_reverse() {
        assert_eq!(23, reverse_string(32));
        assert_eq!(23, reverse_stack(32));
        assert_eq!(23, reverse_maths(32));
    }

    #[test]
    fn three_digit_reverse() {
        assert_eq!(321, reverse_string(123));
        assert_eq!(321, reverse_stack(123));
        assert_eq!(321, reverse_maths(123));
    }

    #[test]
    fn leading_zero_reverse() {
        assert_eq!(547, reverse_string(7450));
        assert_eq!(547, reverse_stack(7450));
        assert_eq!(547, reverse_maths(7450));
    }

    #[test]
    fn minus_reverse() {
        assert_eq!(-236, reverse_string(-632));
        assert_eq!(-236, reverse_stack(-632));
        assert_eq!(-236, reverse_maths(-632));
    }
}
