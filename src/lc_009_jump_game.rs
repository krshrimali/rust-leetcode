// Title: Jump Game
// Link: https://leetcode.com/problems/jump-game/

pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut curr_idx = nums.len() - 1;

    for right_idx in (0..nums.len()).rev() {
        if right_idx as i32 + nums[right_idx] >= curr_idx as i32 {
            curr_idx = right_idx;
        }
    }

    curr_idx == 0
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1_jump_game() {
        let inp: Vec<i32> = vec![2,3,1,1,4];
        assert!(can_jump(inp));
    }

    #[test]
    fn ex2_jump_game() {
        let inp: Vec<i32> = vec![3,2,1,0,4];
        assert!(!can_jump(inp));
    }
}
