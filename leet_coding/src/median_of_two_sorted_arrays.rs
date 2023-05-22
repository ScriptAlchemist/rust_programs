#[allow(dead_code)]
enum MedianKind {
    Even(i32, i32),
    Odd(i32),
}

fn median(median_kind: MedianKind) -> f64 {
    match median_kind {
        MedianKind::Even(a, b) => (a as f64 + b as f64) / 2.0,
        MedianKind::Odd(a) => a as f64,
    }
}

#[allow(dead_code)]
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let (mut nums1_idx, mut nums2_idx, mut loop_idx)= (0, 0, 0);
    let max_l = nums1.len() + nums2.len();
    let is_even = max_l % 2 == 0;

    let max_loops = match is_even {
        true => max_l / 2 + 1,
        false => (max_l / 2) + 1,
    };

    let mut last_num = 0;
    let mut second_last_num = 0;
    while loop_idx < max_loops {
        second_last_num = last_num;
        if nums1_idx < nums1.len() && (nums2_idx == nums2.len() || nums1[nums1_idx] < nums2[nums2_idx]) {
            last_num = nums1[nums1_idx];
            nums1_idx += 1;
        } else if nums2_idx < nums2.len() {
            last_num = nums2[nums2_idx];
            nums2_idx += 1;
        }
        loop_idx += 1;
    }

    match is_even {
        true => median(MedianKind::Even(second_last_num, last_num)),
        false => median(MedianKind::Odd(last_num)),
    }
}


#[cfg(test)]
mod finding_median_sorted_array {
    use super::*;

    #[test]
    fn test_1() {
        let nums1 = vec![1,3];
        let nums2 = vec![2];
        let output = 2 as f64;
        // merged array = [1,2,3] and median is 2
        assert_eq!(find_median_sorted_arrays(nums1, nums2), output);
    }

    #[test]
    fn test_2() {
        let nums1 = vec![1,2];
        let nums2 = vec![3,4];
        let output = 2.5 as f64;
        // merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5
        assert_eq!(find_median_sorted_arrays(nums1, nums2), output);
    }
}
