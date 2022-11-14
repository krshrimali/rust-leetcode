use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn k_closest_min_heap(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    // by default, a BinaryHeap is a MaxHeap in Rust
    let mut heap: BinaryHeap<Reverse<Vec<i32>>> = BinaryHeap::new();
    for pt in points.iter() {
        let dist: i32 = (pt[0] * pt[0]) + (pt[1] * pt[1]);
        heap.push(Reverse(vec![dist, pt[0], pt[1]]));
    }

    let mut result: Vec<Vec<i32>> = Vec::new();
    for _ in 0..k {
        let top_elem = heap.pop().unwrap();
        result.push(vec![top_elem.0[1], top_elem.0[2]]);
    }

    result
}

pub fn k_closest_max_heap(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let k = k as usize;
    let mut heap = BinaryHeap::with_capacity(k);
    for pt in points.iter() {
        let dist: i32 = (pt[0] * pt[0]) + (pt[1] * pt[1]);
        heap.push((dist, vec![pt[0], pt[1]]));
        if heap.len() > k {
            heap.pop();
        }
    }

    // https://leetcode.com/problems/k-closest-points-to-origin/discuss/2003312/Rust-BinaryHeap
    // Full credits to the guy above for this max heap function... was just trying to see how
    // he used .map(...).collect(), neat but maybe not too readable (but I still wanted to learn)
    heap.into_iter().map(|(_, coord)| coord).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1_k_closest_points_to_origin_max_heap() {
        let inp: Vec<Vec<i32>> = vec![vec![1, 3], vec![-2, 2]];
        let expected_out: Vec<Vec<i32>> = vec![vec![-2, 2]];
        assert_eq!(k_closest_max_heap(inp, 1), expected_out);
    }

    #[test]
    fn ex2_k_closest_points_to_origin_max_heap() {
        let inp = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let mut expected_out = vec![vec![3, 3], vec![-2, 4]];
        expected_out.sort();
        assert_eq!(k_closest_max_heap(inp, 2), expected_out);
    }

    #[test]
    fn ex1_k_closest_points_to_origin_min_heap() {
        let inp: Vec<Vec<i32>> = vec![vec![1, 3], vec![-2, 2]];
        let expected_out: Vec<Vec<i32>> = vec![vec![-2, 2]];
        assert_eq!(k_closest_min_heap(inp, 1), expected_out);
    }

    #[test]
    fn ex2_k_closest_points_to_origin_min_heap() {
        let inp = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let expected_out = vec![vec![3, 3], vec![-2, 4]];
        assert_eq!(k_closest_min_heap(inp, 2), expected_out);
    }
}
