fn main() {
    println!("Hello, world!");
}

struct Solution {}
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 1 {
            return 1;
        }
        let mut left = 0;
        let mut right = x;
        let mut ans = 0;

        while left <= right {
            let mid = left + (right - left) / 2;
            if mid == 0 {
                return 0;
            }

            if mid <= x / mid {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        ans
    }
}

// Goal
// Calculate the integer square root of x without using built-in exponent functions.
//
// 1. Initialize left to 1 and right to x
// 2. Loop while left is less than or equal to right.
// 3. Calculate the midpoint: mid = left + (right - left)/2
// 4. Compare
//    - If mid <= x / mid:
//      - Store mid as the provisional answer.
//      - Update left = mid + 1 to search the upper half.
//    - Else:
//      - Update right = mid - 1 to search the lower half.
// 5. Return the provisional answer as the final result once the loop terminates.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_sqrt_case1() {
        assert_eq!(Solution::my_sqrt(4), 2);
    }

    #[test]
    fn test_my_sqrt_case2() {
        assert_eq!(Solution::my_sqrt(8), 2);
    }

    #[test]
    fn test_my_sqrt_case3() {
        assert_eq!(Solution::my_sqrt(2), 1);
    }

    #[test]
    fn test_my_sqrt_case4() {
        assert_eq!(Solution::my_sqrt(1), 1);
    }

    #[test]
    fn test_my_sqrt_case5() {
        assert_eq!(Solution::my_sqrt(2147395599), 46339);
    }
}
