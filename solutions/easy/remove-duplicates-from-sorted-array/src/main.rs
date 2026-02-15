fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut latest_new_num: i32 = nums[0];
        let mut latest_new_num_index = 0;

        for i in 1..nums.len() {
            if latest_new_num == nums[i] {
                continue;
            }

            nums[latest_new_num_index + 1] = nums[i];
            latest_new_num = nums[i];
            latest_new_num_index += 1;
        }

        (latest_new_num_index + 1) as i32
    }
}

// Modifying the array in-place to "remove" duplicates
// 1. Track the value of the last unique element ound.
// 2. Track the index where the next unique element should be placed.
// 3. Iterate through the nums array:
//    - if the current num mathces latest_new_num: it's a duplicate, so skip it.
//    - if they don't match: Update the element at the next index in nums,
//      then update latet_new_num and increment latest_new_num_index.
// 4. Finally, return laest_new_num_index + 1 as the count of unique elements.
