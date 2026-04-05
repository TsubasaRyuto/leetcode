fn main() {
    println!("Hello, world!");
}

struct Solution {}
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for (i, &value) in nums2.iter().enumerate() {
            nums1[i + (m as usize)] = value;
        }
        nums1.sort();
    }
}

// MEMO:
// Iterate through nums2 from index 0 to n-1. For each element, update the value at nums1[index + m]
// with the current value from nums2.
// Finally, sorting nums1 will produce the merged result.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_case1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, [1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_merge_case2() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];
        Solution::merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(nums1, [1]);
    }

    #[test]
    fn test_merge_case3() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        Solution::merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(nums1, [1]);
    }
}
