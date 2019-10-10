/*
给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。
示例:
给定 nums = [2, 7, 11, 15], target = 9
因为 nums[0] + nums[1] = 2 + 7 = 9
所以返回 [0, 1]

Given an array of integers, return indices of the two numbers such that they add up to a specific target.
You may assume that each input would have exactly one solution, and you may not use the same element twice.
Example:
Given nums = [2, 7, 11, 15], target = 9,
Because nums[0] + nums[1] = 2 + 7 = 9,
return [0, 1].
*/

/*
基本思路:
数组中有两个数相加等于 target, 也就是说，用 target 减去数组中某个数，一定等于数组中另一个数。
遍历数组中每一个数，用 target 减去当前的数，计算出结果来，检查一下这个结果，是否在HashMap中，
如果不在，那就将当前的数，和当前数对应的所引，存入HashMap，在未来，可能是 target 减去某个数的结果。
当 target 减去当前的数，所得结果，在HashMap中时，那就说明找到了答案，
HashMap中的结果对应的所引，和当前数对应的所引，即是返回结果。
*/

use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: Vec<i32> =  Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();
    let len = nums.len();
    for i in 0..len {
        let x = nums[i];
        let index = i as i32;
        let minus_value = target - x;
        if map.contains_key(&minus_value)  {
            result.push(map[&minus_value]);
            result.push(index);
        }
        else{
            map.insert(x, index);
        }
    }

    result
}

pub fn solution() {
    let nums = vec![1, 3, 5, 2, 7];
    let target = 10;
    let result = two_sum(nums, target);
    println!("Result: {:?}", result);
}