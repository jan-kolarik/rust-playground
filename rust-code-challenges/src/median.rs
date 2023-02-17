fn median(mut list: Vec<f64>) -> Option<f64> {
    list.sort_by(|x, y| x.partial_cmp(y).unwrap());

    match list.len() {
        0 => None,
        n if n % 2 == 1 => {
            let middle_index = (list.len() - 1) / 2;
            Some(list[middle_index])
        },
        _ => {
            let left_middle_index = list.len() / 2 - 1;
            let right_middle_index = list.len() / 2;
            Some((list[left_middle_index] + list[right_middle_index]) / 2.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_size_vector() {
        let list = vec![1.0, 4.0, 5.0];
        assert_eq!(median(list), Some(4.0));
    }

    #[test]
    fn even_size_vector() {
        let list = vec![1.5, 3.0, 5.0, 8.8];
        assert_eq!(median(list), Some(4.0));
    }

    #[test]
    fn empty_vector() {
        let list = vec![];
        assert_eq!(median(list), None);
    }
}