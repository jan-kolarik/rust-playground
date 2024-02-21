use std::cmp;

fn median_of_sorted_array(nums: &[i32]) -> f64 {
    let len = nums.len();
    if len % 2 != 0 {
        nums[(len + 1) / 2] as f64
    } else {
        (nums[len / 2] + nums[len / 2 + 1]) as f64 / 2.0
    }
}

// slices are expected of the same size
fn find_median_sorted_slices(nums1: &[i32], nums2: &[i32]) -> f64 {
    let n = nums1.len();

    if n == 0 {
        return 0.0;
    }

    if n == 1 {
        return (nums1[0] + nums2[0]) as f64 / 2.0;
    }

    if n == 2 {
        return (cmp::max(nums1[0], nums2[0]) + cmp::min(nums1[1], nums2[1])) as f64 / 2.0;
    }

    let median1 = median_of_sorted_array(nums1);
    let median2 = median_of_sorted_array(nums2);

    match median1.total_cmp(&median2) {
        std::cmp::Ordering::Equal => median1,
        std::cmp::Ordering::Greater => find_median_sorted_slices(&nums1[0 .. n / 2], &nums2[n / 2 .. nums2.len() - 1]),
        std::cmp::Ordering::Less => find_median_sorted_slices(&nums1[n / 2 .. nums1.len() - 1], &nums2[0 .. n / 2])
    }
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    find_median_sorted_slices(&nums1, &nums2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_arrays() {
        assert_eq!(1.5, find_median_sorted_arrays(vec![1], vec![2]));
        assert_eq!(2.5, find_median_sorted_arrays(vec![1,3], vec![2,4]));
        assert_eq!(1.0, find_median_sorted_arrays(vec![1,1,1], vec![1,1,1]));
    }
}
