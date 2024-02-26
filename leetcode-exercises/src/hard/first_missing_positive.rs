const VALUE_OUT_OF_BOUNDS: i32 = i32::max_value();

// it must be O(n) time and O(1) space
fn first_missing_positive(mut numbers: Vec<i32>) -> i32 {
    for i in 0..numbers.len() {
        if numbers[i] < 1 || numbers[i] > numbers.len() as i32 {
            numbers[i] = VALUE_OUT_OF_BOUNDS;
        }
    }

    for i in 0..numbers.len() {
        let number = numbers[i].abs();
        if number != VALUE_OUT_OF_BOUNDS {
            numbers[number as usize - 1] = -numbers[number as usize - 1].abs()
        }
    }

    for i in 0..numbers.len() {
        if numbers[i] > 0 {
            return i as i32 + 1;
        }
    }

    numbers.len() as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raising_then_zero() {
        assert_eq!(3, first_missing_positive(vec![1,2,0]));
    }

    #[test]
    fn with_negative() {
        assert_eq!(2, first_missing_positive(vec![3,4,-1,1]));
    }

    #[test]
    fn raising_without_start() {
        assert_eq!(1, first_missing_positive(vec![7,8,9,11,12]));
    }

    #[test]
    fn just_one() {
        assert_eq!(2, first_missing_positive(vec![1]));
    }
}
