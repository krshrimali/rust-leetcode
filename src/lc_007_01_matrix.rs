// Title: 01 Matrix
// Link: https://leetcode.com/problems/01-matrix/

// Couldn't do it myself correctly
// Please refer to: https://leetcode.com/problems/01-matrix/discuss/1370412/rust-bfs-solution
// Credits to him^

use std::collections::VecDeque;

pub fn update_matrix(mat: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (r, c) = (mat.len(), mat[0].len());
    // initializing 2D vector with all i32::MAX values
    // size: r x c (height x width)
    let mut answer = vec![vec![i32::MAX; c]; r];
    let mut vd = VecDeque::new();

    for (i, row) in mat.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if * col == 0 {
                answer[i][j] = 0;
                vd.push_back((i, j));
            }
        }
    }

    while let Some((i, j)) = vd.pop_front() {
        for d in [0, 1, 0, !0, 0].windows(2) {
            let di = i.wrapping_add(d[0]);
            let dj = j.wrapping_add(d[1]);
            if di < r && dj < c && answer[di][dj] > answer[i][j] {
                answer[di][dj] = answer[i][j] + 1;
                vd.push_back((di, dj));
            }
        }
    }

    answer
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let inp: Vec<Vec<i32>> =
            [[0, 0, 0].to_vec(), [0, 1, 0].to_vec(), [1, 1, 1].to_vec()].to_vec();
        let out: Vec<Vec<i32>> =
            [[0, 0, 0].to_vec(), [0, 1, 0].to_vec(), [1, 2, 1].to_vec()].to_vec();
        assert_eq!(update_matrix(&inp), out);
    }

    #[test]
    fn ex2() {
        let inp = [[0,0,0].to_vec(),[0,1,0].to_vec(),[0,0,0].to_vec()].to_vec();
        let out = [[0,0,0],[0,1,0],[0,0,0]];
        assert_eq!(update_matrix(&inp), out);
    }
}
