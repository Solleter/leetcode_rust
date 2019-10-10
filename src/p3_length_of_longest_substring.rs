/*
给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度。

示例 1:
输入: "abcabcbb"
输出: 3
解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。

示例 2:
输入: "bbbbb"
输出: 1
解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。

示例 3:
输入: "pwwkew"
输出: 3
解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
     请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。

Given a string, find the length of the longest substring without repeating characters.

Example 1:
Input: "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3.

Example 2:
Input: "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.

Example 3:
Input: "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3.
             Note that the answer must be a substring, "pwke" is a subsequence and not a substring.
*/

use std::collections::HashMap;

fn length_of_longest_substring(s: String) -> i32 {
    let mut char_index: HashMap<char, i32> = HashMap::new();
    let mut index_char: HashMap<i32, char> = HashMap::new();
    let mut index = 0;
    let mut start = 0;
    let mut longest = 0;

    for char in s.chars() {
        if !char_index.contains_key(&char) {
            char_index.insert(char, index);
            index_char.insert(index, char);
        } else {
            let prev_index = char_index[&char];
            let start_remove_index = start;
            let end_remove_index = prev_index + 1;
            start = end_remove_index;

            for i in start_remove_index .. end_remove_index {
                let remove_char = index_char[&i];
                char_index.remove(&remove_char);
                index_char.remove(&i);
            }

            char_index.insert(char, index);
            index_char.insert(index, char);
        }

        let len = (index - start) + 1;
        if len > longest {
            longest = len;
        }

        index += 1;
    }
    longest
}

pub fn solution() {
    let s = "11111".to_string();
    let result = length_of_longest_substring(s);
    println!("Result: {}", result);
}
