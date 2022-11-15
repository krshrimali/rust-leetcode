// Title: Best Time to Buy and Sell Stock
// Link: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

// it's a maximum difference between 2 elements problem
// with a condition that the 2 elements should be: a..b and considering b..a is not allowed
use std::cmp;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_diff = 0;
    let mut min_price = i32::MAX;

    for price in prices.iter() {
        min_price = cmp::min(min_price, *price);
        max_diff = cmp::max(*price - min_price, max_diff);
    }

    max_diff
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(max_profit([7, 6, 4, 3, 1].to_vec()), 0);
    }

    #[test]
    fn ex2() {
        assert_eq!(max_profit([7, 1, 5, 3, 6, 4].to_vec()), 5);
    }
}
