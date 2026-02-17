fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0 as i32;
        }

        let mut writer = 0;

        for reader in 0..nums.len() {
            if nums[reader] != val {
                nums[writer] = nums[reader];
                writer += 1;
            }
        }

        writer as i32
    }
}

// Modifying the array in-place to "remove" the target value
// 1. Track the writer index
// 2. Iterate through the nums array:
//    - if the nums[reader] is not equal val
//       | This is a value we want to keep
//       | so update the element at the writer index
//       | nums[writer] = nums[reader]
//       | and increment the `writer`
// 3. Finally, return `writer` as the new length of the modified array.
