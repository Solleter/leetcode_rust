use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, usize> = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        let minus_value = target - v;
        if map.contains_key(&minus_value) && map[&minus_value] != i {
            result.push(map[&minus_value] as i32);
            result.push(i as i32);
            break;
        }
        map.insert(*v, i);
    }

    result
}

pub fn solution() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(nums, target);
    println!("Result: {:?}", result);
}