// Title: Longest Substring without Repeating Characters
// Link: https://leetcode.com/problems/longest-substring-without-repeating-characters/

use std::cmp;
use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let s_bytes = s.as_bytes();

    let mut cur_window_map: HashMap<u8, i32> = HashMap::new();
    let mut left: i32 = 0;
    let mut right: i32 = 0;

    let mut result: i32 = 0;

    while (left as usize) < s.len() && (right as usize) < s.len() {
        let char_string: &u8 = &s_bytes[right as usize];

        if cur_window_map.contains_key(char_string) {
            let new_left = cur_window_map[char_string] + 1;
            for range_idx in left..new_left {
                let to_remove = &s_bytes[range_idx as usize];
                if cur_window_map.contains_key(to_remove) {
                    cur_window_map.remove_entry(to_remove);
                }
            }
            left = new_left;
            continue;
        }

        cur_window_map.insert(*char_string, right);

        result = cmp::max(right - left + 1, result);
        right += 1;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1_length_of_longest_substring_wo_repeating_chars() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn ex2_length_of_longest_substring_wo_repeating_chars() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn ex3_length_of_longest_substring_wo_repeating_chars() {
        assert_eq!(length_of_longest_substring("abccbdefgh".to_string()), 7);
    }

    #[test]
    fn ex4_length_of_longest_substring_wo_repeating_chars() {
        assert_eq!(length_of_longest_substring("dvdf".to_string()), 3);
    }
}
