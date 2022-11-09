pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let new_start = new_interval[0];
    let new_end = new_interval[1];

    let mut start_idx: i32 = -1;
    let mut end_idx: i32 = -1;
    for (idx, interval) in intervals.iter().enumerate() {
        if new_start >= interval[0] {
            if new_start >= interval[1] {
                continue;
            } else {
                start_idx = idx as i32;
            }

            if new_end >= interval[1] {
                continue;
            } else {
                return intervals;
            }
        } else if new_end >= interval[0] {
            if new_end <= interval[1] {
                end_idx = idx as i32;
            } else {
                continue;
            }
        }

        if end_idx == -1 && new_end <= interval[0] {
            end_idx = idx as i32 - 1;
        }
        if start_idx == -1 && new_start <= interval[0] {
            start_idx = idx as i32 - 1;
        }
    }

    let mut result: Vec<Vec<i32>> = [].to_vec();

    if start_idx == -1 && end_idx == -1 {
        intervals
    } else {
        let mut i = 0;
        while i < intervals.len() {
            let interval: &Vec<i32> = &intervals[i];
            println!("start_idx: {}, end_idx: {}", start_idx, end_idx);
            if i as i32 == start_idx {
                result.push(
                    [
                        intervals[start_idx as usize][0],
                        intervals[end_idx as usize][1],
                    ]
                    .to_vec(),
                );
                while i as i32 != end_idx {
                    i += 1;
                }
            } else {
                result.push(interval.to_vec());
            }
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        let mut inp: Vec<Vec<i32>> = [].to_vec();
        inp.push([1, 3].to_vec());
        inp.push([6, 9].to_vec());
        assert_eq!(insert(inp, [2, 5].to_vec()), [[1, 5], [6, 9]]);
    }

    #[test]
    fn ex2() {
        let inp = [
            [1, 2].to_vec(),
            [3, 5].to_vec(),
            [6, 7].to_vec(),
            [8, 10].to_vec(),
            [12, 16].to_vec(),
        ]
        .to_vec();
        assert_eq!(insert(inp, [4, 8].to_vec()), [[1, 2], [3, 10], [12, 16]]);
    }
}
