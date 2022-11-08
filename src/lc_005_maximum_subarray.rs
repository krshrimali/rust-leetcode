// Status: NOT WORKING
use std::cmp;

pub fn cumsum(left: &mut i32, right: &mut i32, nums: &[i32]) -> i32 {
    let mut sum = 0;
    let mut mut_left = *left;

    // if mut_left >= nums.len() as i32 {
    //     return i32::MIN;
    // }

    while mut_left <= *right {
        if mut_left >= nums.len() as i32 {
            break;
        }
        sum += nums[mut_left as usize];
        mut_left += 1;
    }
    sum
}

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sum: i32 = i32::MIN;
    let mut left: i32 = 0;
    let mut right: i32 = 0;
    let mut sum = 0;

    while right < nums.len() as i32 {
        if left >= nums.len() as i32 {
            break;
        }

        sum += nums[right as usize];

        max_sum = cmp::max(max_sum, sum);
        if sum < 0 {
            if left == right {
                right += 1;
            }
            sum -= nums[left as usize];
            left += 1;
        } else {
            right += 1;
        }
    }

    max_sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(max_sub_array([-2, 1, -3, 4, -1, 2, 1, -5, 4].to_vec()), 6);
    }

    #[test]
    fn ex2() {
        assert_eq!(max_sub_array([1].to_vec()), 1);
    }

    #[test]
    fn ex3() {
        assert_eq!(max_sub_array([5, 4, -1, 7, 8].to_vec()), 23);
    }

    #[test]
    fn ex4() {
        assert_eq!(max_sub_array([-2, -1, -3, -4].to_vec()), -1);
    }
}
