pub struct Solution {}

impl Solution {
    pub fn two_sum() {
        let fun = two_sum_brute_force;
        // let fun = two_sum_hash_table1;

        let start = std::time::Instant::now();
        let mut res = vec![];
        let mut count = 0;
        while count < 100_000 {
            count += 1;
            res = fun(vec![2, 7, 11, 15], 9);
        }
        eprintln!("{:#?}", start.elapsed() / count);
        println!("{res:?}");
    }
}

fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
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

fn two_sum_hash_table(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::with_capacity(nums.len());
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&complement_index) = map.get(&(target - num)) {
            return vec![complement_index as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}
