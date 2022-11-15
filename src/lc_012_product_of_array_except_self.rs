// Title: Product of Array except Self
// Link: https://leetcode.com/problems/product-of-array-except-self

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return nums;
    }

    let mut left_mul: Vec<i32> = vec![0; nums.len()];
    let mut right_mul: Vec<i32> = vec![0; nums.len()];

    left_mul[0] = 1;
    for idx in 1..nums.len() {
        left_mul[idx] = left_mul[idx - 1] * nums[idx - 1];
    }

    right_mul[nums.len()-1] = 1;
    for idx in (0..nums.len() - 1).rev() {
        right_mul[idx] = right_mul[idx + 1] * nums[idx + 1];
    }

    left_mul.iter().zip(right_mul.iter()).map(|(x, y)| x * y).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1_product_except_self() {
        assert_eq!(product_except_self(vec![1, -1, 2, 0]), vec![0, 0, 0, -2]);
    }

    #[test]
    fn ex2_product_except_self() {
        assert_eq!(product_except_self(vec![0, 0, 0, 0]), vec![0, 0, 0, 0])
    }

    #[test]
    fn ex3_product_except_self() {
        assert_eq!(product_except_self(vec![]), vec![])
    }

    #[test]
    fn ex4_product_except_self() {
        assert_eq!(product_except_self(vec![1, 4, 3, 6]), vec![72, 18, 24, 12]);
    }
}
