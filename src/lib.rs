pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let nums_multiset: HashMap<i32, (usize, Option<usize>)> = nums
            .into_iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (index, item)| {
                acc.entry(item)
                    .and_modify(|entry| entry.1 = Some(index))
                    .or_insert((index, None));

                acc
            });

        for (n, (i1, i2)) in &nums_multiset {
            let rest = target - n;
            if *n == rest {
                if let Some(j) = i2 {
                    return vec![*i1 as i32, *j as i32];
                }
            } else {
                if let Some((j, _)) = nums_multiset.get(&rest) {
                    return vec![*i1 as i32, *j as i32];
                }
            }
        }

        return vec![];
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![3,3], 6, vec![0,1])]
    #[case(vec![2,7,11,15], 9, vec![0,1])]
    #[case(vec![3,2,4], 6, vec![1,2])]
    fn test(#[case] nums: Vec<i32>, #[case] target: i32, #[case] result: Vec<i32>) {
        assert_eq!(
            Solution::two_sum(nums, target)
                .into_iter()
                .collect::<std::collections::HashSet<_>>(),
            std::collections::HashSet::from_iter(result.into_iter()),
        );
    }
}
