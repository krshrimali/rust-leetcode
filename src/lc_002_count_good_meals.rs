use std::collections::HashMap;

pub fn is_factor_of(
    mut sum: i32,
    _idx1: i32,
    _idx2: i32,
    record_map: &mut HashMap<i32, bool>,
    factor_of: i32,
) -> bool {
    let mut visited = vec![];
    if record_map.contains_key(&sum) {
        record_map[&sum]
    } else {
        while sum != 0 {
            if sum != 1 && sum % factor_of != 0 {
                return false;
            } else {
                visited.push(sum);
                sum /= factor_of;
                if record_map.contains_key(&sum) {
                    break;
                } else {
                    continue;
                }
            }
        }
        for val in visited.iter() {
            record_map.insert(*val, true);
        }
        true
    }
}

pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
    let mut record_map = HashMap::<i32, bool>::new();
    let mut count: i32 = 0;
    for i in 0..deliciousness.len() {
        for j in i + 1..deliciousness.len() {
            let sum = deliciousness[i] + deliciousness[j];
            if sum == 0 {
                continue;
            } else if is_factor_of(sum, i as i32, j as i32, &mut record_map, 2) {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(count_pairs(vec![1, 3, 5, 7, 9]), 4);
    }
}
