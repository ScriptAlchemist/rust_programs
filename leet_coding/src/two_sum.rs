use std::collections::HashMap;

#[allow(dead_code)]
pub fn v1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if map.contains_key(&complement) {
            return vec![*map.get(&complement).unwrap(), i as i32];
        }
        map.insert(*num, i as i32);
    }
    vec![]
}

#[cfg(test)]
mod v1_tests {
    use super::v1;
    #[test]
    fn test_v1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = v1(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
    #[test]
    fn test_v1_empty() {
        let nums = vec![];
        let target = 9;
        let result = v1(nums, target);
        assert_eq!(result, vec![]);
    }
}

#[allow(dead_code)]
pub fn v2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, n) in nums.iter().enumerate() {
        if let Some(x) = map.get(&(target-n)) {
            return vec![*x, i as i32];
        }
        map.insert(n, i as i32);
    }
    unreachable!("No solution!");
}

#[cfg(test)]
mod v2_tests {
    use super::v2;
    #[test]
    fn test_v2() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = v2(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
    #[test]
    #[should_panic(expected = "No solution!")]
    fn test_v2_empty() {
        let nums = vec![];
        let target = 9;
        v2(nums, target);
    }
}
