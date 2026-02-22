fn main() {
    println!("Hello, world!");
}

struct Solution {}
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums: &[i32] = &nums;
        let mut idx = 0;

        if nums.is_empty() {
            return 0;
        }

        while nums.len() > 1 {
            let half_idx = nums.len() / 2;
            let halfed_first = &nums[0..half_idx];
            let halfed_second = &nums[half_idx..];

            // Compare the last value with the first half
            if target > halfed_first[halfed_first.len() - 1] {
                idx += halfed_first.len();
                nums = halfed_second;
            } else {
                nums = halfed_first;
            }
        }

        if target > nums[0] {
            return (idx + 1) as i32;
        } else {
            return idx as i32;
        }
    }
}

// Divide the nums array into halves to find the insertion position.
// 1. Divide the sorted array into two halves.
// 2. Compare the target with the last element of the first half.
// 3. If the target is greater, narrow the search to the second half;
//    otherwise, proceed with the first half.
// 4. Repeat until a single element remains to determine the insertion index.
