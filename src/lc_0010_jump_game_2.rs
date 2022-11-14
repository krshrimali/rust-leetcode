use std::cmp;

pub fn jump(nums: Vec<i32>) -> i32 {
    let mut cur_reach: i32 = 0;
    let mut cur_max: i32 = 0;
    let mut jumps_count: i32 = 0;

    for (idx, val) in nums.iter().enumerate().take(nums.len() - 1) {
        cur_max = cmp::max(cur_max, idx as i32 + val);
        if idx as i32 == cur_reach {
            jumps_count += 1;
            cur_reach = cur_max;
        }
    }

    jumps_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1_jump_game_2() {
        assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn ex2_jump_game_2() {
        assert_eq!(jump(vec![2, 3, 0, 1, 4]), 2);
    }
}
