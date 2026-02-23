fn main() {
    println!("Hello, world!");
}

struct Solution {}
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        // NOTE: Use i32 instead of usize to support negative indices/values.
        let mut idx: i32 = (digits.len() - 1) as i32;

        while idx >= 0 {
            if digits[idx as usize] + 1 == 10 {
                digits[idx as usize] = 0;
                idx -= 1;
            } else {
                digits[idx as usize] += 1;
                return digits;
            }
        }

        digits.insert(0, 1);
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one_case_1() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4])
    }

    #[test]
    fn test_plus_one_case_2() {
        assert_eq!(Solution::plus_one(vec![1, 2, 9]), vec![1, 3, 0])
    }

    #[test]
    fn test_plus_one_case_3() {
        assert_eq!(Solution::plus_one(vec![1, 9, 9]), vec![2, 0, 0])
    }

    #[test]
    fn test_plus_one_case_4() {
        assert_eq!(Solution::plus_one(vec![9, 9]), vec![1, 0, 0])
    }
}

// Plus One at the represented as an integer array
//
// 1. Iterate backwards throught the array
// 2. if digits[current_index] + 1 is less than 10:
//    increment the current digit and return the array
// 3. if digits[current_index] + 1 equals 10:
//    set the current digit to 0 and continue to the next iteration.
// 4. if the loop finishes without returing, prepend 1 to the array and return it.

// AI code review
pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
    // Itereate backwards using `.rev()`.
    for digit in digits.iter_mut().rev() {
        if *digit < 9 {
            *digit += 1;
            return digits;
        }

        *digit = 0;
    }

    digits.insert(0, 1);
    digits
}
