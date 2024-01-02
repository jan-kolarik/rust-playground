use std::collections::HashMap;

fn two_sum_naive(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() - 1 {
        for j in i+1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }            
        }
    }
    panic!()
}

// use the hash map to store indexes of the numbers
// that could be added up with their supplements to obtain
// the resulting target
fn two_sum_hashed(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map = HashMap::with_capacity(nums.len());
    for (i, num) in nums.iter().enumerate() {
        match hash_map.get(num) {
            Some(j) => return vec![i as i32, *j as i32],
            None => {
                hash_map.insert(target - num, i);
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_two_numbers() {
        assert_eq!(vec![0, 1], two_sum_naive(vec![1, 1], 2));

        let mut hashed_indexes = two_sum_hashed(vec![1, 1], 2);
        hashed_indexes.sort();
        assert_eq!(vec![0, 1], hashed_indexes);
    }

    #[test]
    fn sum_more_numbers() {
        assert_eq!(vec![1, 2], two_sum_naive(vec![3, 2, 4], 6));

        let mut hashed_indexes = two_sum_hashed(vec![3, 2, 4], 6);
        hashed_indexes.sort();
        assert_eq!(vec![1, 2], hashed_indexes);
    }

    #[test]
    fn sum_even_more_numbers() {
        assert_eq!(vec![0, 1], two_sum_naive(vec![2, 7, 11, 15], 9));

        let mut hashed_indexes = two_sum_hashed(vec![2, 7, 11, 15], 9);
        hashed_indexes.sort();
        assert_eq!(vec![0, 1], hashed_indexes);
    }
}
