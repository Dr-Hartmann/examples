pub fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, iv) in nums.iter().enumerate() {
        for (j, jv) in nums.iter().enumerate().skip(i + 1) {
            if let Some(sum) = iv.checked_add(*jv) {
                if sum == target {
                    return vec![i as i32, j as i32];
                }
            } else {
                panic!("Переполнение")
            }
        }
    }
    vec![]
}

use std::collections::HashMap;
pub fn two_sum_hash_table(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&complement_index) = map.get(&(target - num)) {
            return vec![complement_index as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum() {
        let res = two_sum_hash_table(vec![2, 7, 11, 15, i32::MAX], 22);
        assert_eq!(vec![1, 3], res);

        let res = two_sum_brute_force(vec![-100, -50, 0, 10, 50, 100, 200], 50);
        assert_eq!(vec![1, 5], res);

        let res = two_sum_hash_table(vec![3, 5, 2, 5, 8, 5], 10);
        assert_eq!(vec![1, 3], res);

        let res = two_sum_hash_table(
            vec![
                2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
                131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432,
                67108864, 134217728, 268435456, 536870912, 1073741824, 7,
            ],
            9,
        );
        assert_eq!(vec![0, 30], res);
    }
}
