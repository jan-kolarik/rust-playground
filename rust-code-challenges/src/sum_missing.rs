fn sum_with_missing1(numbers: Vec<Option<i32>>) -> i32 {
    let mut sum = 0;
    for number in numbers {
        match number {
            Some(n) => sum += n,
            None => (),
        }
    }
    sum
}

fn sum_with_missing2(numbers: Vec<Option<i32>>) -> i32 {
    numbers.iter().filter(|x| x.is_some()).map(|x| x.unwrap()).sum()
}

fn sum_with_missing3(numbers: Vec<Option<i32>>) -> i32 {
    numbers.iter().map(|x| x.unwrap_or_default()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_missing() {
        let data = vec![
            Some(4),
            None,
            Some(1)
        ];
        assert_eq!(5, sum_with_missing1(data.clone()));
        assert_eq!(5, sum_with_missing2(data.clone()));
        assert_eq!(5, sum_with_missing3(data.clone()));
    }

    #[test]
    fn all_missing() {
        let data = vec![
            None,
            None,
            None,
            None
        ];
        assert_eq!(0, sum_with_missing1(data.clone()));
        assert_eq!(0, sum_with_missing2(data.clone()));
        assert_eq!(0, sum_with_missing3(data.clone()));
    }
}